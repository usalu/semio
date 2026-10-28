"""🧱 Observe declared-shell counterexamples through normal external physical APIs."""
import ctypes,json,sys,hashlib,math
from pathlib import Path
root=Path(__file__).parent;dep=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
class ShellOracle:
 def observe(self,row):raise NotImplementedError
class IfcOpenShellShellOracle(ShellOracle):
 def observe(self,row):
  import ifcopenshell,ifcopenshell.validate,ifcopenshell.geom
  f=ifcopenshell.file(schema='IFC4X3_ADD2');cache={}
  def point(p):
   p=tuple(float(v)for v in p)
   if p not in cache:cache[p]=f.create_entity('IfcCartesianPoint',Coordinates=p)
   return cache[p]
  def shell(data):
   ring=data['outer'];base=data['base'];top=base+data['height'];loops=[[(*p,base)for p in reversed(ring)],[(*p,top)for p in ring]]
   for a,b in zip(ring,ring[1:]+ring[:1]):loops.append([(*a,base),(*b,base),(*b,top),(*a,top)])
   faces=[]
   for loop in loops:
    if data.get('reverse'):loop=list(reversed(loop))
    poly=f.create_entity('IfcPolyLoop',Polygon=tuple(point(p)for p in loop));bound=f.create_entity('IfcFaceOuterBound',Bound=poly,Orientation=True);faces.append(f.create_entity('IfcFace',Bounds=(bound,)))
   return f.create_entity('IfcClosedShell',CfsFaces=tuple(faces))
  geometry=base['geometry'];outer=shell(geometry);voids=[]
  if row['change']=='voids':voids=[shell(v)for v in row['voids']]
  if row['change']=='open':outer.CfsFaces=outer.CfsFaces[:-1]
  if row['change']=='duplicate':outer.CfsFaces=outer.CfsFaces+(outer.CfsFaces[0],)
  if row['change']=='join':
   dx,dy,dz=row['offset'];other=shell(dict(outer=[[x+dx,y+dy]for x,y in geometry['outer']],base=geometry['base']+dz,height=geometry['height']));outer.CfsFaces+=other.CfsFaces
  solid=f.create_entity('IfcFacetedBrepWithVoids',Outer=outer,Voids=tuple(voids))if voids else f.create_entity('IfcFacetedBrep',Outer=outer)
  path=out/(row['id']+'.ifc');path.write_text(f.to_string(),encoding='utf-8');logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True)
  result=dict(id=row['id'],sha256=hashlib.sha256(path.read_bytes()).hexdigest(),header=list(f.header.file_schema.schema_identifiers),solid=str(solid.id()),diagnostics=logger.statements)
  try:
   shape=ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(),solid);result['normalGeometry']=dict(vertices=len(shape.verts)//3,triangles=len(shape.faces)//3)
  except Exception as error:result['normalGeometryError']=str(error)
  return result
base=json.loads((root/'faceted-brep-geometry-vectors.json').read_text())['cases'][0]
cases=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/faceted-shell-cases.json');rows=json.loads(cases.read_text())['cases'];out=root/'🗑️generated/faceted-shell-independent';out.mkdir(exist_ok=True);oracle=IfcOpenShellShellOracle();results=[]
for row in rows:
 result=oracle.observe(row);results.append(result);(out/'oracle.json').write_text(json.dumps(dict(version='IfcOpenShell0.9.0.post1',casesSha256=hashlib.sha256(cases.read_bytes()).hexdigest(),rows=results),indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(dict(id=row['id'],diagnostics=len(result['diagnostics']),geometry=result.get('normalGeometry'),geometryError=result.get('normalGeometryError'))),flush=True)
