#!/usr/bin/env python3
"""rebase-g: lands oracle2-g's staged files into a MIRROR (never the live tree) so a batch that started from them (conv-g) can be applied there: a file is replaced when the mirror still equals oracle2-g's base, kept when it already equals the staged file, a new file is created. usage: land-oracle2.py <mirror-root>"""
import filecmp, os, shutil, sys
mirror = sys.argv[1].rstrip("/")
S = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/oracle2-g-tools/staged"
if mirror.startswith("/Users/ueli/Documents/semio"): sys.exit("refused: the live tree is not a mirror")
landed = kept = 0
for base, _, names in os.walk(S + "/files"):
    for name in names:
        if name == ".DS_Store": continue
        rel = os.path.relpath(os.path.join(base, name), S + "/files"); new, old, live = os.path.join(base, name), S + "/base/" + rel, mirror + "/" + rel
        if os.path.exists(live) and filecmp.cmp(live, new, shallow=False): kept += 1; continue
        if os.path.exists(live) and not (os.path.exists(old) and filecmp.cmp(live, old, shallow=False)): sys.exit("oracle2-g file moved in the mirror: " + rel)
        os.makedirs(os.path.dirname(live), exist_ok=True); shutil.copyfile(new, live); landed += 1
print("oracle2-g in mirror: landed", landed, "kept", kept)
