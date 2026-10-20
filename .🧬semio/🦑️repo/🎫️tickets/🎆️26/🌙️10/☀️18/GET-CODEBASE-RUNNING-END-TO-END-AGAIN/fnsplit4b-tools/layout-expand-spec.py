import json, re, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
R = ROOT + "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/"
F = R + "📖️expand-document/🦀️.rs"
ENUM = R + "🌳️typed-json-nodes-and-scalars/🦀️.rs"
text = open(ENUM, encoding="utf8").read()
body = text[text.index("pub enum TypedJsonNode {"):]
body = body[:body.index("\n}\n")]
variants = {}
for line in body.split("\n")[1:]:
    line = line.strip().rstrip(",")
    m = re.match(r"(\w+)\s*\{(.*)\}$", line)
    if m:
        fields = {}
        depth = 0; cur = ""; parts = []
        for ch in m.group(2):
            if ch in "<([": depth += 1
            if ch in ">)]": depth -= 1
            if ch == "," and depth == 0: parts.append(cur); cur = ""
            else: cur += ch
        if cur.strip(): parts.append(cur)
        for p in parts:
            k, v = p.split(":", 1); fields[k.strip()] = v.strip()
        variants[m.group(1)] = ("struct", fields)
        continue
    m = re.match(r"(\w+)\((.*)\)$", line)
    if m: variants[m.group(1)] = ("tuple", [m.group(2).strip()]); continue
    m = re.match(r"(\w+)$", line)
    if m: variants[m.group(1)] = ("unit", None)
G = {
 "doc": child("📄", "document-and-page-nodes", "Document and page nodes: the document root, the top level arrays, the styles, stories, links, parents, spreads, pages and string arrays of the layout export JSON expansion."),
 "geo": child("🖼", "layer-frame-and-page-geometry-nodes", "Layer, frame and page geometry nodes: the layers, frames, bounds, rectangles, insets, margins, guides and overrides of the layout export JSON expansion."),
 "drw": child("🎨", "drawing-nodes", "Drawing nodes: the drawing canvas, styles, layers, nodes, children, segments, points and byte streams of the layout export JSON expansion."),
 "pkg": child("📦", "package-and-manifest-nodes", "Package and manifest nodes: the artifact references, the referenced model, the link pins, the blob references and the manifest of the layout export JSON expansion."),
}
EMOJIS = list("🧾🧮🗂🗃🗄🗒🗓📊📈📉🔖🏷🔗🧷📌📍🧭🧩🧱🧲🧪🧰🧶🧵🪡🪢🪵🪨🪙🪜🪝🪞🪟🧿🧸")
arms_src = [
 (8, "Document", "doc"), (54, "TopArray", "doc"), (81, "Paragraph", "doc"), (103, "Character", "doc"), (144, "StoryRun", "doc"), (158, "Link", "doc"), (182, "Parent", "doc"), (211, "Page", "doc"), (243, "StringArray", "doc"),
 (272, "Layer", "geo"), (298, "Frame", "geo"), (353, "Bounds", "geo"), (369, "Rect", "geo"), (383, "TextInset", "geo"), (400, "Margins", "geo"), (438, "Override", "geo"), (452, "OverrideBounds", "geo"),
 (530, "DrawStyle", "drw"), (559, "DrawLayer", "drw"), (574, "DrawNode", "drw"), (619, "DrawChildren", "drw"), (638, "DrawSegments", "drw"), (656, "DrawSegment", "drw"), (718, "DrawBytes", "drw"),
 (749, "BlobRef", "pkg"), (764, "MissingLinks", "pkg"), (794, "ManifestLink", "pkg"),
]
def snake(s): return re.sub(r"(?<!^)([A-Z])", lambda m: "_" + m.group(1), s).lower()
arms = []
for i, (line, name, g) in enumerate(arms_src):
    kind, fields = variants[name]
    binds = {}
    if kind == "struct": binds = dict(fields)
    elif kind == "tuple":
        pass
    arms.append({"line": line, "name": "expand_" + snake(name), "doc": "%s Expands the `%s` node into its output fragments." % (EMOJIS[i], name), "child": G[g], "allow_returns": True, "binds": binds, "_tuple": fields if kind == "tuple" else None})
src = open(F, encoding="utf8").read().split("\n")
for a in arms:
    if a["_tuple"]:
        pat = src[a["line"] - 1]
        m = re.search(r"TypedJsonNode::\w+\((\w+)\)", pat)
        a["binds"] = {m.group(1): a["_tuple"][0]}
    del a["_tuple"]
spec = {"file": F, "function": "expand", "match_line": 7, "types": {"snapshot": "&LayoutSnapshot"}, "ret": "Result<(), String>", "ensure_semicolon": True, "fall_tail": "Ok(())",
        "method": {"self_param": "&mut self", "impl_header": "impl TypedJsonCursor", "call": "self.{name}({args})?", "vis": "pub(super)"}, "arms": arms}
dump(spec, sys.argv[1])
