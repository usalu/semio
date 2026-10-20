"""Go canonical-layout overlay plan (ported from gocomp/overlay.py): GOPLAN_TAXONOMY names the taxonomy.json whose member names decide which emoji directories are Go sources."""
#!/usr/bin/env python3
import hashlib,json,os,re,sys
import os as _os
TAX=_os.environ['GOPLAN_TAXONOMY']
t=json.load(open(TAX))
SRC=set(t['semanticDirectoryMemberKinds']['members-of-members-of-members-of-modules']['memberNames'])
TESTS=t['testsDirName']; PKGS=t['packagesDirName']; FIX=t['testFixturesDirName']
IMPL='🐹️.go'
def module_plan(root):
    root=os.path.realpath(root)
    src={}; tst={}
    def mp(owner,source,test):
        rel=os.path.relpath(source,root).replace(os.sep,'/')
        i=hashlib.sha256(rel.encode()).hexdigest()[:16]
        v=os.path.join(owner,f"zz_semio_{i}{'_test' if test else ''}.go")
        assert not os.path.exists(v),v
        (tst if test else src)[v]=source
    def walk(d):
        if d!=root and os.path.exists(os.path.join(d,'go.mod')): return
        for name in sorted(os.listdir(d)):
            p=os.path.join(d,name)
            if not os.path.isdir(p) or os.path.islink(p): continue
            if os.path.exists(os.path.join(p,'go.mod')): continue
            if name in SRC:
                s=os.path.join(p,IMPL)
                if os.path.isfile(s) and not os.path.islink(s): mp(d,s,False)
                continue
            if name==TESTS:
                for c in sorted(os.listdir(p)):
                    cp=os.path.join(p,c)
                    if not os.path.isdir(cp) or os.path.islink(cp): continue
                    s=os.path.join(cp,IMPL)
                    if os.path.isfile(s) and not os.path.islink(s): mp(d,s,True)
                continue
            if name in ('.git','node_modules','target','vendor',FIX,PKGS): continue
            walk(p)
    walk(root)
    return src,tst
def replace_roots(root):
    gm=os.path.join(root,'go.mod')
    if not os.path.exists(gm): return []
    out=[]; block=False
    for raw in open(gm).read().splitlines():
        line=re.sub(r'//.*$','',raw).strip()
        if not line: continue
        if re.match(r'^replace\s*\($',line): block=True; continue
        if block and line==')': block=False; continue
        d=line
        if re.match(r'^replace\s+',line): d=re.sub(r'^replace\s+','',line)
        elif not block: continue
        parts=d.split()
        if '=>' not in parts: continue
        rep=parts[parts.index('=>')+1]
        if not (os.path.isabs(rep) or rep.startswith('.')): continue
        c=os.path.realpath(os.path.join(root,rep))
        if os.path.exists(os.path.join(c,'go.mod')): out.append(c)
    return sorted(set(out))
def plan(root):
    root=os.path.realpath(root)
    src,tst=module_plan(root)
    seen={root}
    def add(o):
        for dep in replace_roots(o):
            if dep in seen: continue
            seen.add(dep)
            s,_=module_plan(dep)
            for k,v in s.items():
                assert k not in src or src[k]==v,k
                src[k]=v
            add(dep)
    add(root)
    return src,tst
if __name__=='__main__':
    root,out=sys.argv[1],sys.argv[2]
    s,tt=plan(root)
    if '--workspace' in sys.argv:
        wk=sys.argv[sys.argv.index('--workspace')+1]; base=os.path.dirname(os.path.abspath(wk))
        for l in open(wk,encoding='utf-8'):
            l=l.strip()
            if l.startswith('./'):
                ms,_=module_plan(os.path.join(base,l[2:]))
                for k,v in ms.items(): s.setdefault(k,v)
    if '--extra-tests' in sys.argv:
        ex=sys.argv[sys.argv.index('--extra-tests')+1]
        only=sys.argv[sys.argv.index('--only')+1] if '--only' in sys.argv else None
        for c in sorted(os.listdir(ex)):
            if only and c!=only: continue
            f=os.path.join(ex,c,IMPL)
            import re as _re
            def pk(path):
                m=_re.search(r'^package\s+(\w+)',open(path,encoding='utf-8').read(),_re.M); return m.group(1) if m else None
            pkg=pk(next(iter(s.values())))
            if os.path.isfile(f) and pk(f)==pkg:
                i=hashlib.sha256(os.path.relpath(f,os.path.realpath(root)).encode()).hexdigest()[:16]
                tt[os.path.join(os.path.realpath(root),f"zz_semio_extra_{i}_test.go")]=f
    rep=dict(s)
    if '--tests' in sys.argv: rep.update(tt)
    json.dump({'Replace':rep},open(out,'w'),ensure_ascii=False,indent=1)
    print('sources',len(s),'tests',len(tt),'written',len(rep))
