"""🧊️ Independent metadata supports borrowed query laws without claiming native or complex physical execution."""
import ctypes,json,sys
from pathlib import Path
ticket=Path(__file__).resolve().parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
 buffer=ctypes.create_unicode_buffer(32768)
 ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
 dependency=buffer.value or dependency
sys.path.insert(0,dependency)
class MetadataOracle:
 def fields(self,name,direct):raise NotImplementedError
class IfcOpenShellMetadata(MetadataOracle):
 def __init__(self):
  import ifcopenshell,ifcopenshell.ifcopenshell_wrapper as wrapper
  self.version=ifcopenshell.version
  self.declarations={item.name().upper():item for item in wrapper.schema_by_name('IFC4X3_ADD2').declarations()}
 def fields(self,name,direct):
  declaration=self.declarations.get(name.upper())
  if declaration is None or not hasattr(declaration,'all_attributes'):return None
  result=[]
  for index,attribute in enumerate(declaration.attributes() if direct else declaration.all_attributes()):
   owner=declaration
   while owner and not any(item.name()==attribute.name() for item in owner.attributes()):owner=owner.supertype()
   result.append({'index':index,'name':attribute.name(),'owner':owner.name().upper() if owner else None,'type':str(attribute.type_of_attribute())})
  return result
oracle=IfcOpenShellMetadata()
repo=ticket
while not(repo/'✏️s').exists():repo=repo.parent
fixture=json.loads((repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🔎️query/🧪️tests/cases.json').read_text(encoding='utf-8'))
rows=[]
for case in fixture['cases']:
 entity=case['entity'];matches=[];unknown=False
 for part,source in enumerate([entity]+entity.get('complex',[])):
  attributes=oracle.fields(source['name'],bool(entity.get('complex')))
  if attributes is None:unknown=True;continue
  if len(matches)==2:continue
  for attribute in attributes:
   if attribute['name'].upper()!=case['name'].upper() or case.get('owner') and attribute['owner']!=case['owner'].upper():continue
   index=attribute['index'];matches.append({'value':source['args'][index] if index<len(source['args']) else None,'pointer':f"/entities/7/{'complex/'+str(part-1)+'/' if part else ''}args/{index}",'owner':attribute['owner']})
   if len(matches)==2:break
 status='unknown' if unknown else 'ambiguous' if len(matches)==2 else 'unique' if matches else 'missing'
 rows.append({'id':case['id'],'status':status,'matches':matches,'equal':status==case['status'] and [entry['value']for entry in matches]==case['values'] and [entry['pointer']for entry in matches]==case['pointers']})
representation=oracle.fields('IFCREPRESENTATION',True)
predicates=[{'name':name,'index':index,'equal':any(item['name']==name and item['index']==index and item['owner']=='IFCREPRESENTATION' for item in representation)}for name,index in [('RepresentationIdentifier',1),('RepresentationType',2),('Items',3)]]
result={'library':'IfcOpenShell','version':oracle.version,'schema':'IFC4X3_ADD2','rows':rows,'representationPredicates':predicates,'nativeExecuted':False,'physicalComplexValidated':False}
output=ticket/'🗑️generated/native-attribute-query';output.mkdir(exist_ok=True)
(output/'oracle.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
assert all(row['equal']for row in rows+predicates)
print('[DEBUG] '+json.dumps({'queryCases':len(rows),'predicateOwners':len(predicates),'matched':len(rows)+len(predicates),'version':oracle.version,'nativeExecuted':False}))
