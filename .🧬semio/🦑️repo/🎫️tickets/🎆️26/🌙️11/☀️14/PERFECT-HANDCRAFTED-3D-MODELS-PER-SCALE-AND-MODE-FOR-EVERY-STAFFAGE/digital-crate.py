"""🧺 Publish the moulded crate stock and the corresponding sections into the shared catalog."""
from pathlib import Path
import json
from runpy import run_path
T=Path(__file__).resolve().parent
builder=run_path(str(T/'r3-storage-crate-stack-build.py'))
lib=run_path(str(T/'r5-range-gaps-lib.py'))
catalog=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/storage-crate-stack/🔣️.json'))
for p in (T/'volumes/objects/storage-crate-stack.json',catalog):
    before=p.read_text();new=before;doc=json.loads(before);a=doc.get('asset',doc)
    for scale in a['volumes']:
        existing=a['volumes'][scale];v=builder['volume'](scale)
        for section in ('body','thermal','structure'):
            by_id={x['id']:x for x in existing[section]}
            for s in v[section]:
                old=by_id.get(s['id'],{})
                for key in ('layer','tags','envelope','weights'):
                    if key in old:s[key]=old[key]
        lo,hi=lib['key_value'](new,scale,*lib['key_value'](new,'volumes',0,len(new)))
        line=new[new.rfind('\n',0,lo)+1:lo];new=new[:lo]+lib['render'](v,len(line)-len(line.lstrip()),2)+new[hi:]
        a['volumes'][scale]=v
    if p==catalog:
        draw=run_path(str(T/'r3-storage-crate-stack-roots.py'))
        for scale,r in a['representations'].items():
            r.update(geometry=draw['drawings'](scale),sections={'front':{'plane':0,'label':{'en':'Longitudinal crate section','de':'Kistenlängsschnitt'}}})
            r['decisions']['thinElements']={'en':'The front section crosses the 3 mm moulded sidewalls and 8 mm base plate. The line weight keeps these cut edges legible.','de':'Der Längsschnitt trifft die 3-mm-Formwände und die 8-mm-Bodenplatte. Die Strichstärke macht die Schnittkanten lesbar.'}
            lo,hi=lib['key_value'](new,scale,*lib['key_value'](new,'representations',0,len(new)))
            line=new[new.rfind('\n',0,lo)+1:lo];new=new[:lo]+lib['render'](r,len(line)-len(line.lstrip()),2)+new[hi:]
    assert doc==json.loads(new) and p.read_text()==before
    if new!=before:p.write_text(new)
