#!/usr/bin/env python3
"""Shared helpers for regenerating plugin-units preservation data in the overlay from the current Rust sources."""
import hashlib, json, os, subprocess
O = os.environ.get("TFR_ROOT", "/private/tmp/testfix-real/r").rstrip("/")
U = O + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/"
def sha(b): return hashlib.sha256(b).hexdigest()
def fnlist(jobs):
    jf = "/private/tmp/testfix-real/fnjobs.json"; of = "/private/tmp/testfix-real/fnout.json"
    json.dump(jobs, open(jf, "w", encoding="utf8"), ensure_ascii=False)
    r = subprocess.run(["bun", ".tfr-fnlist.ts", O, of, jf], cwd=O, capture_output=True, text=True)
    if r.returncode: raise SystemExit(r.stderr[-2000:])
    return json.load(open(of, encoding="utf8"))
def load(name): return json.load(open(U + name, encoding="utf8"))
def save(name, d):
    open(U + name, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")

import re
MOUNT = re.compile(r'#\[path\s*=\s*"([^"\n]+)"\]|include!\(\s*"([^"\n]+)"\s*\)')
def closure(start_rel, only_rs=False):
    seen = []
    def walk(rel):
        if rel in seen or (only_rs and not rel.endswith(".rs")): return
        seen.append(rel)
        text = open(O + "/" + rel, encoding="utf8").read()
        for m in MOUNT.finditer(text):
            child = m.group(1) or m.group(2)
            walk(os.path.relpath(os.path.normpath(os.path.join(os.path.dirname(O + "/" + rel), child)), O))
    walk(start_rel)
    return seen

def lock_packages(rel):
    text = open(O + "/" + rel, encoding="utf8").read()
    out = {}
    for blk in text.split("[[package]]\n")[1:]:
        m = re.search(r'^name = "([^"]+)"', blk, re.M)
        if not m: continue
        deps = []
        dm = re.search(r'^dependencies = \[\n(.*?)^\]', blk, re.M | re.S)
        if dm: deps = re.findall(r'"([^"]+)"', dm.group(1))
        out.setdefault(m.group(1), []).append({"version": (re.search(r'^version = "([^"]+)"', blk, re.M) or [None, None])[1], "dependencies": deps, "source": "source =" in blk})
    return out
