"""🖋️ Maintain the tapered foot geometry with a 0.2 mm office-laser contour in front view."""
import json
from pathlib import Path

catalog=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/pigeon/*.json'))
source=Path(__file__).parent/'r3-pigeon-2d.json'
for path in (catalog,source):
    raw=path.read_text()
    doc=json.loads(raw)
    representations=doc.get('asset',doc)['representations']
    for s in representations['200']['geometry']['front']:
        if s['id'] in ('near-foot','far-foot'):
            x=-0.02 if s['id']=='near-foot' else 0.02
            bottom=-0.10251 if x<0 else -0.10253
            top=-0.06933
            middle=round(top+(bottom-top)*0.77,5)
            outline=[(x-.0015,top),(x+.0015,top),(x+.0021,middle),(x+.0025,bottom),(x-.0025,bottom),(x-.0021,middle)]
            s.update(filled=True,closed=True,penMm=0.20,points=[[round(u,5),y] for u,y in outline])
    loc=raw.index('"representations":')
    start=raw.index('{',loc)
    old,n=json.JSONDecoder().raw_decode(raw[start:])
    if old==representations: continue
    if 'asset' in doc: patch=json.dumps(representations,indent=2,ensure_ascii=False).replace('\n','\n    ')
    else: patch=json.dumps(representations,ensure_ascii=False,separators=(',',':'))
    if raw!=path.read_text(): raise SystemExit('Pigeon changed concurrently')
    path.write_text(raw[:start]+patch+raw[start+n:])

