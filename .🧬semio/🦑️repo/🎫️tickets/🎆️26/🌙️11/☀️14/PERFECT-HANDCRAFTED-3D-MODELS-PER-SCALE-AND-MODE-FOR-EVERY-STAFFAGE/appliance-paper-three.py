"""📐 Redraws the diffuser backplate edge, the heater cable-entry section, and the MVHR sleeve lip symbols for paper."""
from pathlib import Path
from runpy import run_path
import hashlib,json,sys
t=Path(__file__).resolve().parent;r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/*appliances'));sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
for slug in ['ceiling-diffuser','electric-fan-heater','mvhr-unit']:
    src=next((r/slug).iterdir());raw=src.read_text();doc=json.loads(raw);a=doc['asset'];volumes=json.dumps(a['volumes']);reps=a['representations']
    if slug=='ceiling-diffuser':
        for view in ['front','elevation']:
            next(q for q in reps['50']['geometry'][view]if q['id']=='backplate')['penMm']=.2
    elif slug=='electric-fan-heater':
        for view in ['elevation','plan']:
            next(q for q in reps['100']['geometry'][view]if q['id']=='outlet-grille')['penMm']=.2
        for items in reps['50']['geometry'].values():
            if isinstance(items,list):
                for q in items:
                    if q['id'] in ['boss-left','boss-right','boss-front','boss-back']:q['penMm']=.2
        q=next(q for q in reps['50']['geometry']['elevation']if q['id']=='cord-grommet')
        q.update(kind='rect',filled=True,x=-.098,y=.033,w=.006,h=.0035,penMm=.2)
        for key in ['points','closed']:q.pop(key,None)
    else:
        for q in reps['100']['geometry']['front']:
            if q['id'] in ['spigot-1','spigot-2']:q['penMm']=.13
        for q in reps['100']['geometry']['elevation']:
            if q['id'] in ['bead-1','bead-2','bead-3','bead-4'] and q['h']>.101:
                centre=q['y']+q['h']/2;q['h']=.10;q['y']=round(centre-.05,6)
    c=t/'🗑️generated'/'rotor'/(slug+'.json');c.write_text(json.dumps(doc,ensure_ascii=False))
    if '--apply' in sys.argv:
        text=raw
        for d,rep in reps.items():
            if json.loads(raw)['asset']['representations'][d]==rep:continue
            base=text.index('"representations": {');start=text.index('{',text.index('"'+d+'": {',base));end=sp['member_end'](text,start)
            text=text[:start]+sp['dump'](rep,3,'compact')+text[end:]
        assert src.read_text()==raw and json.loads(text)==doc and json.dumps(doc['asset']['volumes'])==volumes
        if text!=raw:src.write_text(text)
