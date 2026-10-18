#!/usr/bin/env python3
import hashlib,json,os,re,sys
TAX='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json'
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
    rep=dict(s)
    if '--tests' in sys.argv: rep.update(tt)
    json.dump({'Replace':rep},open(out,'w'),ensure_ascii=False,indent=1)
    print('sources',len(s),'tests',len(tt),'written',len(rep))
