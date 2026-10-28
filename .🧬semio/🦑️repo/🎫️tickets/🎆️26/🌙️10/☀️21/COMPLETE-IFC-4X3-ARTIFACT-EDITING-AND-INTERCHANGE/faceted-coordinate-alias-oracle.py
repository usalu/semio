"""🧱 Observe distinct exact-coordinate labels on genuine faceted shell roles."""
import ctypes,json,sys,hashlib
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom,ifcopenshell.validate
source=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/brep-model.ifc');f=ifcopenshell.open(str(source));solid=f.by_id(1021);roster=[]
for face in solid.Outer.CfsFaces:
 for bound in face.Bounds:
  old=bound.Bound.Polygon;new=tuple(f.create_entity('IfcCartesianPoint',Coordinates=point.Coordinates)for point in old);bound.Bound.Polygon=new;roster.append(dict(face=face.id(),original=[point.id()for point in old],replacement=[point.id()for point in new]))
out=p/'🗑️generated/faceted-coordinate-alias';out.mkdir(exist_ok=True);target=out/'distinct-labels.ifc';f.write(str(target));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);mesh=ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(),solid);result=dict(version=ifcopenshell.version,sourceHash=hashlib.sha256(source.read_bytes()).hexdigest(),authoredHash=hashlib.sha256(target.read_bytes()).hexdigest(),header=list(f.header.file_schema.schema_identifiers),roster=roster,diagnostics=log.statements,mesh=dict(vertices=len(mesh.verts)//3,triangles=len(mesh.faces)//3));(out/'oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(dict(errors=len(log.statements),mesh=result['mesh'])))
