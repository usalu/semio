import pathlib,json,hashlib
import ifcopenshell,ifcopenshell.validate
p=pathlib.Path(__file__).resolve().parent
case=json.loads((p/'tfs-admission-cases.json').read_text())['cases'][0];s=case['source']
f=ifcopenshell.file(schema='IFC4X3_ADD2')
origin=f.create_entity('IfcCartesianPoint',Coordinates=(0.,0.,0.));axis=f.create_entity('IfcAxis2Placement3D',Location=origin)
context=f.create_entity('IfcGeometricRepresentationContext',ContextIdentifier=None,ContextType='Model',CoordinateSpaceDimension=3,Precision=1.e-6,WorldCoordinateSystem=axis)
unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE');units=f.create_entity('IfcUnitAssignment',Units=[unit]);project=f.create_entity('IfcProject',GlobalId='0000000000000000000001',Name=case['id'],RepresentationContexts=[context],UnitsInContext=units)
points=f.create_entity('IfcCartesianPointList3D',CoordList=[tuple(float(x) for x in row)for row in s['points']])
face=f.create_entity('IfcTriangulatedFaceSet',Coordinates=points,Normals=None,Closed=s['closed'],CoordIndex=[tuple(row)for row in s['triangles']],PnIndex=None)
rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=context,RepresentationIdentifier='Body',RepresentationType='Tessellation',Items=[face]);shape=f.create_entity('IfcProductDefinitionShape',Representations=[rep]);placement=f.create_entity('IfcLocalPlacement',RelativePlacement=axis)
products=[]
for at in range(32):
 number=at+2;word='';digits='0123456789abcdefghijklmnopqrstuvwxyz'
 while number:word=digits[number%36]+word;number//=36
 guid=word.rjust(22,'0')
 products.append(f.create_entity('IfcBuildingElementProxy',GlobalId=guid,Name=case['id'],ObjectPlacement=placement,Representation=shape))
f.header.file_description.description=('ViewDefinition [notYetAssigned]',);f.header.file_name.name='tfs-shared32';f.header.file_schema.schema_identifiers=('IFC4X3_ADD2',)
out=p/'🗑️generated/tfs-admission/shared32.ifc';f.write(str(out));again=ifcopenshell.open(str(out));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(again,log,express_rules=True)
result={'library':ifcopenshell.version,'fileSha256':hashlib.sha256(out.read_bytes()).hexdigest(),'sourceCase':case['id'],'entities':len(list(again)),'schema':list(again.header.file_schema.schema_identifiers),'products':[{'id':x.id(),'guid':x.GlobalId,'placement':x.ObjectPlacement.id(),'shape':x.Representation.id(),'representation':x.Representation.Representations[0].id(),'items':[a.id()for a in x.Representation.Representations[0].Items]}for x in again.by_type('IfcBuildingElementProxy')],'logicalOutputScalars':32*(len(s['points'])*3+len(s['triangles'])*3),'diagnostics':[{'attribute':x.get('attribute'),'message':x.get('message'),'instance':str(x.get('instance'))}for x in log.statements]}
assert len({x['guid']for x in result['products']})==32
(p/'🗑️generated/tfs-admission/shared32-oracle.json').write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps({'version':result['library'],'entities':result['entities'],'products':len(products),'scalars':result['logicalOutputScalars'],'diagnostics':len(result['diagnostics'])}))
