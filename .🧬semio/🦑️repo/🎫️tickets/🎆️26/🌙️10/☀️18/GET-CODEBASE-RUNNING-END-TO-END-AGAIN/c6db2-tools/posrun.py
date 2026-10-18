"""🔬 C5B oracle launcher: runs the unchanged round-2 typeck oracle (pos-tools/pos.py, POS_PREFIX=c5b) with the sandbox base moved to the stable path /Users/ueli/.cache/c6db2/pos-work (the hard-coded scratchpad of poslib is wiped on restarts).
The oracle reads the live working tree; nothing is written to the repo. usage: POS_PREFIX=c5b python3 posrun.py run <family.json> --cfgs test --label L --no-base"""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
POS = HERE + "/pos-tools"
sys.path.insert(0, POS)
import poslib

poslib.SCRATCH = "/Users/ueli/.cache/c6db2"
poslib.SANDBOX_BASE = poslib.SCRATCH + "/" + poslib.PREFIX + "-work"
os.makedirs(poslib.SANDBOX_BASE, exist_ok=True)
import pos

sys.argv = ["pos.py"] + sys.argv[1:]
pos.main()
