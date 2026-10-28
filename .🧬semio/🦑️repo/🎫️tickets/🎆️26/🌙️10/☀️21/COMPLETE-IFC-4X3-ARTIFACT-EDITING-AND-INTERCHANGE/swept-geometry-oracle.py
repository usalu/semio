"""🧊 Independent official swept/mapped mesh measurement through normal IfcOpenShell geometry API."""
import ctypes,hashlib,json,sys
from pathlib import Path
root=Path(__file__).resolve().parent;dep=str(root/'🗑️generated'/'oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,short,len(short));dep=short.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom
repo=root
while not(repo/'✏️s').exists():repo=repo.parent
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official';manifest=json.loads((base/'cases.json').read_text(encoding='utf-8-sig'));wanted=['extruded-solid.ifc','mapped-shape-without-transformation.ifc','mapped-shape-with-transformation.ifc','mapped-shape-with-multiple-items.ifc'];rows=[]
for name in wanted:
 case=next(c for c in manifest['cases'] if c['file']==name);path=base/name;digest=hashlib.sha256(path.read_bytes()).hexdigest();assert digest==case['sha256'];f=ifcopenshell.open(str(path));products=[]
 for product in f.by_type('IfcProduct'):
  if not product.Representation:continue
  settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True)
  try:
   shape=ifcopenshell.geom.create_shape(settings,product);vertices=list(shape.geometry.verts);faces=list(shape.geometry.faces);points=[vertices[i:i+3] for i in range(0,len(vertices),3)];volume=0.
   for i in range(0,len(faces),3):
    a,b,c=(points[faces[i+j]] for j in range(3));volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6.
   products.append({'id':product.id(),'type':product.is_a(),'vertices':len(points),'triangles':len(faces)//3,'bounds':[[min(p[k] for p in points) for k in range(3)],[max(p[k] for p in points) for k in range(3)]],'signedVolume':volume,'volume':abs(volume),'worldCoordinates':True,'units':'metres-normal-geometry-API'})
  except Exception as error:products.append({'id':product.id(),'error':str(error)})
 rows.append({'file':name,'sha256':digest,'url':case['url'],'header':list(f.header.file_schema.schema_identifiers),'products':products})
out=root/'🗑️generated'/'swept-geometry';out.mkdir(exist_ok=True);(out/'oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell.geom','version':ifcopenshell.version,'commit':manifest['commit'],'rows':rows,'nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'files':len(rows),'products':sum(len(r['products']) for r in rows),'errors':sum('error'in p for r in rows for p in r['products'])}))
