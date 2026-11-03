"""🔎 Cargo.toml vs artifact Cargo.lock (dependency name sets per package), workspace member coverage and workspace.dependencies for every crate of the artifact."""
import os, re, sys, tomllib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *
code = sys.argv[1]
adir = ART + "/" + CODE[code]
lock = tomllib.load(open(adir + "/Cargo.lock", "rb"))
lp = {}
for p in lock["package"]: lp.setdefault(p["name"], []).append(p)
ws = tomllib.load(open(adir + "/Cargo.toml", "rb"))
members = [os.path.normpath(os.path.join(adir, m)) for m in ws.get("workspace", {}).get("members", [])]
wsdeps = ws.get("workspace", {}).get("dependencies", {})
crates = {}
for dp, dn, fn in os.walk(adir):
    dn[:] = [d for d in dn if d not in SKIP]
    if "Cargo.toml" in fn and dp != adir:
        t = tomllib.load(open(os.path.join(dp, "Cargo.toml"), "rb"))
        if "package" in t: crates[t["package"]["name"]] = (dp, t)
for name, (dp, t) in sorted(crates.items()):
    if os.path.normpath(dp) not in members: print(f"{rel(dp)}: crate {name} is not a workspace member")
    if name not in wsdeps and name.startswith("semio-s-artifact-norm"): print(f"{rel(dp)}: crate {name} lacks [workspace.dependencies] entry")
    if name not in lp: print(f"{rel(dp)}: crate {name} missing in Cargo.lock"); continue
    want = set()
    for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
        for k, v in t.get(sec, {}).items():
            real = v.get("package", k) if isinstance(v, dict) else k
            if isinstance(v, dict) and v.get("workspace") and isinstance(wsdeps.get(k), dict): real = wsdeps[k].get("package", k)
            want.add(real)
    have = {re.sub(r'\s.*', '', d) for d in lp[name][0].get("dependencies", [])}
    miss, extra = want - have, have - want
    if miss: print(f"{rel(dp)}: {name} Cargo.toml deps absent from lock: {sorted(miss)}")
    if extra: print(f"{rel(dp)}: {name} lock deps absent from Cargo.toml: {sorted(extra)}")
    for k, v in {**t.get("dependencies", {}), **t.get("dev-dependencies", {})}.items():
        if isinstance(v, dict) and v.get("workspace") and k not in wsdeps: print(f"{rel(dp)}: {name} uses workspace dep {k} which is not in [workspace.dependencies]")
