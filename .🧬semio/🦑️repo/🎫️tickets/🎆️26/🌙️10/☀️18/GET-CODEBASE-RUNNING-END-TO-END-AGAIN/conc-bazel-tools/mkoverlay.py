#!/usr/bin/env python3
"""mkoverlay.py <overlay> [--root R]: a verification copy that is a symlink mirror of the checkout in which only the Bazel adapter directory is real (copied), so patched files and their tests run without touching the checkout."""
import os, shutil, sys
from targets import Z
root = "/Users/ueli/Documents/semio"
overlay = sys.argv[1]
parts = Z.split("/")
if os.path.exists(overlay):
    shutil.rmtree(overlay)
live = root
real = overlay
os.makedirs(real)
for index, part in enumerate(parts):
    for name in os.listdir(live):
        if name == part or name == '.git':
            continue
        os.symlink(os.path.join(live, name), os.path.join(real, name))
    live = os.path.join(live, part)
    real = os.path.join(real, part)
    if index < len(parts) - 1:
        os.makedirs(real)
shutil.copytree(live, real, symlinks=True)
for name in os.listdir(root):
    if name == parts[0] or name == '.git':
        continue
    if not os.path.lexists(os.path.join(overlay, name)):
        os.symlink(os.path.join(root, name), os.path.join(overlay, name))
print(overlay)
