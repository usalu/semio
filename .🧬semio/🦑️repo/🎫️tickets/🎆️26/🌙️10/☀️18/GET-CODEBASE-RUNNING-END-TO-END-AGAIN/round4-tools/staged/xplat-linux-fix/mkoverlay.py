#!/usr/bin/env python3
"""mkoverlay.py OVERLAY DIR...: verification copy of the checkout. Every top-level entry is a symlink into the checkout except the ancestors of the listed repo-relative directories, which are real directories whose siblings are symlinks; the listed directories are copied. Nothing under the checkout is written."""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
overlay, dirs = sys.argv[1], sys.argv[2:]
if not overlay.startswith("/private/tmp/xplat-linux-fix/"):
    raise SystemExit("overlay must live under /private/tmp/xplat-linux-fix/")
if os.path.exists(overlay):
    shutil.rmtree(overlay)
os.makedirs(overlay)

def mirror(rel, targets):
    live, real = os.path.join(ROOT, rel), os.path.join(overlay, rel)
    os.makedirs(real, exist_ok=True)
    heads = {t.split("/")[0] for t in targets}
    for name in os.listdir(live):
        if name == ".git" or name in heads:
            continue
        if not os.path.lexists(os.path.join(real, name)):
            os.symlink(os.path.join(live, name), os.path.join(real, name))
    for head in sorted(heads):
        rest = [t.split("/", 1)[1] for t in targets if t.split("/")[0] == head and "/" in t]
        if any("/" not in t and t == head for t in targets):
            shutil.copytree(os.path.join(live, head), os.path.join(real, head), symlinks=True)
        else:
            mirror(os.path.join(rel, head) if rel else head, rest)

mirror("", [d.strip("/") for d in dirs])
print(overlay)
