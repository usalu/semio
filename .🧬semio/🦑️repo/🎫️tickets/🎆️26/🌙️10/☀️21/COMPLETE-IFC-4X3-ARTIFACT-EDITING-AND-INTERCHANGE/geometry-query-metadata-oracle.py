"""🔎 Independent released-schema direct/flattened metadata query contract comparison."""
import ctypes,json,sys
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.ifcopenshell_wrapper as w
repo=t
while not(repo/'✏️s').exists():repo=repo.parent
cases=json.loads((repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🔎️query/🧪️tests/cases.json').read_text());schema=w.schema_by_name('IFC4X3_ADD2');byname={d.name().upper():d for d in schema.declarations()};rows=[]
def owner_of(entity,name):
 while entity:
  if any(a.name().upper()==name.upper() for a in entity.attributes()):return entity.name().upper()
  entity=entity.supertype()
 return None
for row in cases['cases']:
 entity=row['entity'];parts=[entity]+entity.get('complex',[]);matches=[];unknown=False
 for part,source in enumerate(parts):
  declaration=byname.get(source['name'].upper())
  if declaration is None or not hasattr(declaration,'all_attributes'):unknown=True;continue
  if len(matches)==2:continue
  attributes=declaration.attributes() if entity.get('complex') else declaration.all_attributes()
  for index,a in enumerate(attributes):
   if a.name().upper()!=row['name'].upper():continue
   owner=owner_of(declaration,a.name())
   if row.get('owner')and owner!=row['owner'].upper():continue
   matches.append({'value':source['args'][index]if index<len(source['args'])else None,'pointer':'/entities/7/'+('complex/'+str(part-1)+'/'if part else '')+'args/'+str(index),'owner':owner,'attribute':a.name(),'type':str(a.type_of_attribute())})
   if len(matches)==2:break
 status='unknown'if unknown else'ambiguous'if len(matches)==2 else'unique'if matches else'missing';same=status==row['status']and [m['value']for m in matches]==row['values']and[m['pointer']for m in matches]==row['pointers'];rows.append({'id':row['id'],'status':status,'matches':matches,'same':same})
out=t/'🗑️generated/geometry-query';(out/'metadata-oracle.json').write_text(json.dumps({'library':ifcopenshell.version,'schema':'IFC4X3_ADD2','rows':rows,'scope':'Metadata owner/slot/alias evidence only, not physical complex validation'},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'cases':len(rows),'matched':sum(r['same']for r in rows),'mismatches':[r['id']for r in rows if not r['same']]}))
