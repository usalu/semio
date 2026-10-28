"""📏️ Measure actual emitted mapped-product mesh volume and repeated-source outputs."""
import ctypes,sys,json,math
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom
source=json.loads((p/'mapping-neutral-cases.json').read_text());rows=[]
for row in source['cases']:
 expected=6
 for layer in row['layers']:
  s=1 if layer['scale']is None else layer['scale'];expected*=s*(s if layer['secondary'][0]is None else layer['secondary'][0])*(s if layer['secondary'][1]is None else layer['secondary'][1])
 row['expectedVolume']=expected;f=ifcopenshell.open(str(p/'🗑️generated/mapping-neutral'/(row['id']+'.ifc')));products=[]
 for product in f.by_type('IfcBuildingElementProxy'):
  settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);owner=ifcopenshell.geom.create_shape(settings,product);v=list(owner.geometry.verts);ids=list(owner.geometry.faces);points=[v[i:i+3]for i in range(0,len(v),3)];terms=[]
  for i in range(0,len(ids),3):
   a,b,c=[points[ids[i+j]]for j in range(3)];terms.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
  volume=math.fsum(terms);assert math.isclose(abs(volume),expected,rel_tol=1e-9,abs_tol=1e-9);products.append(dict(id=str(product.id()),signedVolume=volume,volume=abs(volume),triangles=len(ids)//3))
 assert len(products)==(2 if row['repeatedSource']else 1);rows.append(dict(id=row['id'],expectedVolume=expected,products=products))
(p/'mapping-neutral-cases.json').write_text(json.dumps(source,indent=2)+'\n',encoding='utf-8');(p/'🗑️generated/mapping-neutral/volumes.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf-8');print('[DEBUG] eight mappings/nine actual products mesh volumes agree')
