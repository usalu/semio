"""🧮️ Register independently closed logical workspace permission laws."""
from pathlib import Path
import re

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
anchor='  "📐️geometry/🧊️boolean/🧪️tests/session.test.ts",'
new='  "📐️geometry/🧊️boolean/🧪️tests/workspace.test.ts",'
if new not in source:
 assert anchor in source
 source=source.replace(anchor,anchor+"\n"+new,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==154,len(selected)
status="Registered154 suites after independently closed workspace36, arithmetic/isolation50, cumulativeBoolean32, weights32 and fullValue37 additions. Last full green remains135. Failed145 originals and independently closed axis/TFS21 correction remain retained; no full154 run yet. One intentional SELECT test-name supersession is documented. Logical workspace permission is not yet integrated into actual kernel allocation; IFC finite Boolean adapter and native/runtime/renderer proof remain open. Goal/ticket active/incomplete. See registered154.md."
(ticket/"registered154.md").write_text("# Registered 154-Suite Scope\n\n"+status+"\n\nWorkspace independent58-source audit: actual830pass/149633Bun assertions10files, strict10consumers0, all58hashes stationary. New36 include six independent actualSQLite integer programs, safe-capacity conservation, private provenance, atomic refusal, valid failed-scope retirement and terminal close. This certifies the permission owner only; no reserve-before-allocation kernel/IFC producer implementation follows from its unit tests.\n\nArithmetic39 and ValueTS6 independent closures retain their own qualification. Native Value source is approaching a coherent freeze before the next full selection. record-ifc-checkpoint.py will archive terminal results and compare135 with exactly one explicitly named obligation supersession, without overwriting failed145 evidence.\n",encoding="utf8")
for name in ["current-state.md","registered-test-scope.md","exchange-profile-current-state.md","post135-current-state.md"]:
 path=ticket/name
 title,separator,body=path.read_text(encoding="utf8").partition("\n\n")
 path.write_text(title+separator+status+"\n\n"+body,encoding="utf8")
print("[DEBUG] workspace independently registered; actual154, no full154 result")
