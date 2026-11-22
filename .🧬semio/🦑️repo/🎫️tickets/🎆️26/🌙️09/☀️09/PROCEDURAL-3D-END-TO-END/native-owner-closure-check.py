"""🧭️ Walks the Cargo path-dependency closure of build owners and lists roots that have no row in 📋️build-owners.json."""
import json, os, sys, tomllib
root = os.getcwd()
owners = json.load(open("📋️build-owners.json", encoding="utf8"))["owners"]
by_root = {r: o["name"] for o in owners for r in [o["root"], *o.get("nativeRoots", [])]}
def deps(crate):
    path = os.path.join(root, crate, "Cargo.toml")
    if not os.path.exists(path): return None
    data = tomllib.load(open(path, "rb"))
    tables = [data.get("dependencies", {}), data.get("dev-dependencies", {}), data.get("build-dependencies", {})]
    tables += [t.get(k, {}) for t in data.get("target", {}).values() for k in ("dependencies", "dev-dependencies", "build-dependencies")]
    out = []
    for table in tables:
        for spec in table.values():
            if isinstance(spec, dict) and "path" in spec:
                out.append(os.path.relpath(os.path.normpath(os.path.join(root, crate, spec["path"])), root))
    return out
missing, seen = {}, set()
names = sys.argv[1:] or [o["name"] for o in owners]
starts = [o["root"] for o in owners if o["name"] in names]
stack = [(s, s) for s in starts]
while stack:
    crate, origin = stack.pop()
    if crate in seen: continue
    seen.add(crate)
    found = deps(crate)
    if found is None: continue
    for dep in found:
        if dep not in by_root: missing.setdefault(dep, set()).add(crate)
        stack.append((dep, origin))
for dep, users in sorted(missing.items()): print("MISSING", dep, "<-", sorted(users)[:3])
print("checked", len(seen), "crates,", len(missing), "unowned roots")
