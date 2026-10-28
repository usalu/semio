"""🧊️ Register independently closed actual workspace and root-read reference laws."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
entries=["📐️geometry/🧊️boolean/🧪️tests/"+name for name in["workspace-kernel.test.ts","owned-workspace.test.ts","owned-workspace-transfer.test.ts","workspace-copy.test.ts"]]+["🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/root-read.test.ts"]
anchor='  "📐️geometry/🧊️boolean/🧪️tests/workspace.test.ts",'
for entry in entries:
 line='  "'+entry+'",'
 if line not in source:
  assert anchor in source
  source=source.replace(anchor,anchor+"\n"+line,1)
  anchor=line
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==162,len(selected)
(ticket/"registered162.md").write_text("# Independently Closed Registry 162\n\nFour actual workspace suites98 laws are registered after independent69-source closure, fresh928pass331819assert14files and strict14consumer0. The original918 checkpoint and independently reproduced borrowed-copy TOCTOU finding remain retained; corrected indexed copying plus private result admission now rejects the exact invalid-triangle-index probe. All69 source hashes were stationary in final closure. This is actual TS reserved numeric backing ownership, not physical heap/native funding.\n\nThe root-read reference5 suite is registered after independent39-source native audit: fresh5pass10assert, SDK4exact/SQL3exact1outside, all39 hashes unchanged. Mounted native local-view/memo/alias laws remain source-only because authentic Cargo101 refuses before rustc. Native typing, execution, receipts and publication flags remain false.\n\nCurrent162 has no full run yet. Immutable fullgreen154 remains4871pass12skip0fail372425assert4883tests, with three documented prior-obligation supersessions and zero unexplained missing. Rounded75 is held for independent audit; Boolean policy64 is genuine1pass63featurefail red after repairing a malformed new schema JSON harness. Finite31/nested14 are still red. Source writers and actual IFC Boolean dispatcher remain incomplete. Goal/ticket active.\n",encoding="utf8")
print("[DEBUG] registered162; actualworkspace98 and rootread5 independently closed")
