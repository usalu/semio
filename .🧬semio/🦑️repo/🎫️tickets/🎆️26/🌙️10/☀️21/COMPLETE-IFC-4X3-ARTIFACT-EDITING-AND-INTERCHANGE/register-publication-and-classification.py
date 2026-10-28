"""🌳️ Register independently closed indexed publication and all-owner discriminating semantic cases."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
entries=["📐️geometry/🧊️boolean/🧪️tests/indexed-publication.test.ts","🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️classification/🧪️.test.ts"]
anchor='  "📐️geometry/🧊️boolean/🧪️tests/workspace-copy.test.ts",'
for entry in entries:
 line='  "'+entry+'",'
 if line not in source:
  assert anchor in source
  source=source.replace(anchor,anchor+"\n"+line,1)
  anchor=line
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle|🔲️convex)/[^"\r\n]+)"',source)
assert len(selected)==167,len(selected)
(ticket/"registered167.md").write_text("# Independently Closed Registry 167\n\nIndexed publication18 is registered after independent70-source closure:946pass332181assert15files, strict15consumer0 and70stationary hashes. Original workspace69 closure is retained. This publication includes simultaneous original and final flat backing, actual private custody and final observer, with native proof flags false.\n\nClassification162 is registered after independent39-source closure:162/497 plus strict0,154 issued-function inputs, SDK143exact/3storage/8 explicit differences and SQL154exact; all39stationary hashes. Full source has exactly15 intended WHERE failures; fresh SDK13 misses only the known two colour SELECT predicates. All48 issued function owners now have selected direct discriminating cases; this is not whole-branch or all-MVD/spec conformance. No production evaluator change follows from this new coverage.\n\nCurrent167 has no full run yet; immutable fullgreen154 remains4871pass12skip0fail372425assert4883tests with three documented prior-obligation supersessions and zero unexplained missing. The actual firstIFCBoolean producer is green in focused matrices and originalofficialbath but awaits coherent sharedwriter closure/broadstrict/independent audit. Goal/ticket active and incomplete.\n",encoding="utf8")
print("[DEBUG] registered167; indexed18 and classification162 independently closed")
