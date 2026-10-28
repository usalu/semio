"""🧱️ Register the independently closed direct primitive source entry."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
anchor='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-round.test.ts",'
new='  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-primitive.test.ts",'
if new not in source:
 assert anchor in source
 source=source.replace(anchor,anchor+"\n"+new,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==157,len(selected)
(ticket/"registered157.md").write_text("# Independently Closed Registry 157\n\nDirect primitive8 is registered after independent61-source closure, fresh203pass54320assert4files/strict10consumer0, all61hashesstationary during the coordinated replay. The actual Block/Cone/Cylinder body now serves both original CsgSolid and directly legal primitive callers, preserving genuine source identity and frames. This is a prerequisite factor, not actual Boolean dispatch. Continuing shared Boolean core changes begin only after auditreaderterminal, so subsequent stationarity is not claimed.\n\nProjection44/surface66 are already independently closed and registered. Lastfullgreen154 remains4871pass12skip0fail372425Bunassert4883tests534.80s0selectedtestdrift, with3explicit reviewed priorobligation supersessions and0unexplainedmissing. No full157run yet. Redactualkernel61/finite31/nested14 remain unregistered.\n\nNested six actualSTEPs independentlyparse/fullEXP0 and geometryvolumes agree with explicit neutral expectations, including the canonical operator-edit32. This inputcorroboration doesnot make our unsupported actualrenderer pass. Goal/ticketactive; fullIFC4x3/all-MVD, workspaceproducer/native/rounded/Boolean/rendererwork continues.\n",encoding="utf8")
print("[DEBUG] independent primitive8 registered; current157, lastfullgreen154")
