import sys,re,os
D,M=sys.argv[1:3]
r=open(D+'/🦀️.rs',encoding='utf8').read()
names=['literal_id_inverse','create_node_preparation','delete_node_preparation','move_node_preparation','connect_handles_preparation','flag_preparation','text_preparation','text_inverse','native_preparation_child']
blocks=[]
for n in names:
    m=re.search(r'( *)#\[path = "([^"]+)"\]\n *(pub(?:\(crate\))? mod '+n+r';)\n',r)
    assert m,n
    blocks.append((m.group(2),m.group(3))); r=r.replace(m.group(0),'',1)
anchor='                    #[path = "."]\n                    pub mod mutations {'
assert anchor in r
cont='                    #[path = "."]\n                    pub mod preparation {\n'+''.join('                        #[path = "%s"]\n                        %s\n'%(p,d) for p,d in blocks)+'                    }\n'
r=r.replace(anchor,cont+anchor,1)
open(D+'/🦀️.rs','w',encoding='utf8').write(r)
ABS='crate::standards::v1::subsets::any::schema::'
MUT=ABS+'mutations::'; PRE=ABS+'preparation::'
for root,ds,fs in os.walk(D):
    ds[:]=[x for x in ds if x not in('node_modules','target','🤖️generated')]
    for f in fs:
        if f!='🦀️.rs': continue
        p=os.path.join(root,f); t=open(p,encoding='utf8').read(); o=t
        for n in names: t=t.replace(MUT+n,PRE+n)
        if t!=o: open(p,'w',encoding='utf8').write(t)
def sub(path,old,new):
    t=open(path,encoding='utf8').read(); assert old in t,(path,old); open(path,'w',encoding='utf8').write(t.replace(old,new,1))
sub(M+'/🚩️flag/🎮️prepare/🦀️.rs','use super::{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible,ChangeNodeRoot,ChangeTargetRegionHidden,ChangeTargetRegionLocked,Puzzle2dMutation};','use '+MUT+'{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible,ChangeNodeRoot,ChangeTargetRegionHidden,ChangeTargetRegionLocked,Puzzle2dMutation};')
sub(M+'/🔤️text/🎮️prepare/🦀️.rs','use super::{ChangeNodeIcon,EditTargetRegionLabel};','use '+MUT+'{ChangeNodeIcon,EditTargetRegionLabel};')
ti=M+'/🔤️text/↩️inverse/🦀️.rs'; t=open(ti,encoding='utf8').read()
m=re.search(r'use super::\{ChangeNodeIcon,EditTargetRegionLabel,Puzzle2dMutation,text_preparation::\{([^}]*)\}\};',t); assert m
t=t.replace(m.group(0),'use '+MUT+'{ChangeNodeIcon,EditTargetRegionLabel,Puzzle2dMutation};\nuse '+PRE+'text_preparation::{'+m.group(1)+'};',1)
open(ti,'w',encoding='utf8').write(t)
sub(M+'/🏷️literal-id-inverse/🦀️.rs','use super::Puzzle2dMutation;','use '+MUT+'Puzzle2dMutation;')
print('ok')
