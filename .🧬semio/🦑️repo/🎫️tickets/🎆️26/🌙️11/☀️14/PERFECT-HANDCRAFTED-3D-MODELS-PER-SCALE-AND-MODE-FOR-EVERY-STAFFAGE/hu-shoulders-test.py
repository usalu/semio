"""💪 Verifies the chest-to-humerus load path reaches both skater shoulder joints."""
from pathlib import Path
import json,math
p=next(Path.cwd().glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/skateboarder/*'))
a=json.loads(p.read_text())['asset']
for n,v in a['volumes'].items():
    bridge=[s for s in v['structure'] if s['id'].startswith(('clavicle-','scapula-','shoulder-girdle'))]
    for side in ('l','r'):
        b=next(b for b in a['rig']['bones'] if b['id']=='upper-arm-'+side)
        sidepoint=(-b['y'],b['x']-.42,b['z'])
        assert any(math.dist(c['at'],sidepoint)<=max(c['rx'],c['ry']) for s in bridge for c in s.get('sections',[])),n+' disconnected shoulder '+side
