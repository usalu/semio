from pathlib import Path
import importlib,sys,json
ROOT=next(p for p in Path(__file__).resolve().parents if(p/'🧰️framework').is_dir());sys.path.insert(0,str(ROOT));M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')
def inside(point,polygon):
 x,z=point;hit=False
 for a,b in zip(polygon,polygon[1:]+polygon[:1]):
  if(a[1]>z)!=(b[1]>z) and x<(b[0]-a[0])*(z-a[1])/(b[1]-a[1])+a[0]:hit=not hit
 return hit
parts={p.key:p for p in M.capsule_assembly('A','full','Backslash').parts};fixtures=M._reference_interior(M.capsule_type('A'),'Backslash')
corners=[p for p in parts.values()if p.role=='lining'and'corner'in p.key];findings=[]
for fixture in fixtures:
 found=[]
 for p in corners:
  n=len(p.solid.vertices)//2;poly=[(q[0],q[2])for q in p.solid.vertices[:n]];yy=[q[1]for q in p.solid.vertices]
  for v in fixture.solid.vertices:
   if min(yy)+1e-7<v[1]<max(yy)-1e-7 and inside((v[0],v[2]),poly):found.append({'lining':p.key,'fixture_vertex':v});break
 if found:findings.append({'fixture':fixture.key,'hits':found})
# Points strictly inside fixture boxes, to distinguish a material penetration from a shared vertex/contact.
probes=[('wetroom-floor',(-1.119,.50,.201)),('bed-front-rail',(-1.119,2.82,.201)),('media-wall',(1.1099,1.50,.2001))]
probe_results=[]
for key,v in probes:
 p=parts[key];lo=[min(q[k]for q in p.solid.vertices)for k in range(3)];hi=[max(q[k]for q in p.solid.vertices)for k in range(3)];hits=[]
 for c in corners:
  n=len(c.solid.vertices)//2;poly=[(q[0],q[2])for q in c.solid.vertices[:n]];ys=[q[1]for q in c.solid.vertices]
  if min(ys)<v[1]<max(ys)and inside((v[0],v[2]),poly):hits.append(c.key)
 probe_results.append({'fixture':key,'point':v,'strictly_inside_fixture_box':all(lo[k]<v[k]<hi[k]for k in range(3)),'actual_lining_cap_polygon_contains':hits})
OUT=Path(__file__).resolve().parent/'🗑️generated';(OUT/'lining-fixture-intersections.json').write_text(json.dumps({'method':'ray crossing against actual extruded lining cap polygons, not analytic AABB','vertex_findings':findings,'strict_probes':probe_results},indent=2))
print('[DEBUG]',json.dumps({'vertex_findings':findings,'strict_probes':probe_results}),flush=True)
