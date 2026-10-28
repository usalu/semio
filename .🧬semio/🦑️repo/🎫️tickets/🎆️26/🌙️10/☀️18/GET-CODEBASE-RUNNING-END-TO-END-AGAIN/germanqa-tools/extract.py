import json, subprocess, sys, os, re
root = sys.argv[1]
out = sys.argv[2]
roots = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules/🏃️process"]
files = subprocess.run(["git", "-C", root, "ls-files", "-co", "--exclude-standard", "-z", "--"] + roots, capture_output=True, check=True).stdout.decode().split("\0")
files = [f for f in files if f.endswith(".json") and os.path.exists(os.path.join(root, f))]
rows = []
def flat(t, base, into):
    for k, v in t.items():
        if isinstance(v, str): into[base + "/" + k] = v
        elif isinstance(v, dict): flat(v, base + "/" + k, into)
        elif isinstance(v, list):
            for i, x in enumerate(v):
                if isinstance(x, str): into[f"{base}/{k}/{i}"] = x
def walk(v, path, f):
    if isinstance(v, list):
        for i, x in enumerate(v): walk(x, f"{path}/{i}", f)
        return
    if not isinstance(v, dict): return
    keys = list(v)
    if "en" in keys and "de" in keys and set(keys) <= {"en", "de"}:
        if all(isinstance(x, str) for x in v.values()):
            rows.append((f, path, v["en"], v["de"])); return
        if all(isinstance(x, dict) for x in v.values()):
            e, d = {}, {}
            flat(v["en"], "", e); flat(v["de"], "", d)
            for k in sorted(set(e) | set(d)):
                rows.append((f, path + k, e.get(k), d.get(k)))
            return
        if all(isinstance(x, list) for x in v.values()) :
            e, d = {}, {}
            flat({"x": v["en"]}, "", e) if False else None
            for i, x in enumerate(v["en"]): e[str(i)] = x
            for i, x in enumerate(v["de"]): d[str(i)] = x
            for k in sorted(set(e) | set(d), key=lambda s: int(s)):
                rows.append((f, path + "/" + k, e.get(k), d.get(k)))
            return
    for k, x in v.items(): walk(x, f"{path}/{k}", f)
for f in files:
    try: doc = json.load(open(os.path.join(root, f), encoding="utf-8"))
    except Exception: continue
    walk(doc, "", f)
json.dump([{"file": f, "path": p, "en": e, "de": d} for f, p, e, d in rows], open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=0)
print(len(files), "json files scanned,", len(rows), "pairs,", len({r[0] for r in rows}), "files with pairs")
