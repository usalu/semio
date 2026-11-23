"""📐 Prints ShootingDrawing field order for each schema facet."""
import json
import re
from pathlib import Path

base = Path("/home/deck/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")

def fields(text, name):
    match = re.search(rf"type {name} \{{([^}}]*)\}}", text)
    if not match:
        return None
    found = []
    for line in match.group(1).splitlines():
        line = line.split("#", 1)[0].strip()
        token = re.match(r"(\w+)", line)
        if token:
            found.append(token.group(1))
    return found

artifact = json.loads((base / "🔣️.json").read_text())
print("json", list(artifact["$defs"]["ShootingDrawing"]["properties"]))
for relative in ["🔗️.graphql", "📸️snapshot/🔗️.graphql", "🧬️mutations/🔗️.graphql", "🔺️diff/🔗️.graphql"]:
    print(relative, fields((base / relative).read_text(), "ShootingDrawing"))
