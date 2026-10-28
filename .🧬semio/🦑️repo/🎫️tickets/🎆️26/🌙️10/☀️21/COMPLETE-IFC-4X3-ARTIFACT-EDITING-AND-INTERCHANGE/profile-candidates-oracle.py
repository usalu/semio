"""🧊 Independent census/mesh measurements of pinned original profile specimens."""
import ctypes,hashlib,json,sys
from pathlib import Path
ticket=Path(__file__).resolve().parent;dep=str(ticket/'🗑️generated'/'oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom
repo=ticket
while not(repo/'✏️s').exists():repo=repo.parent
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official';manifest=json.loads((base/'cases.json').read_text(encoding='utf-8-sig'));rows=[]
for case in manifest['cases']:
 path=base/case['file'];text=path.read_text(encoding='utf-8-sig')
 if not any(x in text.upper() for x in ['IFCARBITRARYCLOSEDPROFILEDEF','IFCARBITRARYPROFILEDEFWITHVOIDS','IFCCIRCLEPROFILEDEF']):continue
 digest=hashlib.sha256(path.read_bytes()).hexdigest();assert digest==case['sha256'];f=ifcopenshell.open(str(path));profiles=[]
 for p in f.by_type('IfcProfileDef'):
  if not(p.is_a('IfcArbitraryClosedProfileDef') or p.is_a('IfcCircleProfileDef')):continue
  curves=[]
  for curve in ([p.OuterCurve] if hasattr(p,'OuterCurve') else [])+list(getattr(p,'InnerCurves',()) or ()):
   curves.append({'id':curve.id(),'type':curve.is_a(),'points':[list(v.Coordinates) for v in curve.Points] if curve.is_a('IfcPolyline') else None,'closedBySamePoint':curve.Points[0].id()==curve.Points[-1].id() if curve.is_a('IfcPolyline') else None})
  solids=f.get_inverse(p);profiles.append({'id':p.id(),'type':p.is_a(),'profileType':p.ProfileType,'radius':getattr(p,'Radius',None),'curves':curves,'users':[{'id':s.id(),'type':s.is_a()} for s in solids]})
 products=[];target_ids={p['id'] for p in profiles}
 for product in f.by_type('IfcProduct'):
  if not product.Representation:continue
  if not target_ids.intersection(e.id() for e in f.traverse(product.Representation)):continue
  settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True)
  try:
   shape=ifcopenshell.geom.create_shape(settings,product);v=list(shape.geometry.verts);indices=list(shape.geometry.faces);points=[v[i:i+3] for i in range(0,len(v),3)];volume=0.
   for i in range(0,len(indices),3):
    a,b,c=(points[indices[i+j]] for j in range(3));volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6
   products.append({'id':product.id(),'type':product.is_a(),'vertices':len(points),'triangles':len(indices)//3,'bounds':[[min(p[k] for p in points) for k in range(3)],[max(p[k] for p in points) for k in range(3)]],'signedVolume':volume,'volume':abs(volume)})
  except Exception as error:products.append({'id':product.id(),'error':str(error)})
 rows.append({'file':case['file'],'sha256':digest,'url':case['url'],'header':list(f.header.file_schema.schema_identifiers),'profiles':profiles,'products':products})
out=ticket/'🗑️generated'/'profile-candidates';out.mkdir(exist_ok=True);(out/'oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell.geom','version':ifcopenshell.version,'commit':manifest['commit'],'rows':rows},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps([{'file':r['file'],'profiles':len(r['profiles']),'products':len(r['products']),'errors':sum('error'in p for p in r['products'])} for r in rows]))
