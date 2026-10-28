"""🫧 Observe explicit surface topology without imposing solid-volume semantics."""
import ctypes,hashlib,json,math,sys,urllib.request
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom,ifcopenshell.validate
class SurfaceOracle:
 def observe(self,row):raise NotImplementedError
class IfcOpenShellSurfaceOracle(SurfaceOracle):
 def observe(self,row):
  f=ifcopenshell.file(schema='IFC4X3_ADD2');points=[f.create_entity('IfcCartesianPoint',Coordinates=tuple(float(x)for x in point))for point in row['points']];axis=f.create_entity('IfcAxis2Placement3D',Location=points[0]);faces=[]
  for face in row['faces']:
   bounds=[]
   for b in face['bounds']:
    polygon=tuple(points[i]for i in b['loop'])
    if row['damage']=='wrong-point':polygon=(f.create_entity('IfcDirection',DirectionRatios=(1.,0.,0.)),)+polygon[1:]
    loop=f.create_entity('IfcPolyLoop',Polygon=polygon)
    if row['damage']=='wrong-loop':loop=f.create_entity('IfcCartesianPoint',Coordinates=(0.,0.,0.))
    bound=f.create_entity('IfcFaceOuterBound'if b['outer']else'IfcFaceBound',Bound=loop,Orientation=b['orientation'])
    if row['damage']=='wrong-bound':bound=f.create_entity('IfcPlane',Position=axis)
    bounds.append(bound)
   faces.append(f.create_entity('IfcFace',Bounds=tuple(bounds)))
  groups=[f.create_entity(g['kind'],CfsFaces=tuple(faces[i]for i in g['faces']))for g in row['groups']]
  if row['damage']=='duplicate-group-reference':groups.append(groups[0])
  if row['damage']=='empty-groups':groups=[]
  model=f.create_entity('IfcFaceBasedSurfaceModel',FbsmFaces=tuple(groups));path=out/(row['id']+'.ifc');f.write(str(path));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);result=dict(id=row['id'],header=list(f.header.file_schema.schema_identifiers),sha256=hashlib.sha256(path.read_bytes()).hexdigest(),diagnostics=log.statements)
  try:
   mesh=ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(),model);v=list(mesh.verts);ids=list(mesh.faces);pts=[v[i:i+3]for i in range(0,len(v),3)];area=0
   for at in range(0,len(ids),3):
    a,b,c=[pts[ids[at+k]]for k in range(3)];u=[b[k]-a[k]for k in range(3)];w=[c[k]-a[k]for k in range(3)];cross=[u[1]*w[2]-u[2]*w[1],u[2]*w[0]-u[0]*w[2],u[0]*w[1]-u[1]*w[0]];area+=math.sqrt(sum(x*x for x in cross))/2
   result['mesh']=dict(vertices=len(pts),triangles=len(ids)//3,area=area)
  except Exception as e:result['geometryError']=str(e)
  return result
out=p/'🗑️generated/surface-geometry';out.mkdir(exist_ok=True);rows=[];oracle=IfcOpenShellSurfaceOracle()
for row in json.loads((p/'surface-geometry-cases.json').read_text(encoding='utf-8'))['cases']:
 try:r=oracle.observe(row)
 except Exception as e:r=dict(id=row['id'],constructionError=str(e))
 rows.append(r);(out/'oracle.json').write_text(json.dumps(dict(version=ifcopenshell.version,rows=rows),indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(dict(id=row['id'],diagnostics=len(r.get('diagnostics',[])),mesh=r.get('mesh'),error=r.get('constructionError')or r.get('geometryError'))),flush=True)
for name in ['IfcFaceBasedSurfaceModel','IfcConnectedFaceSet','IfcOpenShell']:
 url='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/'+name+'.htm';data=urllib.request.urlopen(url).read();(out/(name+'.htm')).write_bytes(data);print('[DEBUG] '+name+' '+hashlib.sha256(data).hexdigest())
