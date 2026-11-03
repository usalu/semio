import json, pathlib, re, sys
code = sys.argv[1]
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
CODES = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1996": "🪨️en1996", "en1997": "🌍️en1997"}
rows = json.load(open(pathlib.Path(sys.argv[2]) / "🔣️.json", encoding="utf-8"))["inputs"]
ents = {}
for p in (ROOT / CODES[code]).rglob("*.rs"):
    if "target" in p.parts: continue
    for m in re.finditer(r'Entry \{ id: "([^"]+)",[^}]*?inputs: &\[([^\]]*)\]', p.read_text(encoding="utf-8")):
        ents[m.group(1)] = re.findall(r'"([A-Za-z0-9_]+)"', m.group(2))
    for m in re.finditer(r'entry_of\("([^"]+)", "[^"]*", "[^"]*", "(?:[^"\\]|\\.)*", "(?:[^"\\]|\\.)*", &\[([^\]]*)\]', p.read_text(encoding="utf-8")):
        ents[m.group(1)] = re.findall(r'"([A-Za-z0-9_]+)"', m.group(2))
miss = [(e, k) for e, ks in ents.items() for k in ks if not any(r["key"] == k and (not r["scope"] or e in r["scope"].split(",")) for r in rows)]
print(code, len(ents), "entries", len(rows), "rows; missing", miss)
