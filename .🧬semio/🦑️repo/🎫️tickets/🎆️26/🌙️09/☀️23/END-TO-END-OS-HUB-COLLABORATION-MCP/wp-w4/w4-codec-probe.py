#!/usr/bin/env python3
"""🧬️ W4 (14c): the trusted-catalog codec step per package, offline — each plugin's committed descriptor kinds asked of its
component-dev through `semio-framework-plugin-describe codecs` (the publish's own probe); a package with 0 owned rows fails
`trusted <plugin> closure carries no artifact codec`. usage: w4-codec-probe.py <out-dir> [plugin-dir-name…]"""
import json, os, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio"
PLUG = f"{ROOT}/✏️s/🔌️plugins"
EMIT = f"{ROOT}/.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/semio-framework-plugin-describe"
out = sys.argv[1]; os.makedirs(out, exist_ok=True)
names = sys.argv[2:] or sorted(n for n in os.listdir(PLUG) if os.path.isfile(f"{PLUG}/{n}/🔣️.json") and os.path.isdir(f"{PLUG}/{n}/📦️packages/🦀️rust/dist/component-dev"))
for name in names:
    m = json.load(open(f"{PLUG}/{name}/🔣️.json")); m = m.get("manifest", m)
    kinds, seen = [], set()
    for k in list(m.get("artifactKinds") or []) + [k for a in m.get("apps", []) for k in (a.get("artifactKinds") or [])]:
        if k["id"] not in seen: seen.add(k["id"]); kinds.append((k["id"], k["schema"]))
    dev = f"{PLUG}/{name}/📦️packages/🦀️rust/dist/component-dev"
    wasm = [f for f in os.listdir(dev) if f.endswith(".wasm")]
    if not kinds or not wasm: print(f"{name}: kinds={len(kinds)} wasm={wasm} SKIP"); continue
    t = time.time(); dest = f"{out}/{name}.json"
    p = subprocess.run([EMIT, "codecs", f"{dev}/{wasm[0]}", "--kinds", ",".join(f"{a}={b}" for a, b in kinds), "--out", dest], capture_output=True, text=True, cwd=ROOT)
    if p.returncode != 0: print(f"{name}: PROBE FAILED rc={p.returncode} {p.stderr.strip()[-300:]}"); continue
    d = json.load(open(dest))
    verdict = "OK" if d["rows"] else "NO-CODEC"
    print(f"{name}: {verdict} owned={[r['artifactSchema'] for r in d['rows']]} unowned={[u['artifactSchema'] for u in d['unowned']]} {time.time()-t:.0f}s", flush=True)
