"""🧩 Pin released concept inputs and independently traverse actual IFC class membership."""
import ctypes,json,sys,hashlib,urllib.request,re
from pathlib import Path
root=Path(__file__).resolve().parent;path=str(root/'🗑️generated'/'oracle');
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(path,short,len(short));path=short.value or path
sys.path.insert(0,path)
import ifcopenshell
base='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/'
relative=['concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Tessellation_Geometry/content.html','lexical/IfcWall.htm','lexical/IfcTessellatedFaceSet.htm']
out=root/'🗑️generated'/'body-tessellation';out.mkdir(exist_ok=True)
sources=[]
for i,part in enumerate(relative):
 raw=urllib.request.urlopen(base+part).read();(out/(str(i)+'.html')).write_bytes(raw);text=raw.decode('utf8');sources.append({'url':base+part,'sha256':hashlib.sha256(raw).hexdigest(),'executableLinks':re.findall(r'href=[\"\']([^\"\']*(?:mvdxml|\.xml)[^\"\']*)',text),'bodyTessellationMention': 'Body Tessellation Geometry' in text})
contract=json.loads((root/'body-tessellation-cases.json').read_text(encoding='utf-8-sig'));f=ifcopenshell.file(schema='IFC4X3_ADD2');context=f.create_entity('IfcGeometricRepresentationContext',ContextIdentifier='Model',ContextType='Model',CoordinateSpaceDimension=3,Precision=1e-5,WorldCoordinateSystem=f.create_entity('IfcAxis2Placement3D',Location=f.create_entity('IfcCartesianPoint',Coordinates=(0.,0.,0.))))
points=f.create_entity('IfcCartesianPointList3D',CoordList=((0.,0.,0.),(1.,0.,0.),(0.,1.,0.)))
items={'IfcTriangulatedFaceSet':f.create_entity('IfcTriangulatedFaceSet',Coordinates=points,Closed=False,CoordIndex=((1,2,3),)),'IfcPolygonalFaceSet':f.create_entity('IfcPolygonalFaceSet',Coordinates=points,Closed=False,Faces=(f.create_entity('IfcIndexedPolygonalFace',CoordIndex=(1,2,3)),)),'IfcPolyline':f.create_entity('IfcPolyline',Points=(f.create_entity('IfcCartesianPoint',Coordinates=(0.,0.,0.)),f.create_entity('IfcCartesianPoint',Coordinates=(1.,0.,0.))))}
rows=[]
for row in contract['cases']:
 representation=f.create_entity('IfcShapeRepresentation',ContextOfItems=context,RepresentationIdentifier=row['identifier'],RepresentationType=row['type'],Items=tuple(items[name] for name in row['items']))
 actual='not-applicable' if row['applicable'] is False else 'incomplete' if row['applicable'] is None else 'pass' if representation.RepresentationIdentifier=='Body' and representation.RepresentationType=='Tessellation' and all(item.is_a('IfcTessellatedFaceSet') for item in representation.Items) else 'fail'
 rows.append({'id':row['id'],'representationId':representation.id(),'actual':actual,'expected':row['expected'],'match':actual==row['expected'],'memberTypes':[item.is_a() for item in representation.Items],'membership':[item.is_a('IfcTessellatedFaceSet') for item in representation.Items]})
f.write(str(out/'graphs.ifc'));result={'oracle':'IfcOpenShell '+ifcopenshell.version,'sources':sources,'rows':rows,'nativeExecution':False,'fullMvdConformance':False};(out/'oracle-v2.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'rows':len(rows),'matches':sum(row['match'] for row in rows),'sources':sources}));assert all(row['match'] for row in rows)

