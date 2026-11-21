"""🖋️ Give the narrow distal outlines their minimum printable pen across the complete final band."""
from pathlib import Path
import json
t=Path(__file__).parent
for n in ('pig','sheep','roe-deer'):
    a=json.loads((t/'🗑️generated/living-current'/f'{n}-paper-500.json').read_text());ids={id for check in a['checks'] for issue in check['findings'] for id in issue['shapes'] if issue['check']=='feature'}
    p=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/'+n+'/*.json'));raw=p.read_text();i=raw.index('{',raw.index('"representations":'));d=json.JSONDecoder();reps,end=d.raw_decode(raw,i)
    for shapes in reps['500']['geometry'].values():
        for s in shapes:
            if s['id'] in ids:s['penMm']=.2
    new=raw[:i]+json.dumps(reps,ensure_ascii=False,separators=(',',':'))+raw[end:]
    if raw!=p.read_text():raise RuntimeError(p)
    p.write_text(new)

