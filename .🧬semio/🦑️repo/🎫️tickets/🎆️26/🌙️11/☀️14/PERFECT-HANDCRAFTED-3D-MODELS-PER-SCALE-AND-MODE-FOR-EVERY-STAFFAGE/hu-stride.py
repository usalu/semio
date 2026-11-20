"""🦵 Applies only the counter-rotated leg angles of the calibrated rollator stride."""
from pathlib import Path
import json,re
ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/person-with-rollator/*'))
text=path.read_text()
old=text
document=json.loads(text)
c=next(c for c in document['asset']['clips'] if c['id']=='rollator-walk')
keys=json.loads((ticket/'🗑️generated/hu-review/stride.json').read_text())
offset=text.index('"keys": [',text.index('"id": "rollator-walk"'))
for key,after in zip(c['keys'],keys):
    for bone in key['bones']:
        if bone not in ('thigh-l','shank-l','thigh-r','shank-r'):continue
        j=text.index('"angleDeg": ',text.index('"'+bone+'": {',offset))+len('"angleDeg": ')
        k=j+len(re.match(r'[-.0-9eE+]+',text[j:])[0])
        v=after['bones'][bone]['angleDeg']
        assert float(text[j:k])==key['bones'][bone]['angleDeg'] and abs(v-key['bones'][bone]['angleDeg'])<.1
        text=text[:j]+str(v)+text[k:]
        key['bones'][bone]['angleDeg']=v
        offset=j+len(str(v))
assert c['keys']==keys and json.loads(text)==document
assert path.read_text()==old
path.write_text(text)
