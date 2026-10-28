import pathlib,json,hashlib
p=pathlib.Path(__file__).resolve().parent
class RoundPhysicalObserver:
 def observe(self,row):raise NotImplementedError
class IfcRoundPhysical(RoundPhysicalObserver):
 def __init__(self):
  import ifcopenshell,ifcopenshell.validate,ifcopenshell.util.placement
  self.lib=ifcopenshell;self.validation=ifcopenshell.validate;self.placement=ifcopenshell.util.placement;self.version=ifcopenshell.version
 def observe(self,row):
  s=row['source'];f=self.lib.file(schema='IFC4X3_ADD2')
  def point(values):return f.create_entity('IfcCartesianPoint',Coordinates=tuple(float(x)for x in values))
  def direction(values):return None if values is None else f.create_entity('IfcDirection',DirectionRatios=tuple(float(x)for x in values))
  def frame(values):return f.create_entity('IfcAxis2Placement3D',Location=point(values['location']),Axis=direction(values['axis']),RefDirection=direction(values['refDirection']))
  world=frame({'location':[0,0,0],'axis':None,'refDirection':None});context=f.create_entity('IfcGeometricRepresentationContext',ContextType='Model',CoordinateSpaceDimension=3,Precision=1.e-6,WorldCoordinateSystem=world);unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Prefix=s['unitPrefix'],Name='METRE');units=f.create_entity('IfcUnitAssignment',Units=[unit]);f.create_entity('IfcProject',GlobalId='0000000000000000000001',Name=row['id'],RepresentationContexts=[context],UnitsInContext=units)
  try:
   position=point([0,0,0])if s['damage']=='foreign-position'else frame(s['position'])
   def value(field):return str(s[field])if s[field+'Kind']=='string'else int(s[field])if s[field+'Kind']=='integer'else float(s[field])
   name='IfcRightCircularCylinder'if s['primitive']=='cylinder'else'IfcRightCircularCone';dimensions={'Height':value('height'),('Radius'if s['primitive']=='cylinder'else'BottomRadius'):value('radius')};primitive=f.create_entity(name,Position=position,**dimensions);solid=f.create_entity('IfcCsgSolid',TreeRootExpression=primitive);rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=context,RepresentationIdentifier='Body',RepresentationType='CSG',Items=[solid]);mapped=None
   if s['mapping']is not None:
    m=s['mapping'];origin=frame(m['origin']);repmap=f.create_entity('IfcRepresentationMap',MappingOrigin=origin,MappedRepresentation=rep);operator=f.create_entity('IfcCartesianTransformationOperator3DnonUniform',Axis1=direction(m['axes'][0]),Axis2=direction(m['axes'][1]),Axis3=direction(m['axes'][2]),LocalOrigin=point(m['target']),Scale=m['scale'],Scale2=m['secondary'][0],Scale3=m['secondary'][1]);mapped=f.create_entity('IfcMappedItem',MappingSource=repmap,MappingTarget=operator);rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=context,RepresentationIdentifier='Body',RepresentationType='MappedRepresentation',Items=[mapped])
   shape=f.create_entity('IfcProductDefinitionShape',Representations=[rep]);local=f.create_entity('IfcLocalPlacement',RelativePlacement=frame(s['productPlacement']));product=f.create_entity('IfcBuildingElementProxy',GlobalId='0000000000000000000002',Name=row['id'],ObjectPlacement=local,Representation=shape)
   folder=p/'🗑️generated/csg-round-independent';path=folder/(row['id']+'.ifc');f.write(str(path));again=self.lib.open(str(path));log=self.validation.json_logger();self.validation.validate(again,log,express_rules=True)
   return{'id':row['id'],'construction':'physical','fileSha256':hashlib.sha256(path.read_bytes()).hexdigest(),'schema':list(again.header.file_schema.schema_identifiers),'primitive':primitive.id(),'product':product.id(),'positionMatrix':self.placement.get_axis2placement(position).tolist()if s['damage']=='none'else None,'mappingMatrix':None if mapped is None else self.placement.get_mappeditem_transformation(mapped).tolist(),'diagnostics':[{'attribute':x.get('attribute'),'message':x.get('message'),'instance':str(x.get('instance'))}for x in log.statements],'geometry':'not-run; original complete default cylinder/cone normal backend unsupported separately recorded'}
  except Exception as error:return{'id':row['id'],'construction':'refused','error':str(error)}
backend=IfcRoundPhysical();law=json.loads((p/'csg-round-cases.json').read_text());rows=[]
for row in law['cases']:
 observation=backend.observe(row);rows.append(observation);print('[DEBUG] '+row['id']+' '+observation['construction'],flush=True)
(p/'🗑️generated/csg-round-independent/physical.json').write_text(json.dumps({'library':backend.version,'rows':rows},indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'cases':len(rows),'physical':sum(x['construction']=='physical'for x in rows),'diagnosticRows':sum(bool(x.get('diagnostics'))for x in rows)}))
