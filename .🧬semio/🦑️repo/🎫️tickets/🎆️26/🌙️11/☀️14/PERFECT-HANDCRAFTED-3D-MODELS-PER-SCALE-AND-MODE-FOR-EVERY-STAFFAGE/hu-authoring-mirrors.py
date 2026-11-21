"""🪞 Restores individually certified human scales in the ticket authoring documents and checks every digital scale."""
import json,hashlib,sys,re
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
proof={q['slug']:q['hash'] for q in json.loads((t/'🗑️generated/hu-review/heat-field.json').read_bytes())}
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/*humans'));fixed=[];checked=0
for file in root.glob('*/🔣️.json'):
    slug=file.parent.name;catalog=file.read_bytes();assert hashlib.sha256(catalog).hexdigest()==proof[slug]
    live=json.loads(catalog)['asset']['volumes'];p=t/'volumes'/'humans'/(slug+'.json');raw=p.read_text();text=raw;source=json.loads(raw)
    for d,v in live.items():
        if source['volumes'][d]!=v:
            base=re.search(r'"volumes"\s*:\s*\{',text).end();start=base+re.search(r'"'+d+r'"\s*:\s*\{',text[base:]).end()-1;end=sp['member_end'](text,start)
            text=text[:start]+sp['dump'](v,2,'plain')+text[end:];fixed.append((slug,d))
        checked+=1
    assert json.loads(text)['volumes']==live
    if text!=raw and '--apply' in sys.argv:
        assert p.read_text()==raw and file.read_bytes()==catalog;p.write_text(text)
    assert json.loads(p.read_bytes())['volumes']==live
print('checked',checked,'mirrored',len(fixed),'scales in',len(set(x[0]for x in fixed)),'figures')
