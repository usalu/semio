"""🌡️ Separates exposed, warmed service steel from the brass coupling faces while preserving the source's nine-watt heat input."""
from pathlib import Path
import json,copy,math
t=Path(__file__).resolve().parent;p=t/'🗑️generated/rotor/hot-water-cylinder.json';doc=json.loads(p.read_bytes());a=doc['asset'];connection=next(q for q in a['thermal']['regions'] if q['id']=='connections');brass=copy.deepcopy(connection)
brass.update(id='unions',title={'en':'Brass union faces','de':'Messingverschraubungen'},areaM2=.0162,characteristicLengthM=.033)
brass['operating']['heatW']=4.5;brass['layers'][0]['title']={'en':'Brass union wall','de':'Messingwand der Verschraubung'}
connection.update(material='enamelled-steel',areaM2=.0256,title={'en':'Exposed steel service pipes','de':'Freiliegende Stahlstutzen'})
connection['operating']['heatW']=4.5;l=connection['layers'][0];l.update(material='enamelled-steel',title={'en':'Steel service pipe wall','de':'Stahlwand des Anschlussstutzens'},thicknessM=.0035,resistanceM2KW=.0035/50,heatCapacityJM2K=.0035*7850*460);connection['heatCapacityJM2K']=l['heatCapacityJM2K']
a['thermal']['regions'].append(brass)
for pose in a['poses']:
 pose['thermal']['regions']['unions']={**pose['thermal']['regions']['connections'],'exposedFraction':.72}
 pose['thermal']['regions']['connections']['exposedFraction']=.42
original=json.loads(next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/*/hot-water-cylinder/*')).read_bytes())['asset']
fb={q['id']:q for q in original['volumes']['50']['body']}
for d,v in a['volumes'].items():
 v['thermal']=[q for q in v['thermal']if not (d!='50' and q['id'].startswith('th-tank-'))]
 for i,q in enumerate(v['thermal']):
  key=q.get('of')
  if key=='cap' or key in ['zone-1','zone-2','zone-3','zone-4']:
   contour=fb[key] if key!='cap' else next(b for b in original['volumes'][d]['body']if b['id']=='cap')
   v['thermal'][i]={**contour,**{k:x for k,x in q.items()if k!='of'}}
 for q in v['thermal']:
  if q['id'].startswith('th-nut-'):
   q['region']='unions'
   if d in ['200','500']:
    nut=next(k for k in v['body']if k['id']==q['of']);n=20 if d=='200'else 14
    y=next(k for k in v['body']if k['id']==q['of'].replace('nut-','stub-'))['origin'][1]
    hole=[[round(y+.012*math.cos(i*math.tau/n),7),round(.012*math.sin(i*math.tau/n),7)] for i in range(n-1,-1,-1)]
    data={**nut,'holes':[hole],**{k:x for k,x in q.items() if k!='of'}};q.clear();q.update(data)
  if q['id'].startswith('th-stub-'):
   q['profile']=[[0,.0085],[0,.012],[.03,.012],[.03,.0085],[0,.0085]]
 for q in v['body']:
  if q['id'].startswith('nut-'):q['material']='brass'
p.write_text(json.dumps(doc,ensure_ascii=False))
