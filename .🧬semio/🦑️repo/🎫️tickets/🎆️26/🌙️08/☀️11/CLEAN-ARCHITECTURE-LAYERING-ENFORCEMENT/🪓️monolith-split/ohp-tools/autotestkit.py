"""🧪 Converts a `#[cfg(test)]` item that another crate names (E0603 "private", E0425 "exists but is inaccessible") to the repo testkit idiom:
`#[cfg(any(test, feature = "testkit"))] pub`, `testkit = []` in the owner's [features], `features = ["testkit"]` on the consumer's dev-dependency on the owner.
usage: autotestkit.py <family> <label> <cfg> [--apply]"""
import json, os, re, sys, collections
ROOT = "/Users/ueli/Documents/semio"
GEN = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
a = sys.argv[1:]
fam, label, cfg = a[:3]
apply = "--apply" in a
IDX = json.load(open(GEN + "/ohp-manifest-index.json", encoding="utf-8"))
BYU = {n.replace("-", "_"): n for n in IDX}
def rp(fn):
    m = re.match(r"^__m_([A-Za-z0-9_]+)/(.*)$", fn)
    if not m: return None, None
    rel = os.path.normpath(m.group(2))
    return m.group(1), (rel if os.path.exists(os.path.join(ROOT, rel)) else None)
jobs = collections.OrderedDict()
src = f"{GEN}/ohp-{fam}-{label}-cur-{cfg}-final.err"
for line in open(src, encoding="utf-8", errors="replace"):
    if not line.startswith("{"): continue
    try: d = json.loads(line)
    except Exception: continue
    code = (d.get("code") or {}).get("code")
    if code not in ("E0603", "E0425", "E0433", "E0599"): continue
    pr = [s for s in d["spans"] if s["is_primary"]]
    if not pr: continue
    upkg, _ = rp(pr[0]["file_name"])
    cands = [s for s in d["spans"] if not s["is_primary"] and "defined here" in (s.get("label") or "")]
    for ch in d["children"]:
        if "inaccessible" in ch["message"] or "defined here" in ch["message"] or "refers to" in ch["message"]:
            cands += ch["spans"]
    cands = [s for s in cands if s["text"]]
    if not cands: continue
    s = cands[-1]
    dpkg, rel = rp(s["file_name"])
    if not rel or dpkg == upkg or not dpkg or not upkg: continue
    p = os.path.join(ROOT, rel)
    try: lines = open(p, encoding="utf-8").read().split("\n")
    except Exception: continue
    i = s["line_start"] - 1
    if not any("#[cfg(test)]" in lines[k] for k in range(max(0, i - 3), i + 1)): continue
    jobs.setdefault((rel, i, dpkg), set()).add(upkg)
def add_feature(mp):
    t = open(mp, encoding="utf-8").read()
    if re.search(r"^testkit\s*=", t, re.M): return False
    m = re.search(r"^\[features\]\n", t, re.M)
    if m:
        t = t[:m.end()] + "testkit = []\n" + t[m.end():]
    else:
        m = re.search(r"^\[dependencies\]\n", t, re.M)
        pos = m.start() if m else len(t)
        t = t[:pos] + "[features]\ntestkit = []\n\n" + t[pos:]
    open(mp, "w", encoding="utf-8").write(t); return True
def add_devdep(cp, owner):
    t = open(cp, encoding="utf-8").read()
    dm = re.search(r"^\[dev-dependencies\]\n", t, re.M)
    pat = re.compile(r"^" + re.escape(owner) + r"\s*=\s*(\{.*\}|\".*\")\s*$|^" + re.escape(owner) + r"\.workspace\s*=\s*true\s*$", re.M)
    if dm:
        nxt = re.search(r"^\[", t[dm.end():], re.M)
        end = dm.end() + (nxt.start() if nxt else len(t) - dm.end())
        sec = t[dm.end():end]
        mm = pat.search(sec)
        if mm and "testkit" in mm.group(0): return "ok"
        if mm:
            line = mm.group(0)
            if line.startswith(owner + ".workspace"):
                new = f"{owner} = {{ workspace = true, features = [\"testkit\"] }}"
            elif "features" in line:
                new = re.sub(r"features\s*=\s*\[([^\]]*)\]", lambda x: "features = [" + (x.group(1).strip() + ", " if x.group(1).strip() else "") + "\"testkit\"]", line, count=1)
            else:
                new = line.rstrip().rstrip("}").rstrip() + ", features = [\"testkit\"] }"
            sec = sec.replace(line, new, 1)
            t = t[:dm.end()] + sec + t[end:]
            open(cp, "w", encoding="utf-8").write(t); return "merged"
    nd = re.search(r"^\[dependencies\]\n", t, re.M)
    if not nd: return "no-deps"
    nxt = re.search(r"^\[", t[nd.end():], re.M)
    sec = t[nd.end(): nd.end() + (nxt.start() if nxt else len(t) - nd.end())]
    mm = pat.search(sec)
    if not mm: return "not-a-dep"
    line = mm.group(0)
    if line.startswith(owner + ".workspace"):
        spec = "{ workspace = true, features = [\"testkit\"] }"
    elif line.rstrip().endswith("}"):
        spec = line.split("=", 1)[1].strip().rstrip("}").rstrip() + ", features = [\"testkit\"] }"
    else:
        spec = "{ version = " + line.split("=", 1)[1].strip() + ", features = [\"testkit\"] }"
    new = f"{owner} = {spec}\n"
    if dm: t = t[:dm.end()] + new + t[dm.end():]
    else: t = t.rstrip("\n") + "\n\n[dev-dependencies]\n" + new
    open(cp, "w", encoding="utf-8").write(t); return "added"
n = 0
for (rel, i, dpkg), users in jobs.items():
    p = os.path.join(ROOT, rel)
    lines = open(p, encoding="utf-8").read().split("\n")
    k = next((k for k in range(max(0, i - 3), i + 1) if "#[cfg(test)]" in lines[k]), None)
    owner = BYU.get(dpkg)
    if k is None or not owner: continue
    consumers = [BYU[u] for u in users if u in BYU]
    print(f"{rel.split('artifacts/')[-1][-70:]}:{i+1} owner={owner} consumers={len(consumers)}")
    n += 1
    if not apply: continue
    lines[k] = lines[k].replace("#[cfg(test)]", '#[cfg(any(test, feature = "testkit"))]')
    l = lines[i]
    lines[i] = re.sub(r"^(\s*)(?:pub\([^)]*\)\s+)?(?=(?:const|static|fn|struct|enum|type|trait|mod|async|unsafe)\b)", lambda m: m.group(1) + "pub ", l, count=1) if not re.match(r"^\s*pub\s", l) else l
    open(p, "w", encoding="utf-8").write("\n".join(lines))
    add_feature(os.path.join(ROOT, IDX[owner]))
    for c in consumers:
        r = add_devdep(os.path.join(ROOT, IDX[c]), owner)
        print("    consumer", c, r)
print("testkit conversions", n, "APPLIED" if apply else "dry")
