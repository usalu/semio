"""🦶 Requires the coarse runner's foot load path to reach the ankle pivot."""
from pathlib import Path
import json
from math import dist
p=next(Path.cwd().glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/running-child/*'))
for s in json.loads(p.read_text())['asset']['volumes']['500']['structure']:
    if s['id'] not in ('sk-foot-l','sk-foot-r'):continue
    path=[r['at'] for r in s['sections']] if s['kind']=='loft' else [s['p0'],s['p1']]
    limit=max(r['rx'] for r in s['sections']) if s['kind']=='loft' else s['r0']
    assert min(dist([0,0,0],p) for p in path)<=limit,s['id']+' bypasses the ankle'
