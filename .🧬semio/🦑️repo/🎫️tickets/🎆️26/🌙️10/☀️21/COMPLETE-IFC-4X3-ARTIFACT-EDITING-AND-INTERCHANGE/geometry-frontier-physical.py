"""🧭 Measure three unchanged official next-frontier geometries."""
import ast,json,math,hashlib
from pathlib import Path
p=Path(__file__).parent;tree=ast.parse((p/'geometry-frontier-census.py').read_text(encoding='utf-8-sig'));cut=next(i for i,n in enumerate(tree.body)if isinstance(n,ast.Assign)and any(isinstance(t,ast.Name)and t.id=='folder'for t in n.targets));ns={'__file__':str(p/'geometry-frontier-census.py')};exec(compile(ast.Module(body=tree.body[:cut],type_ignores=[]),'owned-interface','exec'),ns)
import ifcopenshell,ifcopenshell.geom,ifcopenshell.validate
folder=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official');rows=[]
for name in ['csg-primitive.ifc','surface-model.ifc','column-extruded-solid.ifc']:
 path=folder/name;f=ifcopenshell.open(str(path));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);r=dict(file=name,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),header=list(f.header.file_schema.schema_identifiers),diagnostics=log.statements,products=[])
 for product in f.by_type('IfcProduct'):
  if not product.Representation:continue
  try:
   settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);owner=ifcopenshell.geom.create_shape(settings,product);mesh=owner.geometry;v=list(mesh.verts);idx=list(mesh.faces);pts=[v[i:i+3]for i in range(0,len(v),3)];vol=math.fsum((pts[idx[i]][0]*(pts[idx[i+1]][1]*pts[idx[i+2]][2]-pts[idx[i+1]][2]*pts[idx[i+2]][1])+pts[idx[i]][1]*(pts[idx[i+1]][2]*pts[idx[i+2]][0]-pts[idx[i+1]][0]*pts[idx[i+2]][2])+pts[idx[i]][2]*(pts[idx[i+1]][0]*pts[idx[i+2]][1]-pts[idx[i+1]][1]*pts[idx[i+2]][0]))/6 for i in range(0,len(idx),3));r['products'].append(dict(id=product.id(),vertices=len(pts),triangles=len(idx)//3,bounds=[[min(v[k]for v in pts)for k in range(3)],[max(v[k]for v in pts)for k in range(3)]],orientedVolume=vol))
  except Exception as e:r['products'].append(dict(id=product.id(),error=str(e)))
 rows.append(r);print('[DEBUG] '+json.dumps(dict(file=name,errors=len(log.statements),products=r['products'])),flush=True)
(p/'🗑️generated/geometry-frontier-physical.json').write_text(json.dumps(dict(version=ifcopenshell.version,rows=rows),indent=2,default=str)+'\n',encoding='utf-8')
