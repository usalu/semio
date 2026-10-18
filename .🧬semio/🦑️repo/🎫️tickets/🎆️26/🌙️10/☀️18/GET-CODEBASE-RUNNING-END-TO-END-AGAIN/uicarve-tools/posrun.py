"""🔬 uicarve oracle launcher (typeck/privacy differential, nightly --emit=metadata). usage: POS_PREFIX=uic python3 posrun.py run <family.json> --cfgs lib,test --label L"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/pos-tools")
import poslib
poslib.SCRATCH = "/Users/ueli/.cache/uicarve"
poslib.SANDBOX_BASE = poslib.SCRATCH + "/" + poslib.PREFIX + "-work"
os.makedirs(poslib.SANDBOX_BASE, exist_ok=True)
import pos
sys.argv = ["pos.py"] + sys.argv[1:]
pos.main()
