"""🕳️ Distinguish schema-valid inward shell source from normal backend meshing observations."""
import importlib.util,json
from pathlib import Path
p=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('faceted',p/'faceted-brep-vectors-oracle.py')
# Reuse only dependency setup; importing the executable builder would rerun its matrix.
spec=importlib.util.spec_from_file_location('surface',p/'body-surface-model-oracle.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
import ifcopenshell,ifcopenshell.geom
path=p/'🗑️generated/faceted-brep-vectors/with-voids.ifc';f=ifcopenshell.open(str(path));solid=f.by_type('IfcFacetedBrepWithVoids')[0];settings=ifcopenshell.geom.settings();rows=[];shells=[]
for shell in[solid.Outer,*solid.Voids]:
 polygons=[]
 for face in shell.CfsFaces:
  assert len(face.Bounds)==1
  bound=face.Bounds[0];coords=[list(v.Coordinates)for v in bound.Bound.Polygon]
  if not bound.Orientation:coords.reverse()
  polygons.append(coords)
 shells.append(dict(id=str(shell.id()),faces=polygons))
 for target in[shell]:
  try:
   mesh=ifcopenshell.geom.create_shape(settings,target);rows.append(dict(target=str(target.id()),type=target.is_a(),vertices=len(mesh.verts)//3,triangles=len(mesh.faces)//3))
  except Exception as e:rows.append(dict(target=str(target.id()),type=target.is_a(),error=str(e)))
for label,target in[('solid',solid)]:
 try:
  mesh=ifcopenshell.geom.create_shape(settings,target);rows.append(dict(target=label,vertices=len(mesh.verts)//3,triangles=len(mesh.faces)//3))
 except Exception as e:rows.append(dict(target=label,error=str(e)))
log=ifcopenshell.get_log()if hasattr(ifcopenshell,'get_log')else'unavailable'
(p/'🗑️generated/faceted-brep-vectors/void-shell-evidence.json').write_text(json.dumps(dict(version=ifcopenshell.version,shells=shells,normalBackend=rows,log=log),indent=2)+'\n');print('[DEBUG] '+json.dumps(rows))

