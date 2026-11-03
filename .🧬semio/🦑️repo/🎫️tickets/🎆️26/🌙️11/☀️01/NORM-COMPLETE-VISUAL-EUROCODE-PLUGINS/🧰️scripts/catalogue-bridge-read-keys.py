import re, sys, pathlib
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
CODES = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1996": "🪨️en1996", "en1997": "🌍️en1997"}
code = sys.argv[1]
decl, read = {}, {}
for path in (ROOT / CODES[code]).rglob("*.rs"):
    if "target" in path.parts: continue
    t = path.read_text(encoding="utf-8")
    for m in re.finditer(r'Entry \{ id: "([^"]+)",[^}]*?en: "([^"]*)",[^}]*?inputs: &\[([^\]]*)\]', t):
        decl[m.group(1)] = (m.group(2), re.findall(r'"([A-Za-z0-9_]+)"', m.group(3)))
    for m in re.finditer(r'entry_of\("([^"]+)", "[^"]*", "[^"]*", "([^"]*)", "[^"]*", &\[([^\]]*)\]', t):
        decl[m.group(1)] = (m.group(2), re.findall(r'"([A-Za-z0-9_]+)"', m.group(3)))
    for m in re.finditer(r'\b(?:c|case)\.(n|o|opt|s|so|v|vec|vo|list|l|lo)\("([A-Za-z0-9_]+)"', t):
        read.setdefault(m.group(2), set()).add(path.name + ":" + m.group(1))
    for m in re.finditer(r'\.(num|text|vec)\.get\("([A-Za-z0-9_]+)"', t):
        read.setdefault(m.group(2), set()).add(path.name + ":" + m.group(1))
dk = {k for _, ks in decl.values() for k in ks}
if len(sys.argv) > 2 and sys.argv[2] == "entries":
    for k, (t, ks) in decl.items(): print(k, "|", t, "|", " ".join(ks))
else:
    print("declared", len(dk), "read-only extras", len(set(read) - dk))
    print("EXTRAS:", " ".join(sorted(set(read) - dk)))
    print("DECLARED:", " ".join(sorted(dk)))
