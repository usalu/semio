"""🧾️ Preserve strict schema typing in the conditional INTEGER carrier law."""
from pathlib import Path
import json

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/rounded-rectangle.schema.json"
schema=json.loads(path.read_text(encoding="utf8"))
condition=schema["properties"]["cases"]["items"]["allOf"][0]["if"]["properties"]["carriers"]
assert "type"not in condition
condition["type"]="object"
path.write_text(json.dumps(schema,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] conditional carrier schema explicitlyobject; strictoptions unchanged")
