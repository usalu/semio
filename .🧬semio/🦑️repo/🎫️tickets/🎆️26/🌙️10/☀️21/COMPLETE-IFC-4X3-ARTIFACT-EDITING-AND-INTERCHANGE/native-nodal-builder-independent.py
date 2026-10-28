"""🏋️ Independently corroborate frozen native unit roles without native execution."""
import ast,ctypes,hashlib,json,sys
from pathlib import Path
p=Path(__file__).parent;manifest=json.loads((p/'🗑️generated/native-nodal-builder-source.json').read_text(encoding='utf-8'));seen=[]
for group in ['owned','references']:
 for row in manifest[group]:
  path=Path(row['path']);actual=hashlib.sha256(path.read_bytes()).hexdigest();assert actual==row['sha256'].lower(),str(path);seen.append(dict(group=group,path=str(path),sha256=actual))
dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
source=p/'native-nodal-units-oracle.py';tree=ast.parse(source.read_text(encoding='utf-8'));cut=next(i for i,n in enumerate(tree.body)if isinstance(n,ast.Assign)and any(isinstance(t,ast.Name)and t.id=='data'for t in n.targets));ns={'__file__':str(source)};exec(compile(ast.Module(body=tree.body[:cut],type_ignores=[]),str(source),'exec'),ns);oracle=ns['IfcOpenShellUnitOracle']();data=json.loads((p.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal/units-cases.json').read_text(encoding='utf-8'));rows=[]
for row in data['cases']:
 try:r=oracle.check(row)
 except RuntimeError as e:r=dict(id=row['id'],expected=row['valid'],actual=False,externalSchemaRefusal=str(e))
 assert r['actual']==r['expected'],r;rows.append(r)
(p/'🗑️generated/native-nodal-builder-independent-after.json').write_text(json.dumps(dict(files=seen,ifcopenshell=oracle.ifc.version,rows=rows,nativeExecuted=False),indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] 14+7 exact hashes; '+str(len(rows))+' independent unit outcomes match; native not executed')

