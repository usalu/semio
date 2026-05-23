"""Fix read-after-writes step in comprehensive fixture JSON."""
import json
from pathlib import Path

QUERY = (
    "query($designId: ID!, $tagId: ID!) { store { wip { theKit { kit { name"
    " tag(id: $tagId) { name } concepts { edges { node { name } } }"
    " qualities { edges { node { key } } }"
    " design(id: $designId) { pieces { edges { node { id } } } } } }"
    " alternatives { edges { node { id name } } } } } }"
)

assert QUERY.count("{") == QUERY.count("}"), (QUERY.count("{"), QUERY.count("}"))

path = Path(r"c:\git\semio\semio\assets\semio\kit-store.comprehensive.semio.json")
fixture = json.loads(path.read_text(encoding="utf-8"))
for step in fixture["steps"]:
    if step.get("id") == "read-after-writes":
        step["query"] = QUERY
        break
else:
    raise SystemExit("read-after-writes step not found")
path.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("patched", path)
