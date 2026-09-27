#!/usr/bin/env python3
"""🪞️ ST1 overlay: copies every git-tracked file outside `.🧬semio/` into the overlay root (APFS clonefile when possible)."""
import os
import shutil
import subprocess
import sys

repo = "/Users/ueli/Documents/semio"
target = sys.argv[1]
listing = subprocess.run(["git", "ls-files", "-z", "--", ":!:.🧬semio/**"], cwd=repo, capture_output=True, check=True).stdout.decode().split("\0")
copied = 0
for rel in filter(None, listing):
    source = os.path.join(repo, rel)
    if not os.path.lexists(source):
        continue
    destination = os.path.join(target, rel)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    if os.path.islink(source):
        if not os.path.lexists(destination):
            os.symlink(os.readlink(source), destination)
    else:
        shutil.copy2(source, destination)
    copied += 1
print(f"st1-overlay: {copied} files into {target}")
