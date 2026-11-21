"""🩼 Seats the fine crutch grips against their shafts and finishes every rubber ferrule at its declared ground anchor."""
import json
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
p=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/person-on-crutches/*'))
old=p.read_text();text=old;asset=json.loads(old)['asset']
for k in ['50','100','200']:
    for q in asset['volumes'][k]['body']:
        if q['id'] not in ['crutch-grip-l','crutch-grip-r','crutch-tip-l','crutch-tip-r']:continue
        profile=q['profile']
        if 'tip' in q['id']:new=[[.90333 if h==.9 else h,r] for h,r in profile]
        elif profile[0][0]>0:new=[[0,0],[0,.011],[.009,.012],*profile[1:]]
        else:continue
        if new==profile:continue
        q['profile']=new;idx=text.index('"'+k+'": {',text.index('"volumes": {'));i=text.index('{\n            "id": "'+q['id']+'"',idx);j=sp['member_end'](text,i)
        text=text[:i]+sp['dump'](q,5,'compact')+text[j:]
if text!=old:
    assert json.loads(text) and p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/'person-on-crutches.json';s=snap.read_text();draft=json.loads(s)
    for k in ['50','100','200']:draft['volumes'][k]['body']=asset['volumes'][k]['body']
    assert snap.read_text()==s;snap.write_text(json.dumps(draft,ensure_ascii=False,separators=(',',':'))+'\n')
