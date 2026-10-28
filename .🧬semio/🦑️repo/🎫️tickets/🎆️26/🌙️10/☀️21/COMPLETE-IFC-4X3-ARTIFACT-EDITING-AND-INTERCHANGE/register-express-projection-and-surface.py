"""📈️ Register independently closed projection and source identity/dimension coverage."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
anchor='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector-arithmetic/isolation.test.ts",'
new='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️projection/🧪️.test.ts",\n  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️surface-curve/🧪️.test.ts",'
if new not in source:
 assert anchor in source
 source=source.replace(anchor,anchor+"\n"+new,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==156,len(selected)
(ticket/"registered156.md").write_text("# Independently Closed Registry 156\n\nProjection44 and surface/curve66 are registered after independent33/34 source closure, strict0 and exact stationary hashes. These are meaningful public direct coverage of existing issued helpers, with no new semantic producer correction or claimed complete branch coverage. Direct owner census is41/48, seven stillunscored. External SDK discrepancies/unsupported paths and deliberate source invalidities remain explicit in their implementation/audit reports.\n\nLast full green154:4871pass12existinggatedskips0fail372425Bunassertions4883tests534.80s,0selectedtesthashchanges. Retention is3491unchanged historical name/outcome observations,3explicit documented supersessions,0unexplained and1389new. Actualarchives remain tests154.txt/selection154.json with final comparison; initial retention refusal is preserved separately. No full156run yet. New primitive8 and red kernel61/finite31/nested14 remain unregistered.\n\nShared Boolean actualworkspace integration, native EXPRESSroot isolation, actualIFCBoolean/roundedprofile, nativeexecution and fullIFC4x3/all-MVD remain unfinished; goal/ticketactive.\n",encoding="utf8")
print("[DEBUG] two independent semantic suites registered; actual156, lastfullgreen154")
