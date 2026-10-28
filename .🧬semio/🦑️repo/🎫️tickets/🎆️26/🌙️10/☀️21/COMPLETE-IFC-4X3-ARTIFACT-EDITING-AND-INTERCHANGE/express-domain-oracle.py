"""🛂 Independent domain uniqueness validation over neutral STEP body vectors."""
import json,pathlib,sys
sys.path.insert(0,sys.argv[1]);import ifcopenshell,ifcopenshell.validate
from multiset import Multiset
def key(value):
 if isinstance(value,ifcopenshell.entity_instance):return ('entity',value.id()) if value.id() else key(value.wrappedValue)
 if isinstance(value,(tuple,list)):return tuple(key(item) for item in value)
 return value
cases=json.loads(pathlib.Path(sys.argv[2]).read_text(encoding='utf-8-sig'));envelope=json.loads(pathlib.Path(sys.argv[3]).read_text(encoding='utf-8-sig'));rows=[]
for row in cases:
 model=ifcopenshell.file.from_string(envelope['header']+row['body']+envelope['footer']);logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=False)
 diagnostics=[{'attribute':entry.get('attribute'),'message':entry['message']} for entry in logger.statements]
 owners=model.by_type('IfcUnitAssignment') or model.by_type('IfcPropertyEnumeration')
 values=owners[0].Units if owners[0].is_a('IfcUnitAssignment') else owners[0].EnumerationValues
 counted=Multiset(key(value) for value in values);duplicate=len(counted)>len(counted.distinct_elements())
 assert duplicate==bool(row['codes'])
 rows.append({'name':row['name'],'expected':row['codes'],'ifcopenshellValidator':diagnostics,'parsedMultisetDuplicate':duplicate,'matches':True})
(pathlib.Path(sys.argv[4])/'express-domain-oracle.json').write_text(json.dumps(rows,indent=2),encoding='utf-8');print(json.dumps(rows))

