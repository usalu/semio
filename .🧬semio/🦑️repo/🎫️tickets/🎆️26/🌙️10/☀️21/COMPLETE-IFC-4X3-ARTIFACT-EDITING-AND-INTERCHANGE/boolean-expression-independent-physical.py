"""🪚 Observe authored nested Boolean STEP without reconstructing source."""
from pathlib import Path
import json,hashlib,math
import ifcopenshell,ifcopenshell.validate,ifcopenshell.geom
p=Path(__file__).parent; rows=json.loads((p/'🗑️generated/boolean-expression-actual/emitted.json').read_text(encoding='utf8'))['records']; out=[]
for row in rows:
 raw=Path(row['path']).read_bytes(); r={'id':row['id'],'sha256':hashlib.sha256(raw).hexdigest(),'hashMatches':hashlib.sha256(raw).hexdigest()==row['sha256'],'expectedVolume':row['volume']}
 try:
  f=ifcopenshell.open(row['path']);r['schema']=f.schema;r['entities']=len(list(f));r['nodes']=[{'id':n['id'],'actual':f.by_id(int(n['id'])).is_a(),'expected':n['name']} for n in row['nodes']];log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);r['diagnostics']=log.statements
  try:
   shape=ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(),f.by_id(int(row['product'])));v=list(shape.geometry.verts);ix=list(shape.geometry.faces);pts=[v[i:i+3] for i in range(0,len(v),3)]
   volume=math.fsum((pts[ix[i]][0]*(pts[ix[i+1]][1]*pts[ix[i+2]][2]-pts[ix[i+1]][2]*pts[ix[i+2]][1])+pts[ix[i]][1]*(pts[ix[i+1]][2]*pts[ix[i+2]][0]-pts[ix[i+1]][0]*pts[ix[i+2]][2])+pts[ix[i]][2]*(pts[ix[i+1]][0]*pts[ix[i+2]][1]-pts[ix[i+1]][1]*pts[ix[i+2]][0]))/6 for i in range(0,len(ix),3))
   r['mesh']={'status':'nonempty' if ix else 'empty','vertices':len(pts),'triangles':len(ix)//3,'signedVolume':volume}
  except Exception as e:r['mesh']={'status':'unsupported','error':str(e)}
 except Exception as e:r['parseError']=str(e)
 out.append(r);print('[DEBUG]',r['id'],len(r.get('diagnostics',[])),r.get('mesh'),flush=True)
(p/'🗑️generated/boolean-expression-independent-physical.json').write_text(json.dumps({'version':ifcopenshell.version,'records':out},indent=2,default=str)+'\n',encoding='utf8')
