"""🚚 Applies the staged scene tree to the repository: writes new and changed files, deletes only the files the baseline had and the carve retired."""
import os, filecmp, shutil
from lib import *
REPO_SCENE = REPO + "/" + SCENE_REL
def walk(root):
    out = set()
    for dp, dn, fn in os.walk(root):
        for f in fn:
            out.add(os.path.relpath(os.path.join(dp, f), root))
    return out
new, base_ = walk(NEW), walk(BASE)
wrote = removed = 0
for rel in sorted(new):
    src, dst = NEW + "/" + rel, REPO_SCENE + "/" + rel
    if rel in base_ and filecmp.cmp(src, BASE + "/" + rel, shallow=False):
        continue
    if os.path.exists(dst) and filecmp.cmp(src, dst, shallow=False):
        continue
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(src, dst)
    wrote += 1
for rel in sorted(base_ - new):
    dst = REPO_SCENE + "/" + rel
    if os.path.exists(dst):
        os.remove(dst)
        removed += 1
        print("removed", rel)
print("wrote", wrote, "removed", removed)
