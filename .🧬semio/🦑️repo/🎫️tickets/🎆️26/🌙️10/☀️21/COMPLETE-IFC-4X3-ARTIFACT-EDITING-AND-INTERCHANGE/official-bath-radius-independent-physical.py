"""🛁 Compare exact original/canonical/repaired bytes through SDK and high-precision math."""
from pathlib import Path
import json,hashlib,math,sys
import ifcopenshell,ifcopenshell.validate,ifcopenshell.geom
p=Path(__file__).parent;folder=p/'🗑️generated/official-bath-radius-actual';manifest=json.loads((folder/'emitted.json').read_text(encoding='utf8'));sys.path.insert(0,str(p/'🗑️generated/determinant'));import mpmath as mp;mp.mp.dps=100
out=[];models=[]
def plain(x):
 if isinstance(x,ifcopenshell.entity_instance):return {'reference':x.id(),'class':x.is_a()}
 if isinstance(x,(tuple,list)):return [plain(v)for v in x]
 return x
for row in manifest['records']:
 path=Path(row['path']);raw=path.read_bytes();f=ifcopenshell.open(str(path));models.append(f);log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);profile=f.by_id(202);r={'id':row['id'],'sha256':hashlib.sha256(raw).hexdigest(),'hashMatches':hashlib.sha256(raw).hexdigest()==row['sha256'],'header':list(f.header.file_schema.schema_identifiers),'entities':len(list(f)),'radius':profile.RoundingRadius,'radiusCarrier':type(profile.RoundingRadius).__name__,'profileRaw':profile.to_string(),'solidRaw':f.by_id(206).to_string(),'booleanRaw':f.by_id(207).to_string(),'productClass':f.by_id(225).is_a(),'diagnostics':log.statements}
 radius=mp.mpf(profile.RoundingRadius)/1000;ideal=mp.mpf('1.28')-mp.mpf('.7')*(mp.mpf('1.8')*mp.mpf('.6')-(4-mp.pi)*radius**2);finite=mp.mpf('1.28')-mp.mpf('.7')*(mp.mpf('1.8')*mp.mpf('.6')-4*radius**2+36*radius**2*mp.sin(mp.pi/36));r['independentMath']={'N':72,'M':76,'idealVolume':mp.nstr(ideal,90),'finitePolygonVolume':mp.nstr(finite,90),'producerExecuted':False}
 try:
  settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);shape=ifcopenshell.geom.create_shape(settings,f.by_id(225));v=list(shape.geometry.verts);ix=list(shape.geometry.faces);pts=[v[i:i+3]for i in range(0,len(v),3)];terms=[];edges={}
  for at in range(0,len(ix),3):
   a,b,c=[pts[id]for id in ix[at:at+3]];terms.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
   for j in range(3):
    x,y=ix[at+j],ix[at+(j+1)%3];key=tuple(sorted((x,y)));edges.setdefault(key,[]).append(1 if x<y else -1)
  r['mesh']={'status':'nonempty'if ix else'empty','vertices':len(pts),'triangles':len(ix)//3,'signedVolume':math.fsum(terms),'closedOppositeEdges':all(len(q)==2 and sum(q)==0 for q in edges.values()),'bounds':[[min(q[k]for q in pts)for k in range(3)],[max(q[k]for q in pts)for k in range(3)]]if pts else None}
 except Exception as e:r['mesh']={'status':'unsupported','error':str(e)}
 out.append(r);print('[DEBUG]',r['id'],len(log.statements),r['radiusCarrier'],r['mesh'],flush=True)
base={e.id():{k:plain(v)for k,v in e.get_info().items()}for e in models[0]};comparisons=[]
for row,f in zip(manifest['records'][1:],models[1:]):
 current={e.id():{k:plain(v)for k,v in e.get_info().items()}for e in f};changes=[]
 for id in base:
  for key,value in base[id].items():
   if value!=current[id].get(key):changes.append({'entityId':str(id),'field':key,'before':value,'after':current[id].get(key)})
 comparisons.append({'id':row['id'],'sameEntityIds':set(base)==set(current),'changes':changes})
(folder/'independent-physical.json').write_text(json.dumps({'ifcopenshell':ifcopenshell.version,'records':out,'attributeComparisons':comparisons,'sourceUnchanged':hashlib.sha256(Path(manifest['records'][0]['path']).read_bytes()).hexdigest()==manifest['sourceSha256']},indent=2,default=str)+'\n',encoding='utf8')
