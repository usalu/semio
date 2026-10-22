#!/usr/bin/env python3
"""Splits `impl MutationKind<P, Op> for Leaf` into MutationKind (label/target/semantics) + ColdMutationKind (diff/inverse/foreign_step_source). Idempotent."""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rsparse import match_brace
from portdiff import items

HDR=re.compile(r'^impl(<[^\n]*?>)?\s+((?:\w+::)*)MutationKind<[^\n]*?>\s+for\s+[^{]*?\{',re.M|re.S)

def cold_header(h):
    h=re.sub(r'\b(?:protocol::)?MutationKind<',lambda m:'protocol::ColdMutationKind<',h,count=1)
    h=re.sub(r'\b(?:protocol::)?Mutation<([^{}]*?),\s*Diff\s*=',r'protocol::ColdMutation<\1, ReferenceDiff =',h)
    h=re.sub(r'(?<![\w:])(?:protocol::)?Mutation<(?![^{}]*ReferenceDiff)',lambda m:'protocol::ColdMutation<' ,h) if False else h
    return h

def body_fix(t):
    t=re.sub(r'<(\w+) as (?:protocol::)?Mutation<([^<>]+(?:<[^<>]*>)?)>>::Diff',r'<\1 as protocol::ColdMutation<\2>>::ReferenceDiff',t)
    return t

def port(s):
    out=[];pos=0;ch=False
    for m in HDR.finditer(s):
        if m.start()<pos: continue
        ob=m.end()-1;eb=match_brace(s,ob);body=s[ob+1:eb]
        its=items(body);names=[n for n,_,_ in its]
        if 'diff' not in names and 'inverse' not in names: continue
        keep=body;cold=[]
        for n,a,b in sorted(its,key=lambda x:-x[1]):
            if n in('diff','inverse','foreign_step_source'):
                cold.insert(0,body[a:b]); keep=keep[:a]+keep[b:]
        keep=re.sub(r'\n[ \t]*\n([ \t]*\n)+','\n\n',keep)
        head=s[m.start():ob]
        newhead=cold_header(head)
        new=head+'{'+keep.rstrip()+"\n}\n\n"+newhead+'{\n'+body_fix('\n'.join(cold))+"\n}"
        out.append(s[pos:m.start()]);out.append(new);pos=eb+1;ch=True
    out.append(s[pos:])
    return ''.join(out),ch

def main():
    dry='--dry' in sys.argv
    files=[a for a in sys.argv[1:] if not a.startswith('--')]
    for a in list(files):
        if a.startswith('@'):
            files.remove(a); files+=[l for l in open(a[1:],encoding='utf-8').read().split('\n') if l]
    n=0
    for f in files:
        s=open(f,encoding='utf-8').read()
        t,ch=port(s)
        if ch:
            n+=1
            if not dry: open(f,'w',encoding='utf-8').write(t)
    print('ported',n,'of',len(files))
if __name__=='__main__': main()
