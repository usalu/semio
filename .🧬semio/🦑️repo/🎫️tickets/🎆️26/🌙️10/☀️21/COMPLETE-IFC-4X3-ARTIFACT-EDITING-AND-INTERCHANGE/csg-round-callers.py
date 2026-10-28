"""🧩️ Update all actual CSG evaluator consumers for the captured circular policy."""
from pathlib import Path
ticket=Path(__file__).parent
tests=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
policy="{absoluteChordErrorMetres:0,relativeChordError:.001,maxSegments:4096}"
edits={
 "csg-block.test.ts":[("ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,59)","ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,"+policy+",59)"),("ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,60)","ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,"+policy+",60)")],
 "csg-round.test.ts":[("ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,grant-1)","ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,cases.cases[0]!.source.policy,grant-1)"),("ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,grant)","ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,cases.cases[0]!.source.policy,grant)")]
}
for name,pairs in edits.items():
 path=tests/name
 before=path.read_bytes()
 after=before
 for old,new in pairs:
  assert after.count(old.encode())==1,(name,old)
  after=after.replace(old.encode(),new.encode())
 assert path.read_bytes()==before
 path.write_bytes(after)
print("[DEBUG] actual CSG caller signatures updated")
