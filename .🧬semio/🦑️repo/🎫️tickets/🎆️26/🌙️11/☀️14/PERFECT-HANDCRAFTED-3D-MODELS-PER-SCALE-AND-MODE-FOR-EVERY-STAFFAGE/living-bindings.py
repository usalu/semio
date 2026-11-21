"""🦴 Verify bone hierarchies, shape references and fields in the 360 living catalog scenes."""
import hashlib
import json
import math
from pathlib import Path

root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
for path in sorted([*root.glob('*animals/*/*.json'),*root.glob('*vegetation/*/*.json')]):
    raw=path.read_bytes()
    asset=json.loads(raw)['asset']
    name=path.parent.name
    bones={b['id']:b['parent'] for b in asset['rig']['bones']}
    assert len(bones)==len(asset['rig']['bones']), name
    for bone in bones:
        trail=set()
        parent=bone
        while parent is not None:
            assert parent in bones and parent not in trail,(name,bone,parent)
            trail.add(parent)
            parent=bones[parent]
    assert set(asset['volumes'])=={'50','100','200','500'},name
    total=0
    regions={q['id'] for q in asset['thermal']['regions']}
    for scale,model in asset['volumes'].items():
        assert model['status']=='authored',(name,scale)
        body={s['id'] for s in model['body']}
        for mode in ('body','thermal','structure'):
            items=model[mode]
            assert items and len({s['id'] for s in items})==len(items),(name,scale,mode)
            for s in items:
                key=(name,scale,mode,s['id'])
                if 'of' in s:
                    assert s['of'] in body,key
                else:
                    assert s['bone'] in bones,key
                if s.get('region'):
                    assert s['region'] in regions,key
                for section in s.get('sections',[]):
                    assert min(section['rx'],section['ry'])>0,key
                    assert all(math.isfinite(n) for n in section['at']),key
                    assert abs(math.hypot(*section.get('rotation',[0,0,0,1]))-1)<0.002,key
                if 'radii' in s:
                    assert all(math.isfinite(n) and n>0 for n in s['radii']),key
                total+=1
    print('[DEBUG]',name,len(bones),total,hashlib.sha256(raw).hexdigest()[:12])

