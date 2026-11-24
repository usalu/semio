"""🧪️ R5: classifies the snapshot (kind, crate, emit, target, origin) and traces every stale verdict to the edited file at the root of its chain; flags stale entries that were read (hit) after their root file changed, the signature of a false stale.

Usage: analyse.py <snapshot.json> <oracle.json> <out.json>
"""
import collections, json, os, re, sys

snap, oracle = json.load(open(sys.argv[1])), json.load(open(sys.argv[2]))
TREE = "/Users/ueli/Documents/semio"
entries = {e["key"]: e for e in snap["entries"]}
verdict, stale = oracle["verdict"], oracle["stale"]


def decl(e):
    d = (e["meta"] or {}).get("declaration")
    return d if d and d.get("version") == 2 else None


def emit(e):
    o = " ".join(e["outputs"])
    if ".rlib" in o or ".dylib" in o or ".so" in o or ".wasm" in o or re.search(r"out/[^ .]+(@@semio-extra@@)?$", o):
        return "link"
    if ".rmeta" in o:
        return "metadata"
    return "other"


def crate(e):
    for o in e["outputs"]:
        m = re.match(r"(?:lib)?([A-Za-z0-9_]+?)(?:-[0-9a-f]{16})?(?:@@semio-extra@@)?\.(?:rmeta|rlib|d|dylib|so|wasm)", o)
        if m:
            return m.group(1)
    return None


def origin(e):
    d = decl(e)
    if not d or not d["sources"]:
        return "no-sources"
    t = all(snap["tree"].get(r["path"]) == r["sha256"] for r in d["sources"])
    p = all(snap["play"].get(r["path"]) == r["sha256"] for r in d["sources"])
    return "both" if t and p else "tree" if t else "play" if p else "neither"


def root(k, seen=()):
    s = stale.get(k)
    if not s:
        return None
    if s[0] != "extern" or k in seen:
        return (k, s[0], s[1])
    return root(s[1], seen + (k,))


rows = {}
for k, e in entries.items():
    d = decl(e)
    meta = e["meta"] or {}
    r = root(k) if k in stale else None
    edit_ms = None
    if r and r[1] == "edited":
        try:
            edit_ms = os.stat(os.path.join(TREE, *r[2].split("/"))).st_mtime * 1000
        except OSError:
            pass
    rows[k] = {
        "verdict": verdict.get(k),
        "kind": (d or {}).get("kind") or ("undeclared-v%s" % ((meta.get("declaration") or {}).get("version")) if meta.get("declaration") else "undeclared"),
        "crate": crate(e), "emit": emit(e), "origin": origin(e), "sources": len((d or {}).get("sources", [])),
        "storedAt": meta.get("storedAt"), "accessed": e["mtime"], "size": e["size"], "refs": len(meta.get("refs", [])),
        "root": r, "rootEditMs": edit_ms,
        "readAfterRoot": bool(edit_ms and e["mtime"] > max(edit_ms, (meta.get("storedAt") or 0)) + 60_000),
        "manifests": e["manifests"], "ns": k.split("/")[0],
    }
json.dump(rows, open(sys.argv[3], "w"))
by = collections.Counter((r["verdict"], r["kind"], r["emit"]) for r in rows.values())
print("verdict x kind x emit")
for (v, kd, em), n in sorted(by.items(), key=lambda x: -x[1]):
    print(f"  {n:6d} {v} {kd} {em}")
print("origin of stale:", collections.Counter((r["verdict"], r["origin"]) for r in rows.values() if r["verdict"] and r["verdict"].startswith("stale")))
print("root files of stale (top 25):")
for (p, n) in collections.Counter(r["root"][2] for r in rows.values() if r["root"] and r["verdict"].startswith("stale")).most_common(25):
    print(f"  {n:6d} {p}")
print("stale read after their root changed:", collections.Counter((r["verdict"], r["emit"]) for r in rows.values() if r["readAfterRoot"] and r["verdict"].startswith("stale")))
