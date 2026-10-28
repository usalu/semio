"""📉️ Add neutral live-buffer retirement without resetting historical peak."""
from pathlib import Path
import json

ticket=Path(__file__).parent
tests=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests"
path=tests/"workspace-cases.json"
value=json.loads(path.read_text(encoding="utf8"))
row=next(row for row in value["cases"]if row["id"]=="grow-before-backing-copy")
assert row["steps"][-2]==["release","first",0,6,15]
row["steps"].insert(-2,["shrink","first",3,12,15])
path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
path=tests/"workspace.schema.json"
value=json.loads(path.read_text(encoding="utf8"))
operations=value["properties"]["cases"]["items"]["properties"]["steps"]["items"]["oneOf"][0]["items"][0]["enum"]
assert "shrink"not in operations
operations.append("shrink")
path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] existing neutral buffer program now exercises explicit shrink")
