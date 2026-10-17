"""🧪 Runs the POS oracle with the pre-edit text of the files named in an overlay JSON (repo-relative path -> backup file), so a baseline run sees exactly the tree before this slice's edits.

usage: rsd_pos.py <overlay.json> run <family.json> [pos.py run options...]
Read only: it patches the in-memory file reader of poslib and writes nothing but the oracle's own sandbox and logs.
"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
POS = os.path.join(os.path.dirname(HERE), "pos-tools")
sys.path.insert(0, POS)
import poslib

overlay = json.load(open(sys.argv[1], encoding="utf-8"))
orig = poslib.Source.read


def read(self, rel):
    if self.rev is None and rel in overlay:
        with open(overlay[rel], "rb") as f:
            return f.read()
    return orig(self, rel)


poslib.Source.read = read
import pos

sys.argv = ["pos.py"] + sys.argv[2:]
pos.main()
