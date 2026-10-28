"""🫧 Capture exact unchanged official face owners and indexable loop records."""
import ast,hashlib,json
from pathlib import Path
p=Path(__file__).parent;source=p/'surface-geometry-oracle.py';tree=ast.parse(source.read_text(encoding='utf-8-sig'));cut=next(i for i,n in enumerate(tree.body)if isinstance(n,ast.Assign)and any(isinstance(t,ast.Name)and t.id=='out'for t in n.targets));ns={'__file__':str(source)};exec(compile(ast.Module(body=tree.body[:cut],type_ignores=[]),str(source),'exec'),ns);library=ns['ifcopenshell'];path=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/surface-model.ifc');f=library.open(str(path));m=f.by_id(1021);groups=[]
for group in m.FbsmFaces:
 faces=[]
 for face in group.CfsFaces:
  faces.append(dict(id=face.id(),kind=face.is_a(),bounds=[dict(id=b.id(),kind=b.is_a(),orientation=b.Orientation,loop=b.Bound.id(),loopKind=b.Bound.is_a(),points=[dict(id=q.id(),coordinates=list(q.Coordinates))for q in b.Bound.Polygon])for b in face.Bounds]))
 groups.append(dict(id=group.id(),kind=group.is_a(),faces=faces))
r=dict(sha256=hashlib.sha256(path.read_bytes()).hexdigest(),header=list(f.header.file_schema.schema_identifiers),model=m.id(),groups=groups);(p/'🗑️generated/surface-geometry/official-roster.json').write_text(json.dumps(r,indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(dict(groups=len(groups),faces=sum(len(g['faces'])for g in groups),sha256=r['sha256'])))
