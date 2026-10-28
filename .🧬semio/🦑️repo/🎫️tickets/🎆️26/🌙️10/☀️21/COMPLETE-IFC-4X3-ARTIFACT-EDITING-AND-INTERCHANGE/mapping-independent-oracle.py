"""🗺️ Independently observe actual mapped-item composition on authored ADD2 products."""
import ctypes,sys,json,math,hashlib
from pathlib import Path
root=Path(__file__).parent;dep=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
class MappingOracle:
 def observe(self,row):raise NotImplementedError
class IfcOpenShellMappingOracle(MappingOracle):
 def observe(self,row):
  import ifcopenshell,ifcopenshell.geom,ifcopenshell.validate,ifcopenshell.util.placement
  import numpy as np
  f=ifcopenshell.file(schema='IFC4X3_ADD2');point=lambda p:f.create_entity('IfcCartesianPoint',Coordinates=tuple(float(x)for x in p));direction=lambda v:f.create_entity('IfcDirection',DirectionRatios=tuple(float(x)for x in v))
  axis=lambda p,a:f.create_entity('IfcAxis2Placement3D',Location=point(p),Axis=direction([0,0,1]),RefDirection=direction([math.cos(a),math.sin(a),0]))
  world=axis([0,0,0],0);ctx=f.create_entity('IfcGeometricRepresentationContext',ContextType='Model',CoordinateSpaceDimension=3,Precision=1e-8,WorldCoordinateSystem=world);unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE');units=f.create_entity('IfcUnitAssignment',Units=(unit,));f.create_entity('IfcProject',GlobalId=ifcopenshell.guid.new(),Name='Mapping oracle',RepresentationContexts=(ctx,),UnitsInContext=units)
  profile=f.create_entity('IfcRectangleProfileDef',ProfileType='AREA',XDim=2.,YDim=1.);solid=f.create_entity('IfcExtrudedAreaSolid',SweptArea=profile,Position=world,ExtrudedDirection=direction([0,0,1]),Depth=3.);rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=ctx,RepresentationIdentifier='Body',RepresentationType='SweptSolid',Items=(solid,));origin=axis(row['origin'],row['originRotation']);mapping=f.create_entity('IfcRepresentationMap',MappingOrigin=origin,MappedRepresentation=rep)
  kwargs=dict(LocalOrigin=point(row['target']),Scale=row['scale'])
  for i,v in enumerate(row['axes'],1):kwargs['Axis'+str(i)]=direction(v)if v is not None else None
  if 'scale2'in row:kwargs.update(Scale2=row['scale2'],Scale3=row['scale3'])
  op=f.create_entity('IfcCartesianTransformationOperator3DnonUniform'if 'scale2'in row else'IfcCartesianTransformationOperator3D',**kwargs);mapped=f.create_entity('IfcMappedItem',MappingSource=mapping,MappingTarget=op);mappedrep=f.create_entity('IfcShapeRepresentation',ContextOfItems=ctx,RepresentationIdentifier='Body',RepresentationType='MappedRepresentation',Items=(mapped,));shape=f.create_entity('IfcProductDefinitionShape',Representations=(mappedrep,));product=f.create_entity('IfcBuildingElementProxy',GlobalId=ifcopenshell.guid.new(),Name=row['id'],ObjectPlacement=f.create_entity('IfcLocalPlacement',RelativePlacement=world),Representation=shape)
  logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);mesh=ifcopenshell.geom.create_shape(settings,product).geometry;coords=list(mesh.verts);points=[coords[i:i+3]for i in range(0,len(coords),3)]
  target=ifcopenshell.util.placement.get_cartesiantransformationoperator3d(op);source=ifcopenshell.util.placement.get_axis2placement(origin);corners=[np.array([x,y,z,1.])for x in [-1.,1.]for y in [-.5,.5]for z in [0.,3.]]
  bounds=lambda ps:[[float(min(p[k]for p in ps))for k in range(3)],[float(max(p[k]for p in ps))for k in range(3)]]
  actual=bounds(points);direct=bounds([target@source@p for p in corners]);inverse=bounds([target@np.linalg.inv(source)@p for p in corners]);near=lambda a,b:max(abs(x-y)for side,other in zip(a,b)for x,y in zip(side,other))<1e-9
  file=out/(row['id']+'.ifc');file.write_text(f.to_string(),encoding='utf-8');return dict(**row,header=list(f.header.file_schema.schema_identifiers),fileSha256=hashlib.sha256(file.read_bytes()).hexdigest(),mappedId=str(mapped.id()),diagnostics=logger.statements,actualBounds=actual,directBounds=direct,inverseBounds=inverse,directMatches=near(actual,direct),inverseMatches=near(actual,inverse),actualMatrix=ifcopenshell.util.placement.get_mappeditem_transformation(mapped).tolist())
rows=[dict(id='nonzero-origin-rotation',origin=[2,3,4],originRotation=.4,target=[10,20,30],scale=1.,axes=[None,None,None]),dict(id='target-rotation',origin=[2,3,4],originRotation=.4,target=[10,20,30],scale=1.,axes=[[0,1,0],[-1,0,0],[0,0,1]]),dict(id='reflection',origin=[2,3,4],originRotation=.4,target=[10,20,30],scale=1.,axes=[[1,0,0],[0,-1,0],[0,0,1]]),dict(id='nonuniform',origin=[2,3,4],originRotation=.4,target=[10,20,30],scale=2.,scale2=3.,scale3=4.,axes=[None,None,None]),dict(id='axis1-only',origin=[2,3,4],originRotation=.4,target=[10,20,30],scale=1.,axes=[[0,2,0],None,None]),dict(id='axis3-only',origin=[2,3,4],originRotation=.4,target=[10,20,30],scale=1.,axes=[None,None,[2,0,0]])]
out=root/'🗑️generated/mapping-independent';out.mkdir(exist_ok=True);oracle=IfcOpenShellMappingOracle();results=[]
for row in rows:
 result=oracle.observe(row);results.append(result);(out/'oracle.json').write_text(json.dumps(dict(version='IfcOpenShell0.9.0.post1',rows=results),indent=2,default=str)+'\n');print('[DEBUG] '+json.dumps(dict(id=row['id'],direct=result['directMatches'],inverse=result['inverseMatches'],diagnostics=len(result['diagnostics']),bounds=result['actualBounds'])),flush=True)
