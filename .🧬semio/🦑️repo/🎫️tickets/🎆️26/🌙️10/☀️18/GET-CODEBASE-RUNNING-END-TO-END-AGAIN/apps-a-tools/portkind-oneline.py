#!/usr/bin/env python3
"""Splits one-line-fn `impl MutationKind` blocks (fns at column 0) into MutationKind + ColdMutationKind. Idempotent."""
import re,sys
n=0
for f in sys.argv[1:]:
    s=open(f,encoding='utf-8').read()
    if 'ColdMutationKind<' in s: continue
    m=re.search(r'^impl<Op> MutationKind<([^>]+)> for (\w+)\nwhere\n    Op: protocol::Mutation<([^,]+), Diff = (\w+)>([^\n]*),\n\{\n',s,re.M)
    if not m: print('skip',f[-60:]); continue
    snap,leaf,_,diff,rest=m.groups()
    end=s.index('\n}\n',m.end())
    body=s[m.end():end+1]
    lines=body.split('\n')
    cold=[l for l in lines if re.match(r'fn (diff|inverse)\b',l)]
    keep=[l for l in lines if not re.match(r'fn (diff|inverse)\b',l)]
    if len(cold)!=2: print('shape',f[-60:]); continue
    new=s[:m.end()]+'\n'.join(keep)
    new=new.rstrip('\n')+'\n}\n\n'
    new+=f'impl<Op> protocol::ColdMutationKind<{snap}> for {leaf}\nwhere\n    Op: protocol::ColdMutation<{snap.split(",")[0]}, ReferenceDiff = {diff}>{rest},\n{{\n'+'\n'.join(cold)+'\n}\n'
    new=new.replace(f'ColdMutationKind<{snap}>',f'ColdMutationKind<{snap}>')+s[end+3:]
    open(f,'w',encoding='utf-8').write(new);n+=1
print('ported',n)
