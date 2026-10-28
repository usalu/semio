"""🪢 Independent full EXPRESS and normal geometry comparison for emitted profile sources."""
import ctypes,json,sys,hashlib
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom,ifcopenshell.validate
out=t/'🗑️generated/polyline-extrusion';owned=json.loads((out/'owned.json').read_text());rows=[]
def measure(points,indices):
 volume=0.
 for i in range(0,len(indices),3):
  a,b,c=(points[indices[i+j]]for j in range(3));volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6
 return {'bounds':[[min(p[k]for p in points)for k in range(3)],[max(p[k]for p in points)for k in range(3)]],'signedVolume':volume,'vertices':len(points),'triangles':len(indices)//3}
for row in owned['rows']:
 path=out/(row['id']+'.ifc');f=ifcopenshell.open(str(path));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);diagnostics=[{'level':r.get('level'),'attribute':r.get('attribute'),'message':str(r.get('message'))}for r in logger.statements];settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);shape=ifcopenshell.geom.create_shape(settings,f.by_id(row['product']));v=list(shape.geometry.verts);independent=measure([v[i:i+3]for i in range(0,len(v),3)],list(shape.geometry.faces));actual=measure(row['positions'],row['indices']);bounds_error=max(abs(actual['bounds'][i][j]-independent['bounds'][i][j])for i in range(2)for j in range(3));rows.append({'id':row['id'],'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'header':list(f.header.file_schema.schema_identifiers),'diagnostics':diagnostics,'independent':independent,'actual':actual,'boundsMaxError':bounds_error,'volumeError':abs(abs(actual['signedVolume'])-abs(independent['signedVolume']))})
(out/'independent.json').write_text(json.dumps({'library':ifcopenshell.version,'rows':rows,'nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps([{'id':r['id'],'diagnostics':len(r['diagnostics']),'boundsError':r['boundsMaxError'],'volumeError':r['volumeError']}for r in rows]))
