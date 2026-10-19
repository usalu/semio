"""🔬 LFIX oracle launcher: runs the unchanged round-2 typeck oracle (a copy of `monolith-split/c5b-tools/pos-tools`) with POS_PREFIX=lfix, logs under the coordinator ticket `🗑️generated` and sandboxes under `~/.cache/lfix`.
usage: POS_PREFIX=lfix python3 posrun.py run families/<family>.json --cfgs test --label L --no-base | index [--force]"""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
sys.path.insert(0, "/Users/ueli/.cache/lfix/pos-tools")
import poslib

poslib.SCRATCH = "/Users/ueli/.cache/lfix"
poslib.GEN = C + "/🗑️generated"
poslib.SANDBOX_BASE = poslib.SCRATCH + "/" + poslib.PREFIX + "-work"
poslib.Manifests.CACHE = poslib.GEN + "/" + poslib.PREFIX + "-manifest-index.json"
os.makedirs(poslib.SANDBOX_BASE, exist_ok=True)
import pos

pos.GEN = poslib.GEN
sys.argv = ["pos.py"] + sys.argv[1:]
pos.main()
