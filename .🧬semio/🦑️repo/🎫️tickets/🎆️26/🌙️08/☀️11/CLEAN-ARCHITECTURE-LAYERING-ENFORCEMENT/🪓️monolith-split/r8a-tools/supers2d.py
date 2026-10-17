import sys,os,re
M=sys.argv[1]
MUT='crate::standards::v1::subsets::any::schema::mutations::'
SNAP='crate::standards::v1::subsets::any::schema::snapshot::'
leafs={'CreateNode','DeleteNode','MoveNode','ConnectHandles','ChangeNodeIcon','EditTargetRegionLabel','Puzzle2dMutation','ChangeNodeLocked','ChangeNodeVisible','ChangeEdgeLocked','ChangeEdgeVisible','ChangeNodeRoot','ChangeTargetRegionHidden','ChangeTargetRegionLocked'}
dirs=['🟢️node/🌱create-node/🎮️prepare','🟢️node/🗑️delete-node/🎮️prepare','🧭️pose/📍move-node/🎮️prepare','🔌️handle/🪢️connect-handles/🎮️prepare','🚩️flag/🎮️prepare','🔤️text/🎮️prepare','🔤️text/↩️inverse']
pat=re.compile(r'^(\s*)use (super::super|super)::(\{[^;]*\}|[A-Za-z_0-9:]+);',re.M)
n=0
def repl(m):
    ind,sup,body=m.group(1),m.group(2),m.group(3)
    names=[x.strip() for x in body.strip('{}').split(',')] if body.startswith('{') else [body]
    if any('::' in x for x in names): return m.group(0)
    lf=[x for x in names if x in leafs]; sn=[x for x in names if x=='Puzzle2dSnapshot']; rest=[x for x in names if x not in leafs and x!='Puzzle2dSnapshot']
    if not lf and not sn: return m.group(0)
    out=[]
    if rest: out.append('%suse %s::{%s};'%(ind,sup,', '.join(rest)))
    if lf: out.append('%suse %s{%s};'%(ind,MUT,', '.join(lf)))
    if sn: out.append('%suse %sPuzzle2dSnapshot;'%(ind,SNAP))
    return '\n'.join(out)
for d in dirs:
    for root,_,fs in os.walk(M+'/'+d):
        for f in fs:
            if f!='🦀️.rs': continue
            p=os.path.join(root,f); t=open(p,encoding='utf8').read()
            u=pat.sub(repl,t)
            if u!=t: open(p,'w',encoding='utf8').write(u); n+=1
print('files',n)
