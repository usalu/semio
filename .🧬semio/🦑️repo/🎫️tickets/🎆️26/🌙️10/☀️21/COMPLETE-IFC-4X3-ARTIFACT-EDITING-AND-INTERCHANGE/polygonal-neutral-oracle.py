"""🔷 Execute explicit polygonal physical rows behind the owned oracle interface."""
import ast,json,sys,hashlib
from pathlib import Path
root=Path(__file__).parent;source=root/'polygonal-identity-oracle.py';tree=ast.parse(source.read_text(encoding='utf-8-sig'));stop=next(i for i,n in enumerate(tree.body)if isinstance(n,ast.Assign)and any(isinstance(t,ast.Name)and t.id=='p'for t in n.targets));ns={'__file__':str(source)};exec(compile(ast.Module(body=tree.body[:stop],type_ignores=[]),str(source),'exec'),ns);out=root/'🗑️generated/polygonal-neutral';out.mkdir(exist_ok=True);ns['out']=out;oracle=ns['IfcOpenShellPolygonalOracle']();rows=[]
for row in json.loads((root/'polygonal-neutral-cases.json').read_text(encoding='utf-8'))['cases']:
 if row['damage']!='none':rows.append(dict(id=row['id'],notConstructed='requires deliberately malformed physical role/reference injection; no external validity claim'));continue
 try:r=oracle.observe(row)
 except Exception as e:r=dict(id=row['id'],constructionError=str(e))
 rows.append(r);(out/'oracle.json').write_text(json.dumps(dict(version=ns['ifcopenshell'].version,rows=rows),indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(dict(id=row['id'],errors=len(r.get('diagnostics',[])),mesh=r.get('mesh'),error=r.get('constructionError')or r.get('geometryError'))),flush=True)
