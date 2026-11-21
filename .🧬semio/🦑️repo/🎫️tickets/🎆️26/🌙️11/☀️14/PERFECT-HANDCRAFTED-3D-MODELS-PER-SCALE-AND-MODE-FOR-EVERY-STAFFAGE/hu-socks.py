"""🧦 Models the jogger's exposed ankle sock as cotton, separately from the calf and running shoe."""
from pathlib import Path
from runpy import run_path
import json,re
ticket=Path(__file__).resolve().parent
p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/jogger/*'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
old=p.read_text();text=old;a=json.loads(old)['asset']
if any(r['id']=='ankle-socks' for r in a['thermal']['regions']):
    assert all(any(q['region']=='ankle-socks' for q in v['thermal']) for v in a['volumes'].values())
    raise SystemExit(0)
reg=next(q for q in a['thermal']['regions'] if q['id']=='leg-skin')
shoe=next(q for q in a['thermal']['regions'] if q['id']=='shoes')
sock={**reg,'id':'ankle-socks','title':{'en':'Cotton ankle socks','de':'Baumwoll-Knöchelsocken'},'kind':'clothing','material':'cotton-clothing','areaM2':.030,'characteristicLengthM':.065,'layers':[shoe['layers'][0]],'heatCapacityJM2K':300,'body':{**reg['body'],'metabolicShare':.014,'skinFraction':.015},'reference':'iso-9920-2007'}
reg.update(title={'en':'Bare thighs and calves','de':'Unbedeckte Oberschenkel und Waden'},areaM2=.4611)
reg['body'].update(metabolicShare=.226,skinFraction=.23968)
i=text.rindex('{',0,text.index('"id": "leg-skin"'));j=sp['member_end'](text,i)
text=text[:i]+sp['dump'](reg,4,'compact')+',\n        '+sp['dump'](sock,4,'compact')+text[j:]
def binding(m):
    v=json.loads(m[2])
    return m[0]+',\n            "ankle-socks": '+sp['dump']({'exposedFraction':v['exposedFraction'],'sunlitFraction':v['sunlitFraction']},6,'compact')
text=re.sub(r'("leg-skin": )(\{\n(?:[^{}]*?)\})',binding,text)
start=text.index('"volumes": ')
for key,v in a['volumes'].items():
    for q in v['thermal']:
        if q['id'] not in ['th-shank-l','th-shank-r','th-sock-l','th-sock-r']:continue
        made=[]
        if q['id'].startswith('th-sock'):
            made.append({**q,'region':'ankle-socks','layer':'sock-layer','sectionFaces':['-u','+u']})
        else:
            body=next(qb for qb in v['body'] if qb['id']==q['of']);side=q['id'][-1]
            cut=.385 if key in ('50','100') else .382 if key=='200' else .38
            s=body['sections'];aa,bb=next((aa,bb) for aa,bb in zip(s,s[1:]) if aa['at'][0]<cut<=bb['at'][0]);u=(cut-aa['at'][0])/(bb['at'][0]-aa['at'][0])
            def interp(x,y):return round(x+u*(y-x),6)
            ring={**aa,'at':[interp(x,y) for x,y in zip(aa['at'],bb['at'])],'rx':interp(aa['rx'],bb['rx']),'ry':interp(aa['ry'],bb['ry'])}
            skin={**body,'id':q['id'],'region':'leg-skin','sectionFaces':['-u','+u'],'sections':[q for q in s if q['at'][0]<cut]+[ring]}
            made.append(skin)
            if key in ('200','500'):
                made.append({**body,'id':'th-sock-'+side,'material':'cotton-clothing','region':'ankle-socks','sectionFaces':['-u','+u'],'sections':[ring]+[q for q in s if q['at'][0]>cut]})
        i=text.index('{\n            "id": "'+q['id']+'"',start);j=sp['member_end'](text,i)
        term=',\n          '.join(sp['dump'](v,5,'compact') for v in made)
        text=text[:i]+term+text[j:];start=i+len(term)
i=text.index('"representations": {');prefix,text2=text[:i],text[i:]
def shape(m):
    obj=json.loads(m[0])
    if obj.get('region')!='leg-skin':return m[0]
    if obj['id'].startswith('sock-'):return m[0].replace('"region": "leg-skin"','"region": "ankle-socks"')
    if obj['id'].startswith('shank-') and '"denominator": 200' in text2[:m.start()]:
        return m[0].replace('"region": "leg-skin"','"region": "leg-skin",\n              "regions": ["leg-skin", "ankle-socks"]')
    return m[0]
text=prefix+re.sub(r'\{\n              "id": "(?:sock|shank)-[^"]+".*?\n            \}',shape,text2,flags=re.S)
assert json.loads(text) and p.read_text()==old
p.write_text(text)
