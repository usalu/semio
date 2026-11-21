"""👁️ Distinguishes the pigmented brows from sculpted, skin-covered supraorbital ridges."""
from pathlib import Path
from runpy import run_path
import json,re
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
for slug in ['leaning-person','skateboarder','standing-adult','standing-phone-user','worker-carrying-box']:
    p=next((root/slug).iterdir());old=p.read_text();text=old;a=json.loads(old)['asset']
    def replace(start,id,items):
        global text
        i=text.rindex('{',0,text.index('"id": "'+id+'"',start));j=sp['member_end'](text,i);depth=(i-text.rindex('\n',0,i)-1)//2
        text=text[:i]+(',\n'+'  '*depth).join(sp['dump'](q,depth,'compact') for q in items)+text[j:]
    if slug=='skateboarder' and not any(m['id']=='hair' for m in a['materials']):
        last=a['materials'][-1]
        replace(text.index('"materials": ['),last['id'],[last,{'id':'hair','title':{'en':'Eyebrow hair','de':'Augenbrauenhaar'},'emissivity':.95,'solarAbsorptivity':.82,'source':'authored-estimate'}])
    cap=slug in ['leaning-person','skateboarder']
    region=next((r for r in a['thermal']['regions'] if r['id']=='hair' and r['kind']=='hair'),None)
    for d,v in a['volumes'].items():
        start=lambda key:text.index('"'+key+'": [',text.index('"'+d+'": {',text.index('"volumes": {')))
        brows={q['id'] for q in v['body'] if q['id'] in ['brow','brow-l','brow-r'] and q['tone']<=.35}
        for q in v['body']:
            if q['id'] in brows and q['material']!='hair':replace(start('body'),q['id'],[{**q,'material':'hair'}])
        if region:
            for q in v['thermal']:
                if q.get('of') in brows and q['region']!=region['id']:
                    q={**q,'region':region['id']}
                    if d=='50':q['layer']=region['layers'][-1]['id']
                    replace(start('thermal'),q['id'],[q])
    def draw(m):
        q=json.loads(m[0]);q['material']='hair'
        if region:q['region']=region['id']
        return sp['dump'](q,6,'compact')
    text=re.sub(r'\{\n              "id": "brow(?:-[lr])?(?:-line)?".*?\n            \}',draw,text,flags=re.S)
    if cap and not any('eyebrow hairs' in q['en'] for q in a['thermal']['assumptions']):
        before=a['thermal']['assumptions'][-1]
        i=text.index('"assumptions": [',text.index('"thermal": {\n      "environment"'));j=sp['member_end'](text,text.index('[',i))-1
        new={'en':'The sparse eyebrow hairs follow the heat field of the local forehead skin. The insulated hat covers the scalp above them.','de':'Die dünnen Augenbrauenhaare folgen dem Wärmefeld der örtlichen Stirnhaut. Die isolierende Mütze bedeckt die Kopfhaut darüber.'}
        k=text.rfind('}',i,j)+1;text=text[:k]+',\n        '+sp['dump'](new,4,'compact')+text[k:]
    assert json.loads(text) and p.read_text()==old
    if text!=old:p.write_text(text)
