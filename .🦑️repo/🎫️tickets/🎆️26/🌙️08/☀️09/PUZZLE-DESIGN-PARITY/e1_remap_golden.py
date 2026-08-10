#!/usr/bin/env python3
import json
from pathlib import Path
ROOT = Path('/Users/ueli/Documents/semio')
KIT = ROOT / 'compose/fixture/kit/dev/metabolism/wip/initialKit'
OUT = Path(__file__).resolve().parent / '🌙️capsule-dream-out'
dream = json.loads((KIT/'design/capsule-dream.design.compose.json').read_text())
flat = json.loads((KIT/'design/flat.design.compose.json').read_text())
dream_id_by_name = {p.get('name'): p['id'] for p in dream['pieces']['items']}
golden = {}
missing = 0
for p in flat['pieces']['items']:
    dream_id = dream_id_by_name.get(p.get('name'))
    if not dream_id:
        missing += 1
        continue
    pose = p.get('pose') or {}
    plane = pose.get('plane') or {}
    origin = plane.get('origin') or {}
    xa = plane.get('xAxis') or {}
    ya = plane.get('yAxis') or {}
    center = pose.get('center') or {}
    golden[dream_id] = {
        'origin': [origin.get('x',0), origin.get('y',0), origin.get('z',0)],
        'xAxis': [xa.get('x',1), xa.get('y',0), xa.get('z',0)],
        'yAxis': [ya.get('x',0), ya.get('y',1), ya.get('z',0)],
        'center': {'x': center.get('u',0), 'y': center.get('v',0)},
    }
OUT.mkdir(parents=True, exist_ok=True)
(OUT/'🏅golden-poses.json').write_text(json.dumps(golden, indent=2))
print('golden', len(golden), 'missing', missing)
print('has base', '739c5e32-217f-4d03-b7b6-78b9c0cda46a' in golden)
