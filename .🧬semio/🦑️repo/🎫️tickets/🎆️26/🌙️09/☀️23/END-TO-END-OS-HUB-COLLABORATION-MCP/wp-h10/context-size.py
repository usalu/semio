#!/usr/bin/env python3
"""🐳️ H10: approximates the docker build context of the repo root under .dockerignore (root-only names and `**/` names),
reporting the largest included directories. usage: context-size.py <repo> [depth]"""
import os, sys
repo = sys.argv[1]; depth = int(sys.argv[2]) if len(sys.argv) > 2 else 2
root_only, anywhere, anywhere_paths = set(), set(), []
for line in open(os.path.join(repo, ".dockerignore"), encoding="utf-8"):
    line = line.strip()
    if not line or line.startswith("#"): continue
    if line.startswith("**/"):
        rest = line[3:]
        (anywhere_paths if "/" in rest else anywhere).add(rest) if not "/" in rest else anywhere_paths.append(rest)
    else:
        root_only.add(line)
def excluded(rel):
    parts = rel.split("/")
    if any(rel == p or rel.startswith(p + "/") for p in root_only) or any(parts[0].startswith(p[:-1]) for p in root_only if p.endswith("*")): return True
    if any(part in anywhere for part in parts): return True
    if any(rel.endswith("/" + p) or rel == p or ("/" + p + "/") in ("/" + rel + "/") for p in anywhere_paths): return True
    return False
sizes = {}
for dirpath, dirnames, filenames in os.walk(repo):
    rel = os.path.relpath(dirpath, repo)
    rel = "" if rel == "." else rel
    keep = []
    for d in dirnames:
        r = f"{rel}/{d}" if rel else d
        if not excluded(r) and not os.path.islink(os.path.join(dirpath, d)): keep.append(d)
    dirnames[:] = keep
    total = 0
    for f in filenames:
        r = f"{rel}/{f}" if rel else f
        if excluded(r): continue
        try: total += os.lstat(os.path.join(dirpath, f)).st_size
        except OSError: pass
    key = "/".join(rel.split("/")[:depth]) if rel else "."
    sizes[key] = sizes.get(key, 0) + total
for key, size in sorted(sizes.items(), key=lambda kv: -kv[1])[:40]:
    print(f"{size/1e9:8.2f} GB  {key}")
print(f"{sum(sizes.values())/1e9:8.2f} GB  TOTAL")
