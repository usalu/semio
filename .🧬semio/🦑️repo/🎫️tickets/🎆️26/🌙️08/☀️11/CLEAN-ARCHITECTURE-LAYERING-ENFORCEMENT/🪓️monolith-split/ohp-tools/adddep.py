"""➕ Adds `name = spec` to the [dependencies] table of a Cargo.toml in sorted position (refuses when already declared in that table).
usage: adddep.py <Cargo.toml> <name> [<spec>]   (spec defaults to `{ workspace = true }`)"""
import re, sys
p, name = sys.argv[1], sys.argv[2]
spec = sys.argv[3] if len(sys.argv) > 3 else "{ workspace = true }"
t = open(p, encoding="utf-8").read()
m = re.search(r"^\[dependencies\]\n", t, re.M)
assert m, "no [dependencies]"
s = m.end()
n = re.search(r"^\[", t[s:], re.M)
e = s + n.start() if n else len(t)
body = t[s:e]
lines = body.split("\n")
keys = [(i, re.match(r"^([A-Za-z0-9_.\-\"]+)\s*[.=]", l).group(1).split(".")[0].strip('"')) for i, l in enumerate(lines) if re.match(r"^[A-Za-z0-9_.\-\"]+\s*[.=]", l)]
if any(k == name for _, k in keys):
    print("already declared:", name); sys.exit(0)
pos = None
for i, k in keys:
    if k > name:
        pos = i; break
new = f"{name} = {spec}"
if pos is None:
    last = max(i for i, _ in keys) + 1 if keys else 0
    lines.insert(last, new)
else:
    lines.insert(pos, new)
open(p, "w", encoding="utf-8").write(t[:s] + "\n".join(lines) + t[e:])
print("added", name, "to", p.split("artifacts/")[-1][-60:])
