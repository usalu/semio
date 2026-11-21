"""🛏️ Gives the 1∶500 folded duvet a separate articulating calf mound beyond the sleeper's knee."""
from pathlib import Path
from runpy import run_path
import json
ticket=Path(__file__).resolve().parent
p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/*'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
old=p.read_text();text=old
a=json.loads(old)['asset'];v=a['volumes']['500']
if any(q['id']=='duvet-calves' for q in v['body']):
    assert any(q.get('of')=='duvet-calves' for q in v['thermal'])
    raise SystemExit(0)
prox=next(q for q in v['body'] if q['id']=='duvet-legs')
dist=next(q for q in a['volumes']['200']['body'] if q['id']=='duvet-shins')
calf={**prox,'id':'duvet-calves','bone':'shank-r','weights':dist['weights'],'transform':dist['transform'],'profile':[[1.434,0],[1.456,.163],[1.495,.202],[1.61,.191],[1.747,.178],[1.782,.15],[1.80,0]]}
prox={**prox,'profile':[[1.12,0],[1.13,.196],[1.215,.232],[1.30,.2264],[1.46,.202],[1.50,.178],[1.52,0]]}
ref=next(q for q in v['thermal'] if q['id']=='th-duvet-legs')
for original,items in [(prox['id'],[prox,calf]),(ref['id'],[ref,{**ref,'id':'th-duvet-calves','of':'duvet-calves'}])]:
    i=text.index('{\n            "id": "'+original+'"',text.index('"500": {',text.index('"volumes": {')))
    j=sp['member_end'](text,i)
    text=text[:i]+',\n          '.join(sp['dump'](q,5,'compact') for q in items)+text[j:]
assert json.loads(text) and p.read_text()==old
p.write_text(text)
