"""📦️ R5: for registry/host units (no workspace sources) that are live or unreached, reads the unit manifest blob (read-only) for the registry crate name+version and checks it against the coordinator's Cargo.lock.

Usage: registry.py <snapshot.json> <rows.json>
"""
import collections, json, os, re, sys
rows, snap = json.load(open(sys.argv[2])), json.load(open(sys.argv[1]))
C = os.path.expanduser("~/.cache/semio/cas/v1/o")
pk = set(re.findall(r'name = "([^"]+)"\nversion = "([^"]+)"', open("/Users/ueli/Documents/semio/Cargo.lock").read()))
names = {n for n, _ in pk}
c, ex = collections.Counter(), []
for e in snap["entries"]:
    r = rows[e["key"]]
    if not r["verdict"].startswith(("unreached", "live", "stale")) or r["kind"] != "rust-unit" or r["sources"] > 0:
        continue
    ver, found = None, False
    for h in e["meta"]["refs"]:
        p = os.path.join(C, h[:2], h[2:])
        try:
            if os.path.getsize(p) > 400000:
                continue
            j = json.loads(open(p, "rb").read())
        except Exception:
            continue
        if isinstance(j, dict) and "dependencies" in j:
            found = True
            for d in j["dependencies"]:
                path = d if isinstance(d, str) else d.get("path", "") if isinstance(d, dict) else ""
                m = re.search(r"registry/src/[^/]+/([A-Za-z0-9_\-]+?)-(\d+\.\d+\.\d+[^/]*)/", path)
                if m:
                    ver = (m.group(1), m.group(2)); break
            break
    cr = r["crate"] or ""
    nm = cr in names or cr.replace("_", "-") in names
    tag = "lock:name+version" if ver and ver in pk else f"NOT-in-lock:{ver}" if ver else ("manifest-blob-gone" if not found else "no-registry-path") + (":name-in-lock" if nm else ":name-not-in-lock")
    c[(r["verdict"].split(":")[0], tag)] += 1
    if r["verdict"].startswith("unreached") and len(ex) < 6:
        ex.append((e["key"], cr, ver))
for k, n in c.most_common():
    print(n, k)
print(ex)
