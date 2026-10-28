"""🧭️ Register independently closed issued arithmetic and original-source isolation laws."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
anchor='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️aggregate/🧪️.test.ts",'
new='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector-arithmetic/🧪️.test.ts",\n  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector-arithmetic/isolation.test.ts",'
if new not in source:
 assert anchor in source
 source=source.replace(anchor,anchor+"\n"+new,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==153,len(selected)
(ticket/"registered153.md").write_text("# Registered 153-Suite Scope\n\nTwo issued arithmetic/source-isolation suites are registered after independent39-source closure: new50pass/12731assert, selected officialAlignment+Normalise2pass8assert, strict2newconsumers0, all39hashes stationary. High broader actual semantic regression697pass/68241assert19files, strict0. Actual fresh SDK28outputs andSQLite28 match; the SDK also changes two original input directions, retained as explicit external source-isolation disagreement. Our read scope preserves all28original graphs and local query alias semantics.\n\nRegistry now153. No full153 run has completed. Last full green remains135; initial145failed8 archives retained and all eight independently corrected. New workspace36 is outside this selection until independent58-source closure. Native Value integration is moving and must reach coherent source freeze before broad readers. Full IFC4x3/all-MVD goal/ticket stay active/incomplete.\n",encoding="utf8")
print("[DEBUG] arithmetic suites independently registered; actual153, no full153 result")
