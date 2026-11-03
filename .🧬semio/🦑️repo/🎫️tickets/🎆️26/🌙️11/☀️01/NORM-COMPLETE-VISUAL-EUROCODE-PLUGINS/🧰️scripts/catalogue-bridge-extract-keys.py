import re, sys, pathlib, collections
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
CODES = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1996": "🪨️en1996", "en1997": "🌍️en1997"}
out = pathlib.Path(sys.argv[1]); out.mkdir(parents=True, exist_ok=True)
for code, folder in CODES.items():
    entries = {}
    for path in (ROOT / folder).rglob("*.rs"):
        if "target" in path.parts: continue
        text = path.read_text(encoding="utf-8")
        for m in re.finditer(r'Entry \{ id: "([^"]+)",[^}]*?inputs: &\[([^\]]*)\]', text):
            entries[m.group(1)] = re.findall(r'"([A-Za-z0-9_]+)"', m.group(2))
    keys = collections.Counter(k for ks in entries.values() for k in ks)
    (out / f"{code}-keys.tsv").write_text("".join(f"{k}\t{n}\n" for k, n in sorted(keys.items())), encoding="utf-8")
    print(code, "entries", len(entries), "distinct input keys", len(keys))
