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
  profile=f.create_entity('IfcRectangleProfileDef',ProfileType='AREA',XDim=2.,YDim=1.);solid=f.create_entity('IfcExtrudedAreaSolid',SweptArea=profile,Position=world,ExtrudedDirection=direction([0,0,1]),Depth=3.);rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=ctx,RepresentationIdentifier='Body',RepresentationType='SweptSolid',Items=(solid,))
  total=np.identity(4);inverseTotal=np.identity(4);ids=[]
  for data in row['layers']:
   origin=axis(data['origin'],data['originRotation']);mapping=f.create_entity('IfcRepresentationMap',MappingOrigin=origin,MappedRepresentation=rep);kwargs=dict(LocalOrigin=point(data['target']),Scale=data['scale'])
   for i,v in enumerate(data['axes'],1):kwargs['Axis'+str(i)]=direction(v)if v is not None else None
   secondary=data['secondary'];nonuniform=any(v is not None for v in secondary)
   if nonuniform:kwargs.update(Scale2=secondary[0],Scale3=secondary[1])
   op=f.create_entity('IfcCartesianTransformationOperator3DnonUniform'if nonuniform else'IfcCartesianTransformationOperator3D',**kwargs);mapped=f.create_entity('IfcMappedItem',MappingSource=mapping,MappingTarget=op);ids.append(str(mapped.id()));rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=ctx,RepresentationIdentifier='Body',RepresentationType='MappedRepresentation',Items=(mapped,));target=ifcopenshell.util.placement.get_cartesiantransformationoperator3d(op);source=ifcopenshell.util.placement.get_axis2placement(origin);total=target@source@total;inverseTotal=target@np.linalg.inv(source)@inverseTotal
  shape=f.create_entity('IfcProductDefinitionShape',Representations=(rep,));products=[]
  for _ in range(2 if row['repeatedSource']else 1):products.append(f.create_entity('IfcBuildingElementProxy',GlobalId=ifcopenshell.guid.new(),Name=row['id'],ObjectPlacement=f.create_entity('IfcLocalPlacement',RelativePlacement=world),Representation=shape))
  product=products[0]
  logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);owner=ifcopenshell.geom.create_shape(settings,product);mesh=owner.geometry;coords=list(mesh.verts);points=[coords[i:i+3]for i in range(0,len(coords),3)]
  corners=[np.array([x,y,z,1.])for x in [-1.,1.]for y in [-.5,.5]for z in [0.,3.]]
  bounds=lambda ps:[[float(min(p[k]for p in ps))for k in range(3)],[float(max(p[k]for p in ps))for k in range(3)]]
  actual=bounds(points);direct=bounds([total@p for p in corners]);inverse=bounds([inverseTotal@p for p in corners]);near=lambda a,b:max(abs(x-y)for side,other in zip(a,b)for x,y in zip(side,other))<1e-9
  file=out/(row['id']+'.ifc');file.write_text(f.to_string(),encoding='utf-8');return dict(**row,header=list(f.header.file_schema.schema_identifiers),fileSha256=hashlib.sha256(file.read_bytes()).hexdigest(),mappedIds=ids,productIds=[str(p.id())for p in products],diagnostics=logger.statements,actualBounds=actual,directBounds=direct,inverseBounds=inverse,directMatches=near(actual,direct),inverseMatches=near(actual,inverse),actualMatrix=total.tolist())
data=json.loads((root/'mapping-neutral-cases.json').read_text(encoding="utf-8-sig"));rows=[row for row in data['cases']if len(sys.argv)==1 or row['id']==sys.argv[1]];out=root/'🗑️generated/mapping-neutral';out.mkdir(exist_ok=True);oracle=IfcOpenShellMappingOracle();results=[]
for row in rows:
 result=oracle.observe(row);assert result['directMatches'] and not result['diagnostics'];results.append(result);(out/('oracle.json'if len(sys.argv)==1 else 'oracle-'+sys.argv[1]+'.json')).write_text(json.dumps(dict(version='IfcOpenShell0.9.0.post1',rows=results),indent=2,default=str)+'\n');print('[DEBUG] '+json.dumps(dict(id=row['id'],direct=result['directMatches'],inverse=result['inverseMatches'],diagnostics=len(result['diagnostics']),bounds=result['actualBounds'])),flush=True)
for row,result in zip(rows,results):
 row['expectedBounds']=result['actualBounds'];row['expectedVolume']=6*abs(float(__import__('numpy').linalg.det(__import__('numpy').array(result['actualMatrix'])[:3,:3])))
(root/'mapping-neutral-cases.json').write_text(json.dumps(data,indent=2)+'\n')
