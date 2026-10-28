"""🗳️ Register independently closed cumulative Boolean session laws."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
anchor='  "📐️geometry/🧊️boolean/🧪️tests/control-policy.test.ts",'
new='  "📐️geometry/🧊️boolean/🧪️tests/session.test.ts",'
if new not in source:
 assert anchor in source
 source=source.replace(anchor,anchor+"\n"+new,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==151,len(selected)
(ticket/"registered151.md").write_text("# Independently Closed Registry 151\n\nCumulative Boolean session32 is registered after independent53-source closure: actual794pass/149409 Bun assertions across9files, strict9consumers0, all53 hashes stationary. Workspace reservations and IFC dispatch are not implemented by that closure. Registry is151; no full151 run has completed.\n\nLast full green remains135. Initial145 failed8 and its originals are archived; all eight corrected and independently closed by the axis/TFS21 audit. Complete IfcValue37 and aggregate weights32 are also independently closed and registered.\n\nNext retention comparison must explicitly report the one deliberate Details SELECT-name supersession from host-type-value-select-supersession.md. Native execution, renderer and full IFC4x3/all-MVD completion remain unproven; goal/ticket stay active.\n",encoding="utf8")
print("[DEBUG] session independently registered; actual151, no full151 result")
