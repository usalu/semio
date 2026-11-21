"""🌿 Publish the reviewed canopy fields while preserving concurrently authored catalogue content."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json
import sys
ticket=Path(__file__).resolve().parent
slug,filename,expected=sys.argv[1:]
catalog=next(Path.cwd().glob('🎓*/*architecture/*staffage/🗂*/*/'+slug+'/🔣️.json'))
source=ticket/'volumes/vegetation'/(slug+'.json')
candidate=json.loads((ticket/'🗑️generated'/filename).read_text())['asset']
locator=run_path(str(ticket/'r5-range-gaps-lib.py'))['key_value']
def replace(text,wrapped,keys):
    start=locator(text,'asset',0,len(text))[0] if wrapped else text.index('{')
    spans=sorted(((*locator(text,key,start,len(text)),key) for key in keys),reverse=True)
    for lo,hi,key in spans:
        text=text[:lo]+json.dumps(candidate[key],separators=(',',':'),ensure_ascii=False)+text[hi:]
    return text
old=catalog.read_bytes();other=source.read_bytes()
assert sha256(old).hexdigest()==expected
asset=json.loads(old)['asset'];model=json.loads(other)
fields={'volumes','representations'}
assert {k:v for k,v in asset.items() if k not in fields}=={k:v for k,v in candidate.items() if k not in fields}
assert asset['volumes']==model['volumes']
changed={k for k in fields if asset[k]!=candidate[k]}
a=replace(old.decode(),True,changed);b=replace(other.decode(),False,['volumes']) if 'volumes' in changed else other.decode()
assert json.loads(a)['asset']==candidate and json.loads(b)==dict(model,volumes=candidate['volumes'])
assert old==catalog.read_bytes() and other==source.read_bytes()
if old!=a.encode():catalog.write_text(a)
if other!=b.encode():source.write_text(b)
print(slug,sha256(a.encode()).hexdigest())
