"""🧪 Reach and chain depth of the staged carve set (dead edges, ordered wire, derive resources) against the current graph. Run with `cwd` = the generated folder."""
import sys

from sim3 import *
CR=byname['semio-framework-native-build-compiler-resources']
PRINT=byname['semio-framework-print']
def staged(E):
    E=S1(E)
    E=clone(E)
    E[N['intrinsic']].discard(N['ordered']);E[N['ordered']].add(N['intrinsic'])
    E[N['derive']].discard(CR)
    E['NEW:resources']={CR}
    E[PRINT].add('NEW:resources')
    return E
Es=staged(base)
keys={'registry':N['registry'],'schema':N['schema'],'edit':N['edit'],'ordered':N['ordered'],'intrinsic':N['intrinsic'],'derive':N['derive'],'derive-expansion':N['derive-expansion'],'compiler-resources':CR,'hash':HASH,'schema-registry':SREG,'facade':FACADE,'persistent':N['persistent'],'original-aliases':N['original-aliases'],'native-encoding':N['native-encoding'],'native-decoding':N['native-decoding'],'paged-list':N['paged-list'],'control-contract':N['control-contract'],'NEW:resources':'NEW:resources'}
print('%-20s %7s %7s'%('node','now','staged'))
for k,v in keys.items():
    a=tr(base,[v])[v] if v in base or any(v in s for s in base.values()) else 0
    b=tr(Es,[v])[v]
    print('%-20s %7d %7d'%(k,a,b))
h0=chain(base);h1=chain(Es)
print('chain from control-contract',h0(N['control-contract']),h1(N['control-contract']))
print('chain from intrinsic',h0(N['intrinsic']),h1(N['intrinsic']))
print('chain from ordered',h0(N['ordered']),h1(N['ordered']))
print('mean reach all',round(allreach(base),2),round(allreach(Es),2))
