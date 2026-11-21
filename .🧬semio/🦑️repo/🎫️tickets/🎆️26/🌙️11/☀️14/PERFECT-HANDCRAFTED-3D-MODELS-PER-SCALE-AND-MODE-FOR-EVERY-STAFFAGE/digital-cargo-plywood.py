"""🪵 Assign the exterior area of the cargo box and its closed storage bench."""
from pathlib import Path
import json
T=Path(__file__).resolve().parent
CAT=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/cargo-bike/🔣️.json'))
def author(asset):
    wood=next(r for r in asset['thermal']['regions'] if r['id']=='box-wood')
    wood['areaM2']=2.83
if __name__=='__main__':
    doc=json.loads(CAT.read_bytes());author(doc['asset'])
    (T/'🗑️generated/digital-cargo-plywood.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
