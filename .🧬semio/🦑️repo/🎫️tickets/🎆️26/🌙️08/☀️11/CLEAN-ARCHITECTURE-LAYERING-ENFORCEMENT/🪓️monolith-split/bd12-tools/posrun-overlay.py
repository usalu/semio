"""🔬 BD12 differential launcher: like posrun.py, but the baseline side reads the live tree with the reactor turn folder replaced by the pre-edit snapshot `orig/rt` (rev "OVERLAY"). Reads only; writes only inside the sandbox and c5a-* logs.
usage: POS_PREFIX=c5a python3 posrun-overlay.py run families/bd12-diff.json --cfgs lib,test --label L"""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
POS = HERE + "/pos-tools"
sys.path.insert(0, POS)
import poslib

poslib.SCRATCH = "/Users/ueli/.cache/bd12"
poslib.SANDBOX_BASE = poslib.SCRATCH + "/" + poslib.PREFIX + "-work"
os.makedirs(poslib.SANDBOX_BASE, exist_ok=True)
import pos

TURN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn"
SNAP = HERE + "/orig/rt"


class Overlay(poslib.Source):
    def read(self, rel):
        if rel in self.cache:
            return self.cache[rel]
        if rel.startswith(TURN + "/"):
            try:
                with open(SNAP + "/" + rel[len(TURN) + 1:], "rb") as f:
                    data = f.read()
            except (FileNotFoundError, IsADirectoryError, NotADirectoryError):
                data = None
            self.cache[rel] = data
            return data
        return super().read(rel)


_orig = pos.Source
pos.Source = lambda rev=None: Overlay(None) if rev == "OVERLAY" else _orig(rev)
sys.argv = ["pos.py"] + sys.argv[1:]
pos.main()
