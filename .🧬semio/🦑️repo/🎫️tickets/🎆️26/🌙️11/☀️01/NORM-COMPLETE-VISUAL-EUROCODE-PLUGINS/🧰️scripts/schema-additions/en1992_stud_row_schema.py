"""en1992 `members[].studRows[]` (EN 1992-1-1 6.4.5, Figure 6.22): record `StudRow { id, angle, count, spacingFirst, spacingRadial, diameter }` replaces `StudRing` and the member property `studRows` replaces `studRings` in the snapshot and artifact JSON (text surgery, layout preserved, idempotent); twins and structs come from `schema twins generate`."""
import re
ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/"
def length(en, de, step=0.005):
    return '{"type": "number", "x-semio-ui": {"label": {"en": "%s", "de": "%s"}, "widget": "stepper", "unit": "m", "displayUnit": "mm", "displayFactor": 1000, "precision": 1, "step": %s}}' % (en, de, step)
PROPS = '''    "id": {"type": "string", "x-semio-ui": {"label": {"en": "Id", "de": "Id"}}},
    "angle": {"type": "number", "x-semio-ui": {"label": {"en": "Rail angle from the column x axis", "de": "Schienenwinkel zur Stützenachse x"}, "widget": "stepper", "unit": "rad", "displayUnit": "°", "displayFactor": 57.29577951308232, "precision": 1, "step": 0.0872664625997165}},
    "count": {"type": "integer", "minimum": 0, "x-semio-ui": {"label": {"en": "Studs per row", "de": "Dübel je Reihe"}, "widget": "stepper", "precision": 0, "step": 1}, "maximum": 4294967295},
    "spacingFirst": %s,
    "spacingRadial": %s,
    "diameter": %s
''' % (length("First stud distance s₀ from the column face", "Abstand des ersten Dübels s₀ vom Stützenrand"), length("Radial stud spacing s_r", "Radialer Dübelabstand s_r"), length("Bar diameter Ø", "Stabdurchmesser Ø"))
for name in ("🔣️.json", "📸️snapshot/🔣️.json"):
    text = open(ANY + name, encoding="utf8").read()
    if '"StudRow"' in text:
        print("present", name)
        continue
    m = re.search(r'  "StudRing": \{.*?\n  \},\n', text, re.S)
    block = m.group(0)
    new = block.replace('"StudRing"', '"StudRow"').replace('"title": "StudRing"', '"title": "StudRow"')
    new = re.sub(r'"required": \[[^\]]*\]', '"required": ["id", "angle", "count", "spacingFirst", "spacingRadial", "diameter"]', new, 1)
    new = re.sub(r'   "properties": \{\n.*?\n   \},\n', '   "properties": {\n' + PROPS.replace("\\", "\\\\") + '   },\n', new, 1, flags=re.S)
    new = new.replace("StudRing of an explicit geometry collection of a member (lengths in m).", "Shear stud row (rail) of the punching zone of a member: studs at s0 + k sr from the column face along the ray at the angle (lengths in m, angle in rad).")
    text = text.replace(block, new)
    text = text.replace('"studRings"', '"studRows"')
    line = re.search(r'    "studRows": \{"type": "array", "items": \{"\$ref": "#/\$defs/StudRing"\}, "x-semio-ui": \{"label": \{"en": "Stud rails \(rings\)", "de": "Dübelleisten \(Ringe\)"\}\}', text)
    assert line, name
    text = text.replace(line.group(0), '    "studRows": {"type": "array", "items": {"$ref": "#/$defs/StudRow"}, "x-semio-ui": {"label": {"en": "Shear stud rows (rails, 6.4.5)", "de": "Dübelreihen (Schienen, 6.4.5)"}}')
    open(ANY + name, "w", encoding="utf8").write(text)
    print("edited", name)
