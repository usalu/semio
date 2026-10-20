#!/usr/bin/env python3
"""Sets 0644 on the round-3/4 apply outputs under 🏗️build/ and 🏃️process/ that the tempfile-based appliers left at 0600 (listed in mode-600-held.json). Run only after the coordinator announces chain17 green (chmod alters ctime under watched trees)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); BASE = "/Users/ueli/Documents/semio/"
n = 0
for f in json.load(open(HERE + "/mode-600-held.json", encoding="utf8")):
    p = BASE + f
    if os.path.isfile(p) and (os.lstat(p).st_mode & 0o777) == 0o600:
        os.chmod(p, 0o644); n += 1
print("chmod", n)
