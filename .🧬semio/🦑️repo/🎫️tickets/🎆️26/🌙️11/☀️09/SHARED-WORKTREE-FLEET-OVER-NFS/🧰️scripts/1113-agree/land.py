#!/usr/bin/env python3
"""Three-way landing of the view2 working copy onto the live module under the landing lock.
usage: land.py check   (shows per file: live==base? merge clean?)  |  land.py apply (writes merged files, prints before/after sha256)
base = pristine copy taken before the work, ours = live now, theirs = working copy."""
import os, sys, hashlib, subprocess, tempfile, time, shutil, json
SCRATCH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/agree2"
MOD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
LIVE = "/Users/Shared/semio/" + MOD
COPY = SCRATCH + "/" + MOD
BASE = SCRATCH + "/base-module"
TICKET = "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
LOCK = TICKET + "/1113-landing.lock"
def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else "-"
def files():
    out = []
    for root, _, names in os.walk(COPY):
        for n in names:
            p = os.path.join(root, n); rel = os.path.relpath(p, COPY)
            b = os.path.join(BASE, rel)
            if not os.path.exists(b) or sha(p) != sha(b):
                out.append(rel)
    return sorted(out)
def merge(rel):
    live, base, copy = (os.path.join(d, rel) for d in (LIVE, BASE, COPY))
    if not os.path.exists(live):
        return open(copy, "rb").read(), True
    if sha(live) == sha(base):
        return open(copy, "rb").read(), True
    if sha(live) == sha(copy):
        return open(copy, "rb").read(), True
    with tempfile.TemporaryDirectory() as d:
        shutil.copy(live, d + "/ours"); shutil.copy(base, d + "/base"); shutil.copy(copy, d + "/theirs")
        r = subprocess.run(["git", "merge-file", "-p", d + "/ours", d + "/base", d + "/theirs"], capture_output=True)
        out = r.stdout
        if r.returncode != 0:
            text = out.decode("utf8")
            import re
            m = re.fullmatch(r"(?s)(.*?)<<<<<<< [^\n]*\n([^\n]*\n)=======\n([^\n]*\n)>>>>>>> [^\n]*\n(.*)", text)
            marker = "viewDebounceMilliseconds?:number;"
            if m and marker in m.group(2) and "deltaDebounceMilliseconds" in m.group(3):
                text = m.group(1) + m.group(2).replace(marker, marker + "deltaDebounceMilliseconds?:number;deltaLimit?:number;", 1) + m.group(4)
                return text.encode("utf8"), True
        return out, r.returncode == 0
mode = sys.argv[1]
changed = files()
plan = []
for rel in changed:
    data, clean = merge(rel)
    plan.append((rel, data, clean))
    live = os.path.join(LIVE, rel)
    print(("OK   " if clean else "CONFLICT ") + rel, "live==base" if sha(live) == sha(os.path.join(BASE, rel)) else "live-changed", flush=True)
if mode == "check":
    sys.exit(0)
if not all(c for _, _, c in plan):
    print("conflicts: abort"); sys.exit(1)
rows = []
for rel, data, _ in plan:
    live = os.path.join(LIVE, rel)
    before = sha(live)
    tmp = live + ".view2.tmp"
    open(tmp, "wb").write(data); os.replace(tmp, live)
    rows.append((rel, before, sha(live)))
for rel, b, a in rows:
    print(f"| ./{rel} | {b[:16]} | {a[:16]} |")
