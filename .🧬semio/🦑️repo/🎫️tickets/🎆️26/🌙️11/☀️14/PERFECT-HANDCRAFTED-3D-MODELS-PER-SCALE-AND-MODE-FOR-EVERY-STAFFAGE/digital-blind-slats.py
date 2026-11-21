"""🪟 Carry all eighteen articulated slats through the coarse façade and its heat-transfer surface."""
from copy import deepcopy
from pathlib import Path
import json
T=Path(__file__).resolve().parent
CAT=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/external-venetian-blind/🔣️.json'))
def author(a):
    reference=a['volumes']['200'];coarse=a['volumes']['500']
    for group in ('body','thermal'):
        def is_slat(s):return s['id'].startswith(('slat-','th-slat-'))
        coarse[group][:]=[q for q in coarse[group] if not is_slat(q)]+[deepcopy(s) for s in reference[group]if is_slat(s)]
    for view,drawing in a['representations']['500']['geometry'].items():
        reduced=a['representations']['200']['geometry'][view]
        drawing[:]=[q for q in drawing if not q['bone'].startswith('slat-')]+[deepcopy(q)for q in reduced if q['bone'].startswith('slat-')]
    note=dict(en='All eighteen blind slats keep their individual tilt axes and 1.13 m spans. The small-scale solid uses the folded cross section without the fine edge returns.',de='Alle achtzehn Lamellen behalten ihre eigenen Drehachsen und 1,13 m Spannweite. Der Körper im kleinen Maßstab zeigt den gekanteten Querschnitt ohne die feinen Randfalze.')
    if note not in coarse['decisions']:coarse['decisions'].append(note)
if __name__=='__main__':
 doc=json.loads(CAT.read_text());author(doc['asset']);(T/'🗑️generated/digital-blind-slats.json').write_text(json.dumps(doc,ensure_ascii=False))
