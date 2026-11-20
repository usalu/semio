"""🚴 Removes fictitious scalp-cut discs from the thermal crown and lining of the two helmets."""
from pathlib import Path
from runpy import run_path
import json

ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
for slug in ('child-on-kick-scooter','cyclist'):
    path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    old=path.read_text()
    text=old
    offset=text.index('"volumes": ')
    for scale,v in json.loads(old)['asset']['volumes'].items():
        for p in v['thermal']:
            if p.get('of')=='head-helmet' or p['id'] not in ('th-helmet','th-helmet-hm-shell','th-helmet-hm-air','th-helmet-hm-eps','th-helmet-hm-hair','th-helmet-helmet-air'):continue
            bare=next((s for s in v['body'] if s['id']==p.get('of')),None)
            if bare:
                made={**bare,**p}
                del made['of']
                made['capStart']=False
            else:made={**p,'capStart':False}
            i=text.index('{\n            "id": "'+p['id']+'"',offset)
            j=sp['member_end'](text,i)
            replacement=sp['dump'](made,5,'compact')
            text=text[:i]+replacement+text[j:]
            offset=i+len(replacement)
    assert json.loads(text)
    assert path.read_text()==old
    path.write_text(text)
