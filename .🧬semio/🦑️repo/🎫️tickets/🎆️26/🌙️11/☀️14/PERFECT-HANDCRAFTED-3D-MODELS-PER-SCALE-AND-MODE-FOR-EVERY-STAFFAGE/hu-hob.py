"""🍳 Closes the four off-axis cooking-zone ink profiles as annuli at each engraved scale."""
from pathlib import Path
import json,sys
from runpy import run_path
ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/person-cooking/*'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
old=path.read_text()
text=old
offs=text.index('"volumes": ')
for scale in ('50','100'):
    v=json.loads(old)['asset']['volumes'][scale]
    zones=[p for p in v['body'] if p['id'].startswith('zone-')]
    assert len(zones)==4
    for p in zones:
        if p['profile'][0]==p['profile'][-1]:continue
        assert '--write' in sys.argv,'open section on '+scale+' '+p['id']
        p['profile'].append(p['profile'][0])
        i=text.index('{\n            "id": "'+p['id']+'"',offs)
        j=sp['member_end'](text,i)
        replacement=sp['dump'](p,5,'compact')
        text=text[:i]+replacement+text[j:]
        offs=i+len(replacement)
if old!=text:
    assert path.read_text()==old
    path.write_text(text)
