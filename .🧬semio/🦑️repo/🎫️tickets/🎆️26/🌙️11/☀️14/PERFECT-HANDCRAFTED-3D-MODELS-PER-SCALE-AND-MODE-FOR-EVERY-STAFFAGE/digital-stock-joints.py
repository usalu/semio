"""🪑 Preserve the bearing timber and cross dowel of the sling chair and the operating spindle of the gas valve."""
from pathlib import Path
from runpy import run_path
from copy import deepcopy
import json,sys
T=Path(__file__).resolve().parent
C=next(T.parents[6].glob('🎓*/*/*staffage/🗂*catalog'))
def author(a):
    chair=a['id']=='folding-deck-chair'
    if chair:run_path(str(T/'digital-print-stock.py'))['author'](a)
    fine=a['volumes']['50']
    for scale in ('100','200','500'):
        volume=a['volumes'][scale]
        for list in ('body','thermal','structure'):
            src=fine[list]
            if chair:
                stock={'back-rail','front-rail','seat-rail','foot-rail','rear-rail','top-rail','strut-arm','strut-bar'}
                parts=[p for p in src if p.get('of',p['id']) in stock]
            else:
                parts=[p for p in src if p.get('of',p['id']) in ('knob','knob-boss','knob-grip','st-knob-stem')]
            keep={p['id']:deepcopy(p) for p in parts}
            items=volume[list]
            for i,entry in enumerate(items):
                if entry['id'] in keep:items[i]=keep.pop(entry['id'])
            items.extend(keep.values())
        if chair:
            for view,items in a['representations'][scale]['geometry'].items():
                source=a['representations']['50']['geometry'][view]
                rails=[deepcopy(q) for q in source if q.get('material')=='beech-timber' and q['role']=='body']
                for q in rails:q['penMm']=.20 if scale=='100' or q['id']=='rear-rail' else .18
                ids={q['id'] for q in rails}
                items[:]=[q for q in items if q['id'] not in ids]+rails
        note={'en':'The paired 20 × 32 mm props, timber rails and 580 mm turned cross dowel remain at their bearing dimensions as sling and hardware detail simplifies.','de':'Die paarigen 20 × 32 mm starken Stützen, Holzholme und der 580 mm lange gedrechselte Querstab behalten beim Vereinfachen von Bespannung und Beschlägen ihre Lagermaße.'} if chair else {'en':'The 47 mm control has 27 mm of axial depth, a finger grip, a mounting boss and an 8 mm steel spindle aligned to its rotation axis.','de':'Das 47 mm breite Stellrad hat 27 mm axiale Tiefe, eine Griffleiste, einen Befestigungsbund und eine 8-mm-Stahlwelle auf seiner Drehachse.'}
        if note not in volume['decisions']:volume['decisions'].append(note)
if __name__=='__main__':
    for slug in sys.argv[1:]:
        doc=json.loads(next(C.glob('*/'+slug+'/🔣️.json')).read_text());author(doc['asset']);(T/'🗑️generated'/('digital-stock-'+slug+'.json')).write_text(json.dumps(doc,ensure_ascii=False))
