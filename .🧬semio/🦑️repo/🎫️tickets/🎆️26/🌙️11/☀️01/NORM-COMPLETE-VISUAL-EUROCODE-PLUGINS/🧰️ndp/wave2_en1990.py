import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
from rewrite_lib import *
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990")
TABLE = ROOT / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📜️ndp-tables/🦀️.rs"
src = TABLE.read_text()
rows = re.findall(r'ndp_row\("([^"]+)", "([^"]+)", "([^"]*)", ([-\d.]+), ([-\d.]+)\)', src)
assert len(rows) > 60, len(rows)
d = load()
for sym, cl, unit, en, de in rows:
    i = f"ndp.en1990.{sym}"
    put(d, "1990", sym, cl, sym, float(en), float(de), "" if unit == "-" else unit, f"{sym} EN {en} / DE {de} as used by the artifact", part="")
save(d)
print(len(rows))
changed = 0
for f in ROOT.rglob("*.rs"):
    if f.name != "🦀️.rs" or "ndp-tables" in str(f): continue
    s = f.read_text(); o = s
    def sub(inner):
        return re.sub(r'"((?!ndp\.)[^"]*)"', lambda m: '"ndp.en1990.%s"' % m.group(1), inner)
    out = []; pos = 0
    for m in re.finditer(r'(?<![\w.])(?:ndp|ndp_lookup)\(', s):
        if m.start() < pos: continue
        open_i = m.end() - 1; close = match_close(s, open_i)
        inner = s[open_i + 1:close]
        parts = split_top(inner)
        if len(parts) < 2: continue
        a, b = parts[1]
        new = inner[:a] + sub(inner[a:b]) + inner[b:]
        out.append(s[pos:open_i + 1]); out.append(new); pos = close
    out.append(s[pos:]); s = "".join(out)
    if s != o: f.write_text(s); changed += 1; print(str(f)[-60:])
print(changed)
