"""🧭 Path resolution: every #[path] mount and include_str!/include_bytes! target of the scene tree must exist; reports longest path in bytes and region-marker balance."""
import os, re, sys
from lib import *
root = REPO + "/" + SCENE_REL
missing = []
longest = (0, "")
n_paths = 0
for dp, dn, fn in os.walk(root):
    for f in fn:
        full = os.path.join(dp, f)
        b = len(full[len(REPO) + 1:].encode())
        if b > longest[0]:
            longest = (b, full[len(REPO) + 1:])
        if not f.endswith(".rs"):
            continue
        t = rd(full)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]|include_(?:str|bytes)!\(\s*"([^"]+)"', t):
            p = m.group(1) or m.group(2)
            n_paths += 1
            if not os.path.exists(os.path.normpath(os.path.join(dp, p))):
                missing.append((full[len(root) + 1:], p))
        starts = re.findall(r"^\s*//#region\s+(.*?)\s*$", t, re.M)
        ends = re.findall(r"^\s*//#endregion\s+(.*?)\s*$", t, re.M)
        if sorted(starts) != sorted(ends):
            print("REGION MISMATCH", full[len(root) + 1:])
print("paths checked", n_paths, "missing", len(missing), "longest path bytes", longest)
for m in missing: print("MISSING", m)
