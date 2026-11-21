"""🦶 Places each stance correction into its named pose, preserving all independent catalog edits."""
from pathlib import Path
from runpy import run_path
import json
ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
for item in json.loads((ticket/'🗑️generated/hu-review/plants.json').read_text()):
    p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+item['slug']+'/*'))
    old=p.read_text();a=json.loads(old)['asset']
    pose=next(x for x in a['poses'] if x['id']==item['id'])
    for bone,angle in item['changes'].items():pose['bones'][bone]['angleDeg']=angle
    i=old.index('"bones": {',old.index('"id": "'+item['id']+'"',old.index('"poses": [')))+len('"bones": ')
    j=sp['member_end'](old,i)
    text=old[:i]+sp['dump'](pose['bones'],4,'compact')+old[j:]
    assert json.loads(text)['asset']==a and p.read_text()==old
    p.write_text(text)
