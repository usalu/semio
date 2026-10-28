"""🔲️ Register independently closed paired-boundary and actual rounded producer laws."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
entries=["📐️geometry/🔲️convex/🧪️tests/🟦️.test.ts"]+["🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/"+name for name in["rounded-rectangle.test.ts","rounded-workflow.test.ts"]]
anchor='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-primitive.test.ts",'
for entry in entries:
 line='  "'+entry+'",'
 if line not in source:
  assert anchor in source
  source=source.replace(anchor,anchor+"\n"+line,1)
  anchor=line
path.write_text(source,encoding="utf8")
runner=ticket/"ifc4x3-and-details-tests.ts"
runner.write_text(runner.read_text(encoding="utf8").replace("🕳️rings|🧊️boolean|⭕️circle)","🕳️rings|🧊️boolean|⭕️circle|🔲️convex)"),encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle|🔲️convex)/[^"\r\n]+)"',source)
assert len(selected)==165,len(selected)
(ticket/"registered165.md").write_text("# Independently Closed Registry 165\n\nThree rounded/boundary suites72 laws are registered after independent75-source closure:441 affected laws69434assert10files plus pure17/398, strict11consumer0 and75stationary hashes. Independent prior Three8 ring observations match actual scalar output within9e-16 (millimetre source units1.8e-13). Pure paired native mounting/syntax and genuineCargo101 remain source-only. Actual capsule/circle equality, declared REAL carrier refusals, total-boundary caps, reflected oblique sweeps, canonical edits/inverses and realGLB are covered. Primary/SDK divergences and harness corrections remain retained in the implementation/audit reports.\n\nThe broad runner now includes the new actual convex domain path, so its registered suite is selected. Current165 has no full run. Last immutable fullgreen154 remains4871pass12skip0fail372425assert4883tests with three documented prior-obligation supersessions and zero unexplained missing. Workspace98 and rootread5 were previously added in162.\n\nThe closed rounded sources have been released to High for a shared direct-to-owned-writer refactor; this registration does not claim future source stationarity. New original Boolean policy64 now passes132assert but is not independently audited/registered. Actual IFC Boolean AST, official original bath rendering, further EXPRESS owners and native execution remain incomplete. Goal/ticket active.\n",encoding="utf8")
print("[DEBUG] registered165; rounded72 closed; fullselection includes convex domain")
