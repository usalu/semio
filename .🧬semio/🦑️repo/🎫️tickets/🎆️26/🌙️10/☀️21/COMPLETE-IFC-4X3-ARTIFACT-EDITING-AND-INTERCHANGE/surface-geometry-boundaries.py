"""🫧 Extend surface inputs for group boundary and subtype semantics."""
import ast,copy,json
from pathlib import Path
p=Path(__file__).parent;path=p/'surface-geometry-cases.json';data=json.loads(path.read_text(encoding='utf-8'));by={r['id']:r for r in data['cases']};extra=[]
def add(id,base,groups=None,expected=None,area=None):
 r=copy.deepcopy(by[base]);r['id']=id
 if groups is not None:r['groups']=groups
 if expected is not None:r['expected']=expected;r['area']=area
 extra.append(r);return r
add('common-edge-separate-groups','adjacent-open-faces',[dict(kind='IFCCONNECTEDFACESET',faces=[0]),dict(kind='IFCCONNECTEDFACESET',faces=[1])])
add('common-vertex-separate-groups','vertex-connected-open-faces',[dict(kind='IFCCONNECTEDFACESET',faces=[0]),dict(kind='IFCCONNECTEDFACESET',faces=[1])])
add('common-source-face-separate-groups','single-face',[dict(kind='IFCCONNECTEDFACESET',faces=[0]),dict(kind='IFCCONNECTEDFACESET',faces=[0])],expected='supported',area=4)
r=add('identical-boundaries-distinct-faces','single-face',[dict(kind='IFCCONNECTEDFACESET',faces=[0]),dict(kind='IFCCONNECTEDFACESET',faces=[1])],expected='supported',area=4);r['faces'].append(copy.deepcopy(r['faces'][0]))
add('vertex-pinched-open-shell','vertex-connected-open-faces',[dict(kind='IFCOPENSHELL',faces=[0,1])],expected='unsupported',area=None)
r=add('crossing-group-interiors','disconnected-separate-groups',expected='unsupported',area=None);r['points']=[[0,0,0],[2,0,0],[2,2,0],[0,2,0],[1,0,-1],[1,2,-1],[1,2,1],[1,0,1]]
r=add('partial-coplanar-overlap','disconnected-separate-groups',expected='unsupported',area=None);r['points']=[[0,0,0],[2,0,0],[2,2,0],[0,2,0],[1,0,0],[3,0,0],[3,2,0],[1,2,0]]
data['cases'].extend(extra);path.write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8');source=p/'surface-geometry-oracle.py';tree=ast.parse(source.read_text(encoding='utf-8-sig'));cut=next(i for i,n in enumerate(tree.body)if isinstance(n,ast.Assign)and any(isinstance(t,ast.Name)and t.id=='out'for t in n.targets));ns={'__file__':str(source)};exec(compile(ast.Module(body=tree.body[:cut],type_ignores=[]),str(source),'exec'),ns);ns['out']=p/'🗑️generated/surface-geometry';oracle=ns['IfcOpenShellSurfaceOracle']();target=ns['out']/'oracle.json';result=json.loads(target.read_text(encoding='utf-8'))
for row in extra:
 r=oracle.observe(row);result['rows'].append(r);target.write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(dict(id=r['id'],diagnostics=len(r['diagnostics']),mesh=r.get('mesh'),error=r.get('geometryError'))),flush=True)
