"""Finds source-text readers of the five target files: string literals ending in 🦀️.rs that resolve (repo-relative or file-relative) to a target path."""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *

targets = {os.path.normpath(f"{ROOT}/{t['file']}"): n for n, t in TARGETS.items()}
files = subprocess.run(["git", "-c", "core.quotepath=false", "grep", "-l", "-F", "-z", "🦀️.rs", "--", "*.ts", "*.tsx", "*.mjs", "*.js", "*.py", "*.sh", "*.json", "*.jsonc", "*.toml", "*.md", "*.rs", ":!.🧬semio", ":!.cursor"], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace").split("\0")
LIT = re.compile(r'''["'`]([^"'`\n]*🦀️\.rs)["'`]''')
hits = {}
for f in files:
    if not f: continue
    p = os.path.join(ROOT, f)
    try:
        txt = open(p, encoding="utf-8").read()
    except Exception:
        continue
    d = os.path.dirname(p)
    for ln, line in enumerate(txt.split("\n"), 1):
        for m in LIT.finditer(line):
            lit = m.group(1)
            cands = {os.path.normpath(os.path.join(ROOT, lit)), os.path.normpath(os.path.join(d, lit))}
            if lit.startswith("/"): continue
            for c in cands:
                if c in targets:
                    hits.setdefault(targets[c], []).append((f, ln, lit[:120]))
for n, hs in hits.items():
    print(f"== {n}: {len(hs)}")
    for h in hs: print("  ", h[0], h[1], h[2])
