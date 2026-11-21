"""🦿 Separates the exposed silicone suspension cuff of the sprinter from silicone under the carbon socket."""
from pathlib import Path
from runpy import run_path
import json,re
ticket=Path(__file__).resolve().parent
p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/runner-with-blade-prosthesis/*'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
old=p.read_text();text=old;a=json.loads(old)['asset']
if any(r['id']=='suspension-liner' for r in a['thermal']['regions']):
    assert all(any(q['region']=='suspension-liner' for q in v['thermal']) for v in a['volumes'].values())
    raise SystemExit(0)
r=next(x for x in a['thermal']['regions'] if x['id']=='socket')
liner={**r,'id':'suspension-liner','title':{'en':'Exposed silicone suspension cuff','de':'Freiliegende Silikonmanschette'},'material':'silicone-liner','areaM2':.055,'layers':[r['layers'][0]],'heatCapacityJM2K':700,'body':{**r['body'],'skinFraction':.031,'metabolicShare':.029}}
r['title']={'en':'Carbon socket and distal adapter','de':'Karbonschaft mit distalem Adapter'};r['body'].update(skinFraction=.014,metabolicShare=.051)
def replace_reg(text,r,extra=None):
    i=text.rindex('{',0,text.index('"id": "'+r['id']+'"',text.index('"regions": [',text.index('"thermal": {\n      "environment"'))))
    j=sp['member_end'](text,i)
    return text[:i]+sp['dump'](r,4,'compact')+(',\n        '+sp['dump'](extra,4,'compact') if extra else '')+text[j:]
text=replace_reg(text,r,liner)
for rid,sh in [('shorts',.135),('leg-skin',.190)]:
    r=next(x for x in a['thermal']['regions'] if x['id']==rid)
    r['body']['skinFraction']=sh
    if rid=='leg-skin':r['areaM2']=.3374
    text=replace_reg(text,r)
text=re.sub(r'("socket": )(\{\n[^{}]*?\})',lambda m:m[0]+',\n            "suspension-liner": '+m[2],text)
start=text.index('"volumes": ')
for key,v in a['volumes'].items():
    for q in v['thermal']:
        if q['id'] not in ('th-liner-thigh','th-liner-knee','th-liner-patella'):continue
        i=text.index('{\n            "id": "'+q['id']+'"',start);j=sp['member_end'](text,i)
        q['region']='suspension-liner'
        if key=='50':q['layer']='liner'
        if q['id']!='th-liner-patella':q['sectionFaces']=['-u','+u']
        term=sp['dump'](q,5,'compact');text=text[:i]+term+text[j:];start=i+len(term)
i=text.index('"representations": {');prefix,draw=text[:i],text[i:]
def shape(m):
    o=json.loads(m[0])
    if o.get('region')=='socket' and o['id'].startswith('liner-'):return m[0].replace('"region": "socket"','"region": "suspension-liner"')
    if o.get('region')=='socket' and '"denominator": 500' in draw[:m.start()]:return m[0].replace('"region": "socket"','"region": "socket",\n              "regions": ["socket", "suspension-liner"]')
    return m[0]
text=prefix+re.sub(r'\{\n              "id": "(?:liner-[^"]*|socket)".*?\n            \}',shape,draw,flags=re.S)
assert json.loads(text) and p.read_text()==old
p.write_text(text)
