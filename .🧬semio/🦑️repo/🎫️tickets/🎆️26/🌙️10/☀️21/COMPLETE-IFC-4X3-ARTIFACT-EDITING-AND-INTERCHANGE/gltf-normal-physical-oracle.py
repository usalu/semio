import pathlib,json,hashlib
p=pathlib.Path(__file__).resolve().parent
class NormalPhysicalOracle:
 def observe(self,row):raise NotImplementedError
class IfcNormalOracle(NormalPhysicalOracle):
 def __init__(self):
  import ifcopenshell,ifcopenshell.validate
  self.library=ifcopenshell;self.validator=ifcopenshell.validate;self.version=ifcopenshell.version
 def observe(self,row):
  s=row['source'];f=self.library.file(schema='IFC4X3_ADD2');origin=f.create_entity('IfcCartesianPoint',Coordinates=(0.,0.,0.));axis=f.create_entity('IfcAxis2Placement3D',Location=origin);context=f.create_entity('IfcGeometricRepresentationContext',ContextType='Model',CoordinateSpaceDimension=3,Precision=1.e-6,WorldCoordinateSystem=axis);unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE');units=f.create_entity('IfcUnitAssignment',Units=[unit]);f.create_entity('IfcProject',GlobalId='0000000000000000000001',Name=row['id'],RepresentationContexts=[context],UnitsInContext=units)
  points=f.create_entity('IfcCartesianPointList3D',CoordList=[tuple(float(x)for x in point)for point in s['points']]);face=f.create_entity('IfcTriangulatedFaceSet',Coordinates=points,Normals=None if s['normals']is None else[tuple(float(x)for x in normal)for normal in s['normals']],Closed=s['closed'],CoordIndex=[tuple(x)for x in s['triangles']],PnIndex=s['pnIndex']);rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=context,RepresentationIdentifier='Body',RepresentationType='Tessellation',Items=[face]);shape=f.create_entity('IfcProductDefinitionShape',Representations=[rep]);placement=f.create_entity('IfcLocalPlacement',RelativePlacement=axis);f.create_entity('IfcBuildingElementProxy',GlobalId='0000000000000000000002',Name=row['id'],ObjectPlacement=placement,Representation=shape)
  folder=p/'🗑️generated/gltf-normals';path=folder/(row['id']+'.ifc');f.write(str(path));again=self.library.open(str(path));restored=again.by_id(face.id());log=self.validator.json_logger();self.validator.validate(again,log,express_rules=True)
  return{'id':row['id'],'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'schema':list(again.header.file_schema.schema_identifiers),'faceSet':restored.id(),'coordinates':len(restored.Coordinates.CoordList),'normals':None if restored.Normals is None else len(restored.Normals),'pnIndex':restored.PnIndex,'normalValues':restored.Normals,'diagnostics':[{'attribute':x.get('attribute'),'message':x.get('message')}for x in log.statements]}
backend=IfcNormalOracle();rows=[backend.observe(row)for row in json.loads((p/'gltf-authored-normal-cases.json').read_text())['cases']];assert all(not row['diagnostics']for row in rows)
(p/'🗑️generated/gltf-normals/physical.json').write_text(json.dumps({'version':backend.version,'rows':rows},indent=2),encoding='utf8');print(json.dumps({'version':backend.version,'physical':len(rows),'expressDiagnostics':sum(len(row['diagnostics'])for row in rows)}))
