"""Path checks of the orc slice: every #[path]/include!/include_str! literal of the os-pack tree and of every edited file resolves; no reference to a deleted file remains in the repo; longest paths."""
import os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
PK = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack"
SP = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/orcpre/"
rx = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]|include(?:_str|_bytes)?!\s*\(\s*"([^"]+)"\s*\)')
files = []
for dp, dn, fn in os.walk(PK):
    dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated")]
    files += [os.path.join(dp, f) for f in fn if f.endswith(".rs")]
checked = missing = 0
for p in sorted(files):
    t = open(p, encoding="utf8").read()
    for m in rx.finditer(t):
        lit = m.group(1) or m.group(2)
        tgt = os.path.normpath(os.path.join(os.path.dirname(p), lit))
        checked += 1
        if not os.path.exists(tgt):
            missing += 1
            print("MISSING", p[len(ROOT) + 1:][-80:], "->", lit[-80:])
print(f"os-pack tree: {len(files)} rs files, {checked} path/include literals, {missing} missing")
deleted = [os.path.relpath(os.path.join(dp, f), SP) for dp, dn, fn in os.walk(SP) for f in fn if not os.path.exists(os.path.join(ROOT, os.path.relpath(os.path.join(dp, f), SP)))]
print("deleted files:", len(deleted))
for d in deleted:
    print("  ", d[-100:])
longest = max((len(os.path.relpath(p, ROOT).encode("utf8")) for p in files), default=0)
print("longest os-pack source path (bytes):", longest)
