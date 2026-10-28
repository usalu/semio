import sys,json,pathlib
root=pathlib.Path(__file__).parent;sys.path.insert(0,str(root/'🗑️generated'/'ifc083'))
import ifcopenshell,ifcopenshell.validate
header="ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('Caller scalar inputs'),'2;1');\nFILE_NAME('scalars.ifc','2026-10-21T00:00:00',('semio'),('semio'),'semio','semio','');\nFILE_SCHEMA(('IFC4X3_ADD2'));\nENDSEC;\nDATA;\n"
rows=[]
models=[('application',"#1=IFCAPPLICATION(#7,'','Caller application','\\X4\\0000539F000000200001F9ED0000002000000073000000650000006D000000690000006F\\X0\\');\n#7=IFCORGANIZATION($,'source organization',$,$,$);\n"),('unit',"#1=IFCDERIVEDUNITELEMENT(#7,-2);\n#7=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n"),('dimensional',"#1=IFCDIMENSIONALEXPONENTS(1,-2,0,0,0,0,0);\n")]
for name,data in models:
 model=ifcopenshell.file.from_string(header+data+'ENDSEC;\nEND-ISO-10303-21;\n');logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True);assert not logger.statements,(name,logger.statements)
 entity=model.by_id(1);values=[{'reference':value.id()}if isinstance(value,ifcopenshell.entity_instance)else value for value in entity];rows.append({'case':name,'entity':entity.is_a(),'values':values,'fullExpressDiagnostics':0})
assert rows[0]['values'][1]=='';assert rows[0]['values'][3]=='原 🧭 semio',rows[0];assert rows[1]['values']==[{'reference':7},-2];assert rows[2]['values']==[1,-2,0,0,0,0,0]
result={'oracle':ifcopenshell.version,'schema':'IFC4X3_ADD2','nativeExecution':False,'representation':'flattened target tuple','cases':rows};(root/'🗑️generated/type-scalar-oracle.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps(result,ensure_ascii=True))
