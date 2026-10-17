import sys,re
M=sys.argv[1]
base='crate::standards::v1::subsets::any::schema::mutations::'
leaves=[('🟢️node/🌱create-node','preparation','CreateNode','create_node'),
        ('🟢️node/🗑️delete-node','preparation','DeleteNode','delete_node'),
        ('🧭️pose/📍move-node','preparation','MoveNode','move_node'),
        ('🔌️handle/🪢️connect-handles','prepare','ConnectHandles','connect_handles')]
agg=open(M+'/🦀️.rs',encoding='utf8').read()
mounts=''
for d,mod,ty,leaf in leaves:
    lp=M+'/'+d+'/🦀️.rs'; t=open(lp,encoding='utf8').read()
    mount='#[path = "🎮️prepare/🦀️.rs"]\npub mod %s;\n'%mod
    assert mount in t,(d,)
    t=t.replace(mount,'',1); open(lp,'w',encoding='utf8').write(t)
    pp=M+'/'+d+'/🎮️prepare/🦀️.rs'; p=open(pp,encoding='utf8').read()
    old='use super::%s;'%ty; assert old in p,(d,)
    p=p.replace(old,'use %s%s::%s;'%(base,leaf,ty),1); open(pp,'w',encoding='utf8').write(p)
    mounts+='#[path = "%s/🎮️prepare/🦀️.rs"]\npub mod %s_preparation;\n\n'%(d,leaf)
anchor='#[path = "🚩️flag/🎮️prepare/🦀️.rs"]\npub mod flag_preparation;\n'
assert anchor in agg
agg=agg.replace(anchor,mounts+anchor,1)
open(M+'/🦀️.rs','w',encoding='utf8').write(agg)
c=M+'/🎮️prepare/🧰️child/🦀️.rs'; t=open(c,encoding='utf8').read()
for a,b in (('mutations::connect_handles::prepare::','mutations::connect_handles_preparation::'),('mutations::delete_node::preparation::','mutations::delete_node_preparation::')):
    t=t.replace(a,b)
open(c,'w',encoding='utf8').write(t)
print('ok')
