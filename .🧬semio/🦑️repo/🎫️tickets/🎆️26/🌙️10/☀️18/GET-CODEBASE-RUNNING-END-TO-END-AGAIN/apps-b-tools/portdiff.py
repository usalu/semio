#!/usr/bin/env python3
"""Ports pre-split plugin diffs/mutations to the cold-split protocol (MutationDiff/ColdMutationDiff, DiffAlgebra/ColdDiffAlgebra, Mutation/ColdMutation). Idempotent."""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rsparse import match_brace

def items(body):
    """split impl body text into [(name, text)] top-level fns (with leading docs/attrs) and other items."""
    out=[];i=0;n=len(body)
    lines=body.split('\n')
    pos=0;cur_start=0
    # find fn starts at indent 4
    starts=[]
    for m in re.finditer(r'^    (?:pub )?(?:async )?fn (\w+)[^\n]*',body,re.M):
        starts.append((m.start(),m.group(1)))
    res=[]
    for idx,(st,name) in enumerate(starts):
        # include preceding doc/attr lines
        j=st
        while True:
            k=body.rfind('\n',0,j-1) if j>0 else -1
            prev=body[k+1:j-1] if j>0 else ''
            if j>0 and (prev.lstrip().startswith('///') or prev.lstrip().startswith('#[')) and prev.startswith('    '):
                j=k+1
            else: break
        ob=body.find('{',st)
        # signature may contain braces? assume not (no const generics blocks)
        e=match_brace(body,ob)
        res.append((name,j,e+1))
    return res

def port_diffs(s):
    changed=False
    out=[];pos=0
    hdr=re.compile(r'^impl(<[^{\n]*>)?\s+((?:[\w]+::)*)(MutationDiff|DiffAlgebra)<([^{\n]*?)>\s+for\s+([^{\n]*?)\s*\{',re.M)
    for m in hdr.finditer(s):
        if m.start()<pos: continue
        ob=m.end()-1; eb=match_brace(s,ob)
        body=s[ob+1:eb]
        its=items(body)
        names=[n for n,_,_ in its]
        kind=m.group(3)
        generics=m.group(1) or ''
        S_=m.group(4); Y=m.group(5)
        if kind=='MutationDiff' and 'apply' in names and 'absorb' in names:
            keep=[];cold=[]
            for n,a,b in its:
                txt=body[a:b]
                if n=='apply': ap=txt
                elif n=='absorb': cold.append(txt)
                else: keep.append(txt)
            ap2=port_apply(ap)
            if ap2 is None:
                print('  !! apply not portable',m.group(0)[:80]); continue
            new=f"impl{generics} {m.group(2)}MutationDiff<{S_}> for {Y} "+("{\n"+'\n'.join(keep)+"\n}" if keep else "{}")+"\n\n"
            new+=f"impl{generics} protocol::ColdMutationDiff<{S_}> for {Y} {{\n"+'\n'.join(cold)+'\n\n'+ap2+"\n}"
        elif kind=='DiffAlgebra' and 'inverse' in names and 'is_empty' in names:
            keep=[];inv=None
            for n,a,b in its:
                txt=body[a:b]
                if n=='inverse': inv=txt
                else: keep.append(txt)
            new=f"impl{generics} {m.group(2)}DiffAlgebra<{S_}> for {Y} {{\n"+'\n'.join(keep)+"\n}\n\n"
            new+=f"impl{generics} protocol::ColdDiffAlgebra<{S_}> for {Y} {{\n"+inv+"\n}"
        else:
            continue
        out.append(s[pos:m.start()]);out.append(new);pos=eb+1;changed=True
    out.append(s[pos:])
    return ''.join(out),changed

def port_apply(txt):
    m=re.search(r'fn apply\(&self,\s*(\w+):\s*&([^,]+?),\s*(\w+):\s*([^)]+?)\)\s*->\s*([\w:]*MutationApplyResult)<([^{]+?)>\s*\{',txt)
    if not m: return None
    basename,S_,capname,captype,res,_=m.groups()
    o=m.end()-1;e=match_brace(txt,o)
    body=txt[o+1:e]
    head=txt[:m.start()]
    b=body.strip('\n')
    nr=re.sub(r'"(?:[^"\\]|\\.)*"','""',body)
    stripped=body.rstrip()
    closure=bool(re.search(r'\breturn\b',nr))
    # outermost trailing Ok( ... )
    cand=None
    for mm in re.finditer(r'\bOk\(',stripped):
        p=mm.end()-1
        if match_brace(stripped,p)==len(stripped)-1:
            cand=mm;break
    if cand is None: closure=True
    if closure:
        sig=f"fn apply(&self, unpublished: &mut {S_}, {capname}: {captype}) -> {res}<()> {{"
        inner=body.strip('\n')
        return head+sig+f"\n        let applied = (|{basename}: &{S_}| -> {res}<{S_}> {{\n"+inner+f"\n        }})(&*unpublished)?;\n        *unpublished = applied;\n        Ok(())\n    }}"
    expr=stripped[cand.end():-1].strip()
    prefix=stripped[:cand.start()].rstrip()
    use_base=re.search(r'\b'+re.escape(basename)+r'\b',prefix+expr) is not None
    lines=[]
    if use_base: lines.append(f"        let {basename} = &*unpublished;")
    if prefix.strip(): lines.append(re.sub(r'^\n+','',prefix) if False else prefix.strip('\n'))
    if expr==f"{basename}.clone()":
        pass
    else:
        lines.append(f"        *unpublished = {expr};")
    lines.append("        Ok(())")
    sig=f"fn apply(&self, unpublished: &mut {S_}, {capname}: {captype}) -> {res}<()> {{"
    # dedent prefix if it already has its indentation
    return head+sig+"\n"+'\n'.join(lines)+"\n    }"

def port_mutations(s):
    changed=False;out=[];pos=0
    hdr=re.compile(r'^impl(<[^{\n]*>)?\s+((?:[\w]+::)*)Mutation<([^{\n]*?)>\s+for\s+([^{\n]*?)\s*\{',re.M)
    for m in hdr.finditer(s):
        if m.start()<pos: continue
        ob=m.end()-1; eb=match_brace(s,ob); body=s[ob+1:eb]
        its=items(body); names=[n for n,_,_ in its]
        if 'diff' not in names or 'inverse' not in names: continue
        dm=re.search(r'^    type Diff\s*=\s*([^;]+);',body,re.M)
        if not dm: print('  !! no type Diff');continue
        keep=body;cold=[]
        for n,a,b in sorted(its,key=lambda x:-x[1]):
            if n in('diff','inverse','foreign_step_source'):
                cold.insert(0,body[a:b]); keep=keep[:a]+keep[b:]
        # collapse blank runs
        keep=re.sub(r'\n{3,}','\n\n',keep)
        pre=m.group(2)
        new=s[m.start():ob+1]+keep.rstrip()+"\n}\n\n"
        new+=f"impl{m.group(1) or ''} {pre}ColdMutation<{m.group(3)}> for {m.group(4)} {{\n    type ReferenceDiff = {dm.group(1).strip()};\n"+'\n'.join(cold)+"\n}"
        new=new.replace(f"impl{m.group(1) or ''} {pre}ColdMutation<",f"impl{m.group(1) or ''} protocol::ColdMutation<") if not pre else new
        out.append(s[pos:m.start()]);out.append(new);pos=eb+1;changed=True
    out.append(s[pos:])
    return ''.join(out),changed

def main():
    dry='--dry' in sys.argv
    files=[a for a in sys.argv[1:] if not a.startswith('--')]
    for a in list(files):
        if a.startswith('@'):
            files.remove(a); files+= [l for l in open(a[1:],encoding='utf-8').read().split('\n') if l]
    for f in files:
        s=open(f,encoding='utf-8').read()
        t=s;ch=False
        if 'ColdMutationDiff<' not in s and 'MutationDiff<' in s:
            t,c1=port_diffs(t);ch|=c1
        if 'ColdMutation<' not in t and re.search(r'^impl[^\n]*\bMutation<',t,re.M):
            t,c2=port_mutations(t);ch|=c2
        if ch:
            print('port',f[-70:])
            if not dry: open(f,'w',encoding='utf-8').write(t)
if __name__=='__main__': main()
