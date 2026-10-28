"""📦 Independent constructed ADD2 rectangular extrusion geometry vectors."""
import ctypes,json,math,sys
from pathlib import Path
root=Path(__file__).resolve().parent;dep=str(root/'🗑️generated'/'oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,short,len(short));dep=short.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.geom,ifcopenshell.guid
vectors=json.loads((root/'rectangle-extrusion-vectors.json').read_text(encoding='utf-8-sig'));out=root/'🗑️generated'/'rectangle-extrusion';out.mkdir(exist_ok=True);rows=[]
def volume(points,faces):
 total=0.
 for at in range(0,len(faces),3):
  a,b,c=(points[faces[at+i]] for i in range(3));total+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6
 return total
for row in vectors['cases']:
 f=ifcopenshell.file(schema='IFC4X3_ADD2')
 def point(coords):return f.create_entity('IfcCartesianPoint',Coordinates=tuple(float(x) for x in coords))
 def direction(coords):return f.create_entity('IfcDirection',DirectionRatios=tuple(float(x) for x in coords))
 identity=f.create_entity('IfcAxis2Placement3D',Location=point([0,0,0]));context=f.create_entity('IfcGeometricRepresentationContext',ContextType='Model',CoordinateSpaceDimension=3,Precision=1.e-8,WorldCoordinateSystem=identity)
 unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE',Prefix='MILLI' if row.get('millimetres') else None);units=f.create_entity('IfcUnitAssignment',Units=(unit,));f.create_entity('IfcProject',GlobalId=ifcopenshell.guid.compress('%032x'%(len(list(f))+1)),RepresentationContexts=(context,),UnitsInContext=units)
 profile_position=None
 if 'profileOrigin'in row:profile_position=f.create_entity('IfcAxis2Placement2D',Location=point(row['profileOrigin']),RefDirection=direction(row['profileX']))
 profile=f.create_entity('IfcRectangleProfileDef',ProfileType=row.get('profileType','AREA'),Position=profile_position,XDim=float(row['x']),YDim=float(row['y']))
 solid_position=None
 if 'solidOrigin'in row:solid_position=f.create_entity('IfcAxis2Placement3D',Location=point(row['solidOrigin']),Axis=direction(row['solidZ']),RefDirection=direction(row['solidX']))
 solid=f.create_entity('IfcExtrudedAreaSolid',SweptArea=profile,Position=solid_position,ExtrudedDirection=direction(row['direction']),Depth=float(row['depth']));shape=f.create_entity('IfcShapeRepresentation',ContextOfItems=context,RepresentationIdentifier='Body',RepresentationType='SweptSolid',Items=(solid,));definition=f.create_entity('IfcProductDefinitionShape',Representations=(shape,));placement=f.create_entity('IfcLocalPlacement',RelativePlacement=identity);product=f.create_entity('IfcBuildingElementProxy',GlobalId=ifcopenshell.guid.compress('%032x'%(len(list(f))+1)),ObjectPlacement=placement,Representation=definition)
 file=out/(row['id']+'.ifc');f.write(str(file));result={'id':row['id'],'expected':row['expected'],'product':product.id(),'solid':solid.id(),'profile':profile.id(),'file':file.name,'header':list(f.header.file_schema.schema_identifiers)}
 settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True)
 try:
  shape_owner=ifcopenshell.geom.create_shape(settings,product);mesh=shape_owner.geometry;verts=list(mesh.verts);faces=list(mesh.faces);points=[verts[i:i+3] for i in range(0,len(verts),3)];signed=volume(points,faces);result.update(vertices=len(points),triangles=len(faces)//3,bounds=[[min(p[k] for p in points) for k in range(3)],[max(p[k] for p in points) for k in range(3)]],signedVolume=signed,volume=abs(signed))
  if row['expected']=='supported':
   d=row['direction'];scale=.001 if row.get('millimetres') else 1.;analytic=row['x']*row['y']*row['depth']*abs(d[2])/math.sqrt(sum(v*v for v in d))*scale**3;result['analyticVolume']=analytic;result['volumeMatches']=math.isclose(abs(signed),analytic,rel_tol=1e-9,abs_tol=1e-9);assert result['volumeMatches'] and len(faces)==36
 except Exception as error:
  result['oracleError']=str(error)
  if row['expected']=='supported':raise
 rows.append(result)
(out/'oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell.geom','version':ifcopenshell.version,'rows':rows,'invalidAcceptanceIsNotNormative':True,'nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'cases':len(rows),'supported':sum(r['expected']=='supported'for r in rows),'volumeMatches':sum(r.get('volumeMatches',False)for r in rows)}))


