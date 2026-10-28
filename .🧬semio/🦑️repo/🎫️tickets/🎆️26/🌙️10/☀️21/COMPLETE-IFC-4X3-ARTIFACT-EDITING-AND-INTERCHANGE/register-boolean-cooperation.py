from pathlib import Path
import re
ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts"
source=path.read_text(encoding="utf8")
anchor='  "📐️geometry/🧊️boolean/🧪️tests/admission.test.ts",'
names=["control-authority.test.ts","cooperative.test.ts","control-receiver.test.ts","control-policy.test.ts"]
addition="\n".join('  "📐️geometry/🧊️boolean/🧪️tests/'+name+'",'for name in names)
if addition not in source:
 assert anchor in source
 source=source.replace(anchor,anchor+"\n"+addition,1)
path.write_text(source,encoding="utf8")
selected=re.findall(r'"(🏅️standards/🔖️4x3/[^"\r\n]+|🔎️details/[^"\r\n]+|📚️catalog/🔎️query/[^"\r\n]+|📐️geometry/(?:🕳️rings|🧊️boolean|⭕️circle)/[^"\r\n]+)"',source)
assert len(selected)==145,len(selected)
print("[DEBUG] registered four independently closed Boolean suites; actual selected145")

