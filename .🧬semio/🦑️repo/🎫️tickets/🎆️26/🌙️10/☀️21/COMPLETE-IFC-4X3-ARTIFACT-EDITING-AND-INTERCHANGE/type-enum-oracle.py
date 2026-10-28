import sys,json,pathlib
root=pathlib.Path(__file__).parent;sys.path.insert(0,str(root/'🗑️generated'/'ifc083'))
import ifcopenshell,ifcopenshell.validate
contract=json.loads((pathlib.Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-enum-input.json')).read_text(encoding='utf8'))
schema=ifcopenshell.ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
for row in contract['cases']:
 for field in row['required']:
  if field['domain'].startswith('IFC'):assert list(schema.declaration_by_name(field['domain']).enumeration_items())==field['members']
header="ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('Caller enum inputs'),'2;1');\nFILE_NAME('choices.ifc','2026-10-21T00:00:00',('semio'),('semio'),'semio','semio','');\nFILE_SCHEMA(('IFC4X3_ADD2'));\nENDSEC;\nDATA;\n"
points="#9=IFCCARTESIANPOINT((0.,0.));\n#10=IFCCARTESIANPOINT((1.,0.));\n"
models=[('boolean',"#1=IFCCOMPOSITECURVESEGMENT(.CONTINUOUS.,.T.,#7);\n#7=IFCPOLYLINE((#9,#10));\n#11=IFCCOMPOSITECURVE((#1),.F.);\n"+points),('logical',"#1=IFCBSPLINECURVEWITHKNOTS(1,(#9,#10),.POLYLINE_FORM.,.U.,.F.,(2,2),(0.,1.),.UNSPECIFIED.);\n"+points),('full53',"#1=IFCDERIVEDUNIT((#7),.USERDEFINED.,'Caller unit type','Caller unit');\n#7=IFCDERIVEDUNITELEMENT(#8,2);\n#8=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n"),('full34',"#1=IFCSENSORTYPE('0AAAAAAAAAAAAAAAAAAAAA',$,'Caller sensor',$,$,$,$,$,'Caller sensor type',.NOTDEFINED.);\n")]
results=[]
for name,data in models:
 model=ifcopenshell.file.from_string(header+data+'ENDSEC;\nEND-ISO-10303-21;\n');logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True);assert not logger.statements,(name,logger.statements)
 entity=model.by_id(1)
 if name=='boolean':assert entity.SameSense is True;value='T'
 elif name=='logical':assert entity.ClosedCurve=='UNKNOWN' and entity.SelfIntersect is False;value=['U','F']
 elif name=='full53':assert entity.Elements[0].id()==7 and entity.UnitType=='USERDEFINED';value=entity.UnitType
 else:assert entity.PredefinedType=='NOTDEFINED';value=entity.PredefinedType
 results.append({'case':name,'entity':entity.is_a(),'semanticValue':value,'fullExpressDiagnostics':0})
result={'oracle':ifcopenshell.version,'schema':'IFC4X3_ADD2','nativeExecution':False,'representation':'equivalent flattened complete target','exactNamedDomainCounts':[53,34],'cases':results};(root/'🗑️generated/type-enum-oracle.json').write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps(result))
