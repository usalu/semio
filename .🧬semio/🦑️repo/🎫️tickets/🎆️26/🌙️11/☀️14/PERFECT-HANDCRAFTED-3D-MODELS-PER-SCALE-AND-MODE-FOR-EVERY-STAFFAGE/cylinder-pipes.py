"""💧 Authors round enamelled-steel tappings and concentric brass unions across the four digital scales."""
from pathlib import Path
import math,json,copy
t=Path(__file__).resolve().parent;out=t/'🗑️generated'/'rotor';r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/*/hot-water-cylinder/*'));doc=json.loads(r.read_bytes());a=doc['asset'];fine=copy.deepcopy(a['volumes']['50']);fb={q['id']:q for q in fine['body']};fs={q['id']:q for q in fine['structure']}
def circ(y,rad,n):return [[round(y+rad*math.cos(i*math.tau/n),7),round(rad*math.sin(i*math.tau/n),7)]for i in range(n-1,-1,-1)]
def tubing(id,side,start,y,rad,inner,length):
 return {'id':id,'kind':'revolve','bone':'connections','material':'enamelled-steel','role':'body','tone':.52,'profile':[[0,inner],[0,rad],[length,rad],[length,inner],[0,inner]],'origin':[side*start,y,0]if side else [.2039,y,0],'axis':[side,0,0] if side else [0,1,0]}
for d,n in [(50,32),(100,24),(200,20),(500,16)]:
 v=a['volumes'][str(d)];mesh=json.loads((out/('cylinder-'+str(d)+'.json')).read_bytes());zones=list(range(1,5));by={q['id']:q for q in mesh}
 ignore={'lower','upper','cap','hot-out','side-l','side-r',*('zone-'+str(j)for j in zones)}
 v['body']=[q for q in v['body']if q['id']not in ignore and not q['id'].startswith(('stub-','nut-'))]+mesh
 for i in range(5):
  q=fb['stub-'+str(i)];y=q['origin'][1];side=q['axis'][0]
  pipe=tubing(q['id'],side,.235,y,.012,.0085,.095)
  nut={**fb['nut-'+str(i)],'holes':[circ(y,.012,32)]}
  v['body'].extend([pipe,nut])
 v['body'].append(tubing('hot-out',0,0,1.24155,.015,.011,.23645))
 v['thermal']=[q for q in v['thermal']if q['id'].startswith('th-top-plume')]
 for j in zones:
  for name,layer in [('tank','steel'),('zone','jacket')]:
   layer=next((q.get('layer')for q in fine['thermal']if q['id']==('th-steel-'if name=='tank'else'th-jacket-')+str(j)),layer)
   v['thermal'].append({'id':'th-'+name+'-'+str(j),'of':name+'-'+str(j),'region':'zone-'+str(j),'layer':layer,'sectionFaces':['-v','+v']})
 v['thermal'].append({'id':'th-cap','of':'cap','region':'top-cap','layer':'shell','sectionFaces':['-v']})
 for i in range(6):
  key='hot-out'if i==5 else 'stub-'+str(i)
  q=next(q for q in fine['thermal'] if q['id']=='th-'+key);q={**q,'material':'enamelled-steel'}
  rr=.015 if i==5 else .012;ri=.011 if i==5 else .0085;end=q['profile'][-1][0];q['profile']=[[0,ri],[0,rr],[end,rr],[end,ri],[0,ri]]
  v['thermal'].append(q)
  if i<5:v['thermal'].append({'id':'th-nut-'+str(i),'of':'nut-'+str(i),'region':'connections','layer':'tube'})
 v['structure']=[q for q in v['structure']if not q['id'].startswith(('shell-','pipe-','weld-pipe-')) and q['id']!='cover-skin']
 for j in zones:v['structure'].append({'id':'shell-'+str(j),'of':'tank-'+str(j),'member':'zone-'+str(j),'part':'member'})
 for q in [fs['cover-skin']]:
  v['structure'].append({'id':q['id'],'of':'cap','member':q['member'],'part':q['part']})
 for i in range(6):
  tag='hot' if i==5 else str(i);base=fs['pipe-'+tag];v['structure'].append({'id':base['id'],'of':'hot-out' if i==5 else'stub-'+tag,'member':base['member'],'part':'member'});v['structure'].append(fs['weld-pipe-'+tag])
 v['decisions'].append({'en':'Five separate steel side tappings retain a 24 mm outside diameter and 17 mm bore. The top outlet is 30 mm outside and 22 mm inside. The brass unions, pierced polyurethane insulation, domed cover and three-millimetre enamelled-steel pressure shell share the real pipe axes.','de':'Fünf einzelne Stahlstutzen behalten 24 mm Außen- und 17 mm Innendurchmesser. Der obere Anschluss hat 30 mm Außen- und 22 mm Innendurchmesser. Messingverschraubungen, durchbrochene PUR-Dämmung, gewölbte Abdeckung und 3 mm starke emaillierte Stahlwand sind auf die tatsächlichen Rohrachsen ausgerichtet.'})
(out/'hot-water-cylinder.json').write_text(json.dumps(doc,ensure_ascii=False))
