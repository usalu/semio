"""🦴 Validates structural tissue assignments, preserved skin and accessory references in all human documents."""
import json
from pathlib import Path
from collections import Counter
t=Path(__file__).resolve().parent
assets=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
counts=Counter()
for p in sorted(assets.glob('*/*')):
    a=json.loads(p.read_text())['asset']
    mats={m['id'] for m in a['materials']}
    assert 'bone' in mats,(p,'no bone')
    for v in a['volumes'].values():
        for key in ('body','structure','thermal'):
            for q in v[key]:
                if 'material' in q: assert q['material'] in mats,(p,key,q['id'],q['material'])
        for q in v['structure']:
            if q.get('material') in ('skin','skin-child'):
                assert q['part']=='mass' or q['id'] in ['joint-grip-l','joint-grip-r'],(p,q['id'])
                assert q['id'] not in ['mass-pack','mass-box']
            if q.get('material')=='bone':counts['structural bone']+=1
        counts['outer skin']+=sum(q.get('material') in ('skin','skin-child') for q in v['body'])
        counts['thermal skin']+=sum(q.get('material') in ('skin','skin-child') for q in v['thermal'])
        counts['volumes']+=1
    counts['figures']+=1
print(dict(counts))
