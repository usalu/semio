"""🧥 Exposes the denim below the carrier's swinging coat hem in each named pose."""
from pathlib import Path
import re,json
ticket=Path(__file__).resolve().parent
p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/parent-carrying-toddler/*'))
old=p.read_text();text=old
def bind(id,val,text):
    pos=text.index('"id": "'+id+'"',text.index('"poses": ['))
    at=text.index('"jeans": {',pos)
    m=re.search(r'"exposedFraction": ([\d.]+)',text[at:])
    return text[:at+m.start(1)]+str(val)+text[at+m.end(1):]
for id,val in {'standing-carry':.39,'heel-strike-l':.44,'walk-1':.45,'midstance-l':.41,'walk-3':.44,'walk-4':.44,'walk-5':.45,'walk-6':.41,'walk-7':.44,'hoisting':.42}.items():text=bind(id,val,text)
assert json.loads(text) and old==p.read_text()
p.write_text(text)
