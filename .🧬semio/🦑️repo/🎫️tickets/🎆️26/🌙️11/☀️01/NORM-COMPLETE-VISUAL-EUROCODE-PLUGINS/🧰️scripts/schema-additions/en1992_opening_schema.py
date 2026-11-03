"""en1992 `members[].openings[]` (EN 1992-1-1 6.4.2(3), Figure 6.14): record `Opening` and the member property in the snapshot and artifact JSON (text surgery, layout preserved, idempotent); twins and structs come from `schema twins generate`."""
import re
ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/"
def length(en, de, step=0.01):
    return '{"type": "number", "x-semio-ui": {"label": {"en": "%s", "de": "%s"}, "widget": "stepper", "unit": "m", "displayUnit": "mm", "displayFactor": 1000, "precision": 0, "step": %s}}' % (en, de, step)
REC = '''  "Opening": {
   "title": "Opening",
   "type": "object",
   "additionalProperties": false,
   "required": ["id", "x", "y", "width", "depth"],
   "properties": {
    "id": {"type": "string", "x-semio-ui": {"label": {"en": "Id", "de": "Id"}}},
    "x": %s,
    "y": %s,
    "width": %s,
    "depth": %s
   },
   "x-semio-formats": ["🔣️jsonschema", "🟦️typescript", "🔗️graphql", "🛰️protobuf"]%s
  },
''' % (length("Opening centre x (column frame)", "Öffnungsmitte x (Stützenachsen)"), length("Opening centre y (column frame)", "Öffnungsmitte y (Stützenachsen)"), length("Opening width (along x)", "Öffnungsbreite (in x)"), length("Opening depth (along y)", "Öffnungstiefe (in y)"), "%s")
PROP = '    "openings": {"type": "array", "items": {"$ref": "#/$defs/Opening"}, "x-semio-ui": {"label": {"en": "Openings near the column (6.4.2(3))", "de": "Öffnungen neben der Stütze (6.4.2(3))"}}%s}'
for name in ("🔣️.json", "📸️snapshot/🔣️.json"):
    text = open(ANY + name, encoding="utf8").read()
    if '"Opening": {' in text:
        print("present", name)
        continue
    snap = name.startswith("📸")
    m = re.search(r'  "StudRing": \{.*?\n  \},\n', text, re.S)
    rust = re.search(r',\n   "x-semio-rust": \{.*?\n   \}\n  \},\n', m.group(0), re.S)
    ext = ""
    if rust:
        ext = rust.group(0)[1:-len("\n  },\n")].replace("StudRing of an explicit geometry collection of a member (lengths in m).", "Opening near the column of a member (centre and size in the column frame, m).")
        ext = "," + ext
    rec = REC % ext
    text = text.replace(m.group(0), m.group(0) + rec)
    line = re.search(r'    "studRings": \{[^\n]*\}\}(, "x-semio-dsl": \{"table": true\})?\}\n', text)
    assert line
    prop = PROP % (', "x-semio-dsl": {"table": true}' if line.group(1) else "")
    text = text.replace(line.group(0), line.group(0)[:-1] + ",\n" + prop + "\n")
    assert text.count('    "studRings"\n   ],') == 1
    text = text.replace('    "studRings"\n   ],', '    "studRings",\n    "openings"\n   ],')
    open(ANY + name, "w", encoding="utf8").write(text)
    print("edited", name)
