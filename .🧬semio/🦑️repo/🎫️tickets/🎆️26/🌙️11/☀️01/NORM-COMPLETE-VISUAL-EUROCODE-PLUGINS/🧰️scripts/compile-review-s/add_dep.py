"""🔧 Add a workspace dependency to a crate's Cargo.toml ([dependencies]) and to the artifact Cargo.lock entry. Usage: add_dep.py <Cargo.toml> <dep-name> [--lock <Cargo.lock>]"""
import re, sys, tomllib

toml, dep = sys.argv[1], sys.argv[2]
lock = sys.argv[sys.argv.index("--lock") + 1] if "--lock" in sys.argv else None
s = open(toml, encoding="utf-8").read()
t = tomllib.loads(s)
if dep in t.get("dependencies", {}):
    print("toml: already present")
else:
    m = re.search(r'^\[dependencies\]\n(.*?)(?=^\[|\Z)', s, re.S | re.M)
    lines = m.group(1).rstrip("\n").split("\n")
    new = f"{dep} = {{ workspace = true }}"
    idx = next((i for i, l in enumerate(lines) if l.split(" ")[0] > dep and not l.startswith("#")), len(lines))
    lines.insert(idx, new)
    s = s[:m.start(1)] + "\n".join(lines) + "\n" + ("\n" if m.group(1).endswith("\n\n") else "") + s[m.end(1):]
    tomllib.loads(s)
    open(toml, "w", encoding="utf-8").write(s)
    print("toml: added")
if lock:
    name = t["package"]["name"]
    L = open(lock, encoding="utf-8").read()
    m = re.search(r'\[\[package\]\]\nname = "%s"\nversion = "[^"]*"\ndependencies = \[\n(.*?)\]\n' % re.escape(name), L, re.S)
    if not m: print("lock: package entry not found"); sys.exit(1)
    items = re.findall(r'"([^"]+)"', m.group(1))
    if dep in items: print("lock: already present")
    else:
        items = sorted(set(items + [dep]))
        L = L[:m.start(1)] + "".join(f' "{i}",\n' for i in items) + L[m.end(1):]
        tomllib.loads(L)
        open(lock, "w", encoding="utf-8").write(L)
        print("lock: added")
