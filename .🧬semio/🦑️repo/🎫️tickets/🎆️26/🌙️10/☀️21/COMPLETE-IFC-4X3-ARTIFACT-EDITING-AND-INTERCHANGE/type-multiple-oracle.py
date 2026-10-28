import sys,json,pathlib
root=pathlib.Path(__file__).parent
sys.path.insert(0,str(root/'🗑️generated'/'ifc083'))
import ifcopenshell,ifcopenshell.validate
schema=ifcopenshell.ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
maximum=0;names=[]
for declaration in schema.declarations():
 entity=declaration.as_entity()
 if entity is None: continue
 count=0
 for attribute in entity.attributes():
  named=attribute.type_of_attribute().as_named_type()
  if not attribute.optional() and named is not None and named.declared_type().as_entity() is not None: count+=1
 if count>maximum: maximum=count;names=[entity.name().upper()]
 elif count==maximum: names.append(entity.name().upper())
assert (maximum,names)==(4,['IFCSURFACESTYLELIGHTING']),(maximum,names)
rows=[]
for name in ['IfcRelConnectsElements','IfcSurfaceStyleLighting']:
 declaration=schema.declaration_by_name(name)
 rows.append({'name':name,'direct':[{'name':attr.name(),'optional':attr.optional()}for attr in declaration.attributes()]})
text="""ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Multiple caller references'),'2;1');
FILE_NAME('multiple.ifc','2026-10-21T00:00:00',('semio'),('semio'),'semio','semio','');
FILE_SCHEMA(('IFC4X3_ADD2'));
ENDSEC;
DATA;
#1=IFCSURFACESTYLELIGHTING(#10,#11,#12,#13);
#10=IFCCOLOURRGB($,1.,0.,0.);
#11=IFCCOLOURRGB($,0.,1.,0.);
#12=IFCCOLOURRGB($,0.,0.,1.);
#13=IFCCOLOURRGB($,1.,1.,1.);
ENDSEC;
END-ISO-10303-21;
"""
model=ifcopenshell.file.from_string(text)
logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True)
refs=[value.id()for value in model.by_id(1)]
assert refs==[10,11,12,13]
assert not logger.statements,logger.statements
result={'oracle':ifcopenshell.version,'nativeExecution':False,'schema':'IFC4X3_ADD2','releasedMaximum':maximum,'releasedMaximumEntities':names,'declarations':rows,'typedReferences':refs,'fullExpressDiagnostics':len(logger.statements)}
(root/'🗑️generated'/'type-multiple-oracle.json').write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps(result))


