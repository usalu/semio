"""🔎 Checks the gardener's independent digital scales, binding, load paths and thermal region coverage."""
from pathlib import Path
import json
p=next(Path.cwd().glob("🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/kneeling-gardener/*"))
a=json.loads(p.read_text())['asset']
for denominator in ('200','500'):
    v=a['volumes'][denominator]
    assert v['status']=='authored',f'missing gardener at 1:{denominator}'
    assert a['representations'][denominator]['status']=='unavailable'
    bones={bone['id'] for bone in a['rig']['bones']} - {'hip-lift','hip-shift'}
    assert bones=={part['bone'] for part in v['body']}
    assert bones=={part['member'] for part in v['structure'] if part['part']=='member'}
    assert {region['id'] for region in a['thermal']['regions']}=={part['region'] for part in v['thermal']}
    assert all(any(em['id'] in p.get('tags',[]) for p in v['thermal']) for em in a['thermal']['emitters'])
    ids={p['id'] for p in v['body']}
    assert all('of' not in p or p['of'] in ids for p in v['thermal']+v['structure'])
    assert all(len(v[k])<len(a['volumes']['100' if denominator=='200' else '200'][k]) for k in ('body','thermal','structure'))
print('gardener digital scales, heat regions and skeletal binding passed')
