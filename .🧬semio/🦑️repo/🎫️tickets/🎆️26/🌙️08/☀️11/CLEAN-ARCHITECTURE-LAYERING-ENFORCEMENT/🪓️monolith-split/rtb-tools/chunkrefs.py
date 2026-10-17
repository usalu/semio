"""🔍️ Finds every #[path]/include! attribute whose resolved target lies inside a folder scheduled for deletion."""
import os, re, sys
from lib import *
import cleanup as C
targets = [R + "/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/" + x for x in ("⏱️frame-latency","📇️directory-door","🛰️dock-unit")] + [R + "/📥️mounts/🧵️wp3-frame-core/🧪️test-hosts/" + x for x in ("🔎️interpreter-introspection","🧷️interpreter-command-wiring")] + [SH + "/" + c for c in C.CHUNK_DIRS]
targets = [os.path.normpath(t) for t in targets]
pat = re.compile(r'#\[path\s*=\s*"([^"]+)"\]|include!\(\s*"([^"]+)"\s*\)|include_str!\(\s*"([^"]+)"\s*\)')
hits = []
for root, dirs, files in os.walk(E):
    dirs[:] = [d for d in dirs if d not in ("node_modules", "target", ".git", "🗑️generated")]
    for f in files:
        if not f.endswith(".rs"): continue
        p = os.path.join(root, f)
        try: t = open(p, encoding="utf-8").read()
        except Exception: continue
        if "path" not in t and "include" not in t: continue
        for m in pat.finditer(t):
            rel = m.group(1) or m.group(2) or m.group(3)
            tgt = os.path.normpath(os.path.join(os.path.dirname(p), rel))
            for d in targets:
                if tgt == d or tgt.startswith(d + os.sep):
                    hits.append((p, rel))
for h in hits: print(os.path.relpath(h[0], E)[-90:], h[1][-70:])
print("hits", len(hits))
