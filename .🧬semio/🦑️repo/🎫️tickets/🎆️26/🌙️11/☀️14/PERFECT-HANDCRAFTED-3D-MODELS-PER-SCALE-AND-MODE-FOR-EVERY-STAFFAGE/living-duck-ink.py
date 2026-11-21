"""🖋️ Give the mallard eye and terminal primary their 0.20 mm printable contour across the supported band."""
import json
from pathlib import Path
p=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/mallard-duck/*.json'));raw=p.read_text();start=raw.index('{',raw.index('"representations":'));reps,n=json.JSONDecoder().raw_decode(raw[start:])
for k,ids in [('50',('eye-f','eye-n')),('200',('near-outer-primary','far-outer-primary'))]:
    for shapes in reps[k]['geometry'].values():
        for s in shapes:
            if s['id'] in ids:s['penMm']=.2
out=raw[:start]+json.dumps(reps,ensure_ascii=False,separators=(',',':'))+raw[start+n:]
if p.read_text()!=raw:raise RuntimeError(p)
p.write_text(out)
