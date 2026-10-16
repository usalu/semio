#!/usr/bin/env python3
"""♻️ Re-derives the four rehomed files of G3 in the pre-flip world (PI1_REPO = the restored sandbox, PI1_NOFLIP=1) with the descendant-aware rehome, prints their new texts to a json."""
import json, os, sys
import g3spec, groupdrv as G
from lib import *
d = G.Driver(g3spec.S)
d.overlay()
want = {}
for relp, t in d.ov.items():
    if t is None:
        continue
    if any(relp.endswith(x) for x in ("⏳️operation-progress/🦀️.rs", "⏯️tool-run/🦀️.rs", "🛠️tool-machine/🦀️.rs", "⏪️time-travel/🦀️.rs")) and "/🧪️" not in relp:
        want[relp] = t
json.dump(want, open(SCRATCH + "/pi1/oldworld-g3-files.json", "w"), ensure_ascii=False)
print(len(want), list(want))
