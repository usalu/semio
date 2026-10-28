"""🔷 Observe exact-coordinate polygonal closure without source-index welding."""
import ctypes,hashlib,json,math,sys,urllib.request,re
from pathlib import Path
root=Path(__file__).parent; out=root/'🗑️generated/polygonal-identity';out.mkdir(exist_ok=True);dep=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom,ifcopenshell.validate
class PolygonalOracle:
 def observe(self,row):raise NotImplementedError
class IfcOpenShellPolygonalOracle(PolygonalOracle):
 def observe(self,row):
  f=ifcopenshell.file(schema='IFC4X3_ADD2');p=f.create_entity('IfcCartesianPointList3D',CoordList=tuple(tuple(float(x)for x in v)for v in row['coordinates']));faces=tuple(f.create_entity('IfcIndexedPolygonalFaceWithVoids',CoordIndex=tuple(face['outer']),InnerCoordIndices=tuple(tuple(v)for v in face['holes']))if face['holes']else f.create_entity('IfcIndexedPolygonalFace',CoordIndex=tuple(face['outer']))for face in row['faces']);s=f.create_entity('IfcPolygonalFaceSet',Coordinates=p,Closed=row['closed'],Faces=faces,PnIndex=tuple(row['pnIndex'])if row['pnIndex']is not None else None);raw=f.to_string();path=out/(row['id']+'.ifc');path.write_text(raw,encoding='utf-8');logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);r=dict(id=row['id'],sha256=hashlib.sha256(path.read_bytes()).hexdigest(),header=list(f.header.file_schema.schema_identifiers),diagnostics=logger.statements)
  try:
   owner=ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(),s);v=list(owner.verts);idx=list(owner.faces);pts=[v[i:i+3]for i in range(0,len(v),3)];r['mesh']=dict(vertices=len(pts),triangles=len(idx)//3,bounds=[[min(p[k]for p in pts)for k in range(3)],[max(p[k]for p in pts)for k in range(3)]],volume=math.fsum((pts[idx[i]][0]*(pts[idx[i+1]][1]*pts[idx[i+2]][2]-pts[idx[i+1]][2]*pts[idx[i+2]][1])+pts[idx[i]][1]*(pts[idx[i+1]][2]*pts[idx[i+2]][0]-pts[idx[i+1]][0]*pts[idx[i+2]][2])+pts[idx[i]][2]*(pts[idx[i+1]][0]*pts[idx[i+2]][1]-pts[idx[i+1]][1]*pts[idx[i+2]][0]))/6 for i in range(0,len(idx),3)))
  except Exception as e:r['geometryError']=str(e)
  return r
p=[[0,0,0],[2,0,0],[2,1,0],[0,1,0],[0,0,3],[2,0,3],[2,1,3],[0,1,3]];loops=[[4,3,2,1],[1,2,6,5],[2,3,7,6],[3,4,8,7],[4,1,5,8],[5,6,7,8]]
rows=[]
def add(id,points=p,ls=loops,closed=True,pn=None):rows.append(dict(id=id,coordinates=points,faces=[dict(outer=l,holes=[])for l in ls],pnIndex=pn,closed=closed))
add('shared-indices');dup=[p[j-1]for l in loops for j in l];add('distinct-indices-exact-coordinates',dup,[list(range(i*4+1,i*4+5))for i in range(6)])
add('pnindex-aliases',p,[list(range(i*4+1,i*4+5))for i in range(6)],pn=[j for l in loops for j in l]);add('pnindex-permutation',list(reversed(p)),[[9-j for j in l]for l in loops]);add('explicit-end-repeat',p,[l+[l[0]]for l in loops]);add('consecutive-repeat',p,[l[:1]+l for l in loops]);add('open-false',p,loops[:-1],False);add('open-unset',p,loops[:-1],None)
(root/'polygonal-identity-cases.json').write_text(json.dumps(dict(schema='IFC4X3_ADD2',cases=rows),indent=2)+'\n',encoding='utf-8');oracle=IfcOpenShellPolygonalOracle();result=[oracle.observe(r)for r in rows];(out/'oracle.json').write_text(json.dumps(dict(version=ifcopenshell.version,rows=result),indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps([dict(id=r['id'],errors=len(r['diagnostics']),mesh=r.get('mesh'),error=r.get('geometryError'))for r in result]))
for name in ['IfcPolygonalFaceSet','IfcIndexedPolygonalFace','IfcIndexedPolygonalFaceWithVoids']:
 url='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/'+name+'.htm';data=urllib.request.urlopen(url).read();(out/(name+'.htm')).write_bytes(data);print('[DEBUG] '+name+' '+hashlib.sha256(data).hexdigest())
