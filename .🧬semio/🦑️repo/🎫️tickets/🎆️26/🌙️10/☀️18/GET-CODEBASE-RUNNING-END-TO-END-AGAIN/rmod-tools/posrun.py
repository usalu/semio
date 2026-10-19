"""🔬 RMOD oracle launcher: the unchanged round-2 typeck oracle (c5b pos-tools/pos.py) with the sandbox at /Users/ueli/.cache/rmod/pos-work and its logs in the ticket generated folder under the prefix rmod.
usage: POS_PREFIX=rmod python3 posrun.py run <family.json> --cfgs lib,test --label pre|post --no-base"""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
POS = os.path.abspath(HERE + "/../../../../🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c5b-tools/pos-tools")
sys.path.insert(0, POS)
import poslib

poslib.SCRATCH = "/Users/ueli/.cache/rmod"
poslib.SANDBOX_BASE = poslib.SCRATCH + "/" + poslib.PREFIX + "-work"
poslib.GEN = os.path.abspath(HERE + "/../🗑️generated")
os.makedirs(poslib.SANDBOX_BASE, exist_ok=True)
os.makedirs(poslib.GEN, exist_ok=True)
import pos

sys.argv = ["pos.py"] + sys.argv[1:]
pos.main()
