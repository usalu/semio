#!/usr/bin/env python3
"""🩺 Static check: `doc.<field>` / `document.<field>` / `d.<field>` accesses (not method calls) in an artifact's sources name a `pub <field>:` of its model snapshot."""
import os, re, sys
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
arts = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm", "🗿️artifacts")
sel = sys.argv[1:]
import tomllib
def reach_all(root):
    seen=set()
    def walk(f):
        if f in seen or not os.path.isfile(f): return
        seen.add(f); src=open(f,encoding="utf-8").read()
        for m in re.finditer(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*([;{])', src):
            attrs,name,end=m.groups()
            if end=="{": continue
            pm=re.search(r'#\[path\s*=\s*"([^"]+)"\]',attrs)
            if pm: walk(os.path.normpath(os.path.join(os.path.dirname(f),pm.group(1))))
            else:
                b=os.path.dirname(f)
                for c in (os.path.join(b,name+".rs"),os.path.join(b,name,"mod.rs")):
                    if os.path.isfile(c): walk(c); break
    for d,ds,fs in os.walk(root):
        ds[:]=[x for x in ds if x not in ("target","node_modules","🏭️bridge","👽️guest")]
        if "Cargo.toml" in fs and d!=root:
            doc=tomllib.load(open(os.path.join(d,"Cargo.toml"),"rb"))
            if "package" not in doc: continue
            if doc.get("lib",{}).get("path"): walk(os.path.normpath(os.path.join(d,doc["lib"]["path"])))
            for t in doc.get("test",[]): walk(os.path.normpath(os.path.join(d,t["path"])))
    return seen
SKIP = ("target", "node_modules", "🗑️generated", "🏭️bridge", "👽️guest", "🔮️oracles")
for a in sorted(os.listdir(arts)):
    if sel and not any(s in a for s in sel): continue
    root = os.path.join(arts, a)
    snap = os.path.join(root, "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema", "📸️snapshot")
    if not os.path.isdir(snap): continue
    fields = set()
    for d, ds, fs in os.walk(snap):
        ds[:] = [x for x in ds if x not in SKIP and not x.endswith("twin")]
        for f in fs:
            if f.endswith(".rs"): fields.update(re.findall(r'\bpub\s+(?:r#)?(\w+)\s*:', open(os.path.join(d, f), encoding="utf-8").read()))
    uses = {}; mounted = reach_all(root)
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in SKIP and not x.endswith("twin") and x != "📸️snapshot"]
        for f in fs:
            if not f.endswith(".rs"): continue
            p = os.path.join(d, f)
            if p not in mounted: continue
            t = open(p, encoding="utf-8").read()
            for m in re.finditer(r'\b(?:doc|document|snapshot|snap)\.(\w+)\b(?!\s*[(:<])', t):
                n = m.group(1)
                if n in fields or n.isdigit(): continue
                uses.setdefault(n, []).append((p, t.count("\n", 0, m.start()) + 1))
    for n, v in sorted(uses.items()):
        print(f"{a}: `.{n}` not a model field ({len(v)}x) e.g. {os.path.relpath(v[0][0], root)}:{v[0][1]}")
