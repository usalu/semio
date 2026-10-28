"""📋️ Register independently closed aggregate and complete IfcValue tests."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
for anchor,new in [
('  "🔎️details/type_measure_select_input.test.ts",','  "🔎️details/type_value_select_input.test.ts",'),
('  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️placement/🧪️.test.ts",','  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️aggregate/🧪️.test.ts",')]:
 if new not in source:
  assert anchor in source,anchor
  source=source.replace(anchor,anchor+"\n"+new,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==150,len(selected)
(ticket/"registered150.md").write_text("# Independently Closed Registry 150\n\nAdded complete IfcValue37 and issued aggregate-weight32 suites after independent source/test closure. Registry selection is150; no full150 run has completed. Last full green is135. Initial145 failed8, archived unchanged, with all eight closed by the axis/TFS correction21. New Boolean session32 is unregistered pending independent53-source audit.\n\nOne intentional prior Details test-name supersession must be reported during the next broad retention comparison: host-type-value-select-supersession.md records the original oversized SELECT refusal and its recursive real-domain paging replacement, with both hashes. Do not label this as zero missing names.\n\nNative execution, renderer and full IFC4x3/all-MVD completion remain unproven. Goal/ticket are active.\n",encoding="utf8")
print("[DEBUG] independent suites registered; actual selection150, no full150 result")
