"""🧭 Census unchanged official geometry frontier through a test-only backend."""
import ctypes,hashlib,json,sys
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell
folder=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official');rows=[]
for path in sorted(folder.glob('*.ifc')):
 f=ifcopenshell.open(str(path));items=[]
 for rep in f.by_type('IfcShapeRepresentation'):
  for item in rep.Items:
   row=dict(representation=rep.id(),identifier=rep.RepresentationIdentifier,type=rep.RepresentationType,item=item.id(),kind=item.is_a())
   if item.is_a('IfcSweptAreaSolid'):row['profile']=dict(id=item.SweptArea.id(),kind=item.SweptArea.is_a())
   items.append(row)
 placements=[dict(id=e.id(),kind=e.is_a())for e in f.by_type('IfcObjectPlacement')if not e.is_a('IfcLocalPlacement')]
 if items or placements:rows.append(dict(file=path.name,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),header=list(f.header.file_schema.schema_identifiers),items=items,placements=placements))
(p/'🗑️generated/geometry-frontier-census.json').write_text(json.dumps(dict(version=ifcopenshell.version,files=rows),indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+str(len(rows))+' official files observed')
for row in rows:
 interesting=[i for i in row['items']if i['kind']not in ['IfcTriangulatedFaceSet','IfcPolygonalFaceSet','IfcFacetedBrep','IfcFacetedBrepWithVoids','IfcMappedItem','IfcExtrudedAreaSolid']or i.get('profile',{}).get('kind','')not in ['', 'IfcRectangleProfileDef','IfcCircleProfileDef','IfcArbitraryClosedProfileDef','IfcArbitraryProfileDefWithVoids']]
 if interesting or row['placements']:print('[DEBUG] '+json.dumps(dict(file=row['file'],items=interesting,placements=row['placements'])))
