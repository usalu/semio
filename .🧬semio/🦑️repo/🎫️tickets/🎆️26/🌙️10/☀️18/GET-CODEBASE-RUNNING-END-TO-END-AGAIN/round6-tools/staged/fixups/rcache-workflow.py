#!/usr/bin/env python3
"""rcache-server follow-up: the CI workflow calls the renamed command (`cache remote status` -> `remote-cache status`). usage: rcache-workflow.py [--root R] [--write]; exact once, idempotent, dry run by default."""
import os, sys
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.getcwd()
path = os.path.join(root, ".github", "workflows", "build-system.yml")
text = open(path, encoding="utf8").read()
old, new = ' cache remote status', ' remote-cache status'
if text.count(old) == 1:
    print("pending", path)
    if "--write" in sys.argv:
        open(path, "w", encoding="utf8", newline="").write(text.replace(old, new))
        print("written")
elif text.count(new) == 1 and old not in text:
    print("applied", path)
else:
    sys.exit("DRIFT: %s: `cache remote status` found %d times" % (path, text.count(old)))
