"""en1992 `members[].openings[]`: every committed snapshot document gets `"openings": []` after each member's `studRings` array (layout preserved), the DSL assets get the `openings` table column. Idempotent."""
import json, pathlib, re
ANY = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
KEY = re.compile(r'"studRings": *\[')
def sweep(text):
    out, pos = [], 0
    for m in KEY.finditer(text):
        depth, i = 0, m.end() - 1
        while True:
            depth += (text[i] == "[") - (text[i] == "]")
            if depth == 0:
                break
            i += 1
        line = text[text.rfind("\n", 0, m.start()) + 1:m.start()]
        glue = ",\n" + line + '"openings": []' if line.strip() == "" and "\n" in text[:m.start()] else ', "openings": []'
        out.append(text[pos:i + 1] + glue)
        pos = i + 1
    return "".join(out) + text[pos:]
count = 0
for path in (ANY / "🧪️tests/🧫️fixtures").rglob("*.json"):
    text = path.read_text(encoding="utf8")
    if '"studRings": [' not in text and '"studRings":[' not in text or '"openings"' in text:
        continue
    new = sweep(text)
    json.loads(new)
    path.write_text(new, encoding="utf8")
    count += 1
path = ANY / "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json"
text = path.read_text(encoding="utf8")
if '"openings"' not in text and '"studRings"' in text:
    new = sweep(text)
    json.loads(new)
    path.write_text(new, encoding="utf8")
    count += 1
for path in (ANY / "🖼️assets").rglob("*.dsl.semio"):
    lines = path.read_text(encoding="utf8").split("\n")
    if "stud-rings:TABLE]" not in "\n".join(lines) or "openings:TABLE" in "\n".join(lines):
        continue
    block = False
    for k, line in enumerate(lines):
        if line.startswith("members [") and "stud-rings:TABLE]" in line:
            lines[k] = line.replace("stud-rings:TABLE]", "stud-rings:TABLE openings:TABLE]")
            block = True
        elif block and line.startswith("}"):
            block = False
        elif block and line.startswith("  ") and line.rstrip().endswith("]"):
            lines[k] = line.rstrip() + " [ ]"
    path.write_text("\n".join(lines), encoding="utf8")
    count += 1
print("files", count)
