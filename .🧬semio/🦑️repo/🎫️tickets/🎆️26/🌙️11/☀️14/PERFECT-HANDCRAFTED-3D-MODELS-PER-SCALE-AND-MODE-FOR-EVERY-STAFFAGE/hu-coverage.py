"""🧑‍🦽 Verifies human pose bindings, tissue allocation and wearable material coverage from the shipped documents."""
from pathlib import Path
import json,math
ticket=Path(__file__).resolve().parent
root=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
counts=[0,0,0]
for file in sorted(root.glob('*/*.json')):
    a=json.loads(file.read_text())['asset'];regions={r['id']:r for r in a['thermal']['regions']}
    assert set(a['volumes'])=={'50','100','200','500'}
    for v in a['volumes'].values():
        assert v['status']=='authored'
        by={q['id']:q for q in v['body']}
        names={q['region'] for q in v['thermal']}
        assert names==regions.keys(),(a['id'],names^regions.keys())
        for q in v['thermal']:
            solid=by[q['of']] if 'of' in q else q
            r=regions[q['region']]
            if r['kind']=='skin':
                assert solid['material'] not in ['cotton-clothing','silicone-liner','denim','leather-shoe','carbon-socket'],(a['id'],q['id'],solid['material'])
        counts[2]+=sum(len(v[y]) for y in ['body','thermal','structure'])
    for pose in a['poses']:assert pose['thermal']['regions'].keys()==regions.keys(),(a['id'],pose['id'])
    for subject in set(r['body'].get('subject') for r in regions.values() if r['model']=='body'):
        existing=[r['body'] for r in regions.values() if r['model']=='body' and r['body'].get('subject')==subject]
        for name in ['skinFraction','metabolicShare']:
            assert math.isclose(sum(r[name] for r in existing),1,abs_tol=.001),(a['id'],subject,name)
    counts[0]+=1;counts[1]+=len(a['poses'])
print(*counts)
