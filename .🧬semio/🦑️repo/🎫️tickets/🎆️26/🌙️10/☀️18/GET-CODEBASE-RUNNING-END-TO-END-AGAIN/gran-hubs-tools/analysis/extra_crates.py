"""➕ Standalone crates referenced by path but absent from the audit crate list (each is its own workspace): parses them (Bun TOML) and writes an extended graph `🗑️generated/gran-hubs/crates2.json`, `manifests.json` and `manifests-roots.json` so that census, resolver and planners see them. `python3 extra_crates.py`"""
import os, sys, json, re, subprocess, collections
T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.normpath(T + "/../../🗑️generated/gran-hubs"); GEN = os.path.normpath(T + "/../../🗑️generated/gran-value")
ROOT = "/Users/ueli/Documents/semio"
g = json.load(open(GEN + "/crates2.json")); C = g["crates"]
man = json.load(open(OUT + "/manifests.json")); roots = json.load(open(OUT + "/manifests-roots.json"))
added = {}
def targets(dirs):
    miss = set()
    for d in dirs:
        for k in ("normal", "build", "dev"):
            for dn, t in C[d]["deps"][k] if d in C else []:
                pass
    return miss
def find_missing():
    miss = set()
    for d, c in C.items():
        for k in ("normal", "build", "dev"):
            for dn, t in c["deps"][k]:
                if t not in C and os.path.isfile(os.path.join(ROOT, t, "Cargo.toml")): miss.add(t)
    return miss
rounds = 0
while True:
    miss = find_missing()
    if not miss or rounds > 4: break
    rounds += 1
    json.dump(sorted(miss), open(OUT + "/dirs-extra.json", "w"), ensure_ascii=False)
    subprocess.run(["bun", T + "/manifests.ts", OUT + "/dirs-extra.json", OUT + "/manifests-extra.json"], check=True, capture_output=True)
    me = json.load(open(OUT + "/manifests-extra.json")); re_ = json.load(open(OUT + "/manifests-extra-roots.json"))
    for k, v in re_.items(): roots.setdefault(k, v)
    for d, m in me.items():
        man[d] = m
        deps = {"normal": [], "build": [], "dev": []}
        for x in m["deps"]:
            if x["path"]: t = os.path.normpath(os.path.join(d, x["path"]))
            elif x["ws"]:
                e = roots.get(m["wsroot"], {}).get(x["key"]) or roots.get(m["wsroot"], {}).get(x["pkg"])
                t = os.path.normpath(os.path.join(m["wsroot"], e["path"])) if e and e.get("path") else None
            else: t = None
            if t: deps[x["kind"]].append([x["key"], t])
        t = open(os.path.join(ROOT, d, "Cargo.toml"), encoding="utf-8").read()
        lp = re.search(r'^\[lib\][^\[]*?^\s*path\s*=\s*"([^"]+)"', t, re.M | re.S)
        C[d] = {"dir": d, "name": m["name"], "libpath": lp.group(1) if lp else None, "proc_macro": "proc-macro = true" in t, "build": None, "deps": deps, "has_ws_table": False}
        added[d] = m["name"]
g["crates"] = C
json.dump(g, open(OUT + "/crates2.json", "w"), ensure_ascii=False)
json.dump(man, open(OUT + "/manifests.json", "w")); json.dump(roots, open(OUT + "/manifests-roots.json", "w"))
print("added", len(added), "rounds", rounds, "crates", len(C))
