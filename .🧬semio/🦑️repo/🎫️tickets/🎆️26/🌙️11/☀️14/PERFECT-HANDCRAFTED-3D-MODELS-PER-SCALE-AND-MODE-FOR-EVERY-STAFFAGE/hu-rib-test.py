"""🩻 Verifies the paired costal arcs in three pose-specific rib cages."""
import json
from pathlib import Path
from math import dist
for name in ('cyclist','child-on-kick-scooter','skateboarder'):
    path=next(Path.cwd().glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+name+'/*'))
    volumes=json.loads(path.read_text())['asset']['volumes']
    for scale,indexes in [('50',range(1,13)),('100',(1,3,5,7,9,11))]:
        solids={p['id']:p for p in volumes[scale]['structure']}
        assert 'ribcage' not in solids,name+' has a solid thoracic block'
        for i in indexes:
            l,r=[solids[f'costal-{i}-{side}']['sections'] for side in ('l','r')]
            assert all(p['at'][:2]==q['at'][:2] and p['at'][2]==-q['at'][2] for p,q in zip(l,r))
            assert len(l)>=5 and max(p['at'][2] for p in l)>.03
            assert l[-1]['at'][2]<.012 if i<=7 else l[-1]['at'][2]>=.012
print('paired costal paths and open lower ribs verified')
