"""📚 Record exact released owner-qualified slots for next geometry candidates."""
import ast,json
from pathlib import Path
p=Path(__file__).parent;tree=ast.parse((p/'geometry-frontier-census.py').read_text(encoding='utf-8-sig'));cut=next(i for i,n in enumerate(tree.body)if isinstance(n,ast.Assign)and any(isinstance(t,ast.Name)and t.id=='folder'for t in n.targets));ns={'__file__':str(p/'geometry-frontier-census.py')};exec(compile(ast.Module(body=tree.body[:cut],type_ignores=[]),'owned-interface','exec'),ns)
import ifcopenshell.ifcopenshell_wrapper as w
schema=w.schema_by_name('IFC4X3_ADD2');rows=[]
for name in ['IfcCsgSolid','IfcCsgPrimitive3D','IfcBlock','IfcFaceBasedSurfaceModel','IfcConnectedFaceSet','IfcIShapeProfileDef','IfcSweptAreaSolid','IfcExtrudedAreaSolid','IfcBooleanResult','IfcBooleanClippingResult','IfcHalfSpaceSolid','IfcLinearPlacement']:
 d=schema.declaration_by_name(name);owners={};line=d
 while line:
  for a in line.attributes():owners[a.name()]=line.name()
  line=line.supertype()
 rows.append(dict(name=name,attributes=[dict(index=i,name=a.name(),owner=owners[a.name()],optional=a.optional(),type=str(a.type_of_attribute()))for i,a in enumerate(d.all_attributes())]))
(p/'🗑️generated/geometry-frontier-slots.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(rows[:3]))
