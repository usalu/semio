"""🔮 E51 oracle launcher: runs the round-2 union oracle (`pos-tools/pos.py`) with prefix `e51`. With `E51_BEFORE=1` every repo read of a file that this slice edited returns the file with the journalled edits (`GEN/e51-edits.jsonl`) undone in memory,
so one run reproduces the tree without the slice's edits while every peer change stays in place. The repo is never written.
usage: [E51_BEFORE=1] oracle.py run <family.json> [--cfgs lib,test] [--label L] [--no-base]"""
import collections, json, os, sys
os.environ.setdefault("POS_PREFIX", "e51")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../pos-tools")
import poslib

EDITS = collections.defaultdict(list)
JOURNAL = poslib.GEN + "/e51-edits.jsonl"
if os.environ.get("E51_BEFORE") == "1" and os.path.exists(JOURNAL):
    for line in open(JOURNAL, encoding="utf-8"):
        r = json.loads(line)
        EDITS[r["rel"]].append(r)
_orig = poslib.Source.read


def _read(self, rel):
    data = _orig(self, rel)
    if self.rev is None and data is not None and rel in EDITS:
        text = data.decode("utf-8")
        for r in reversed(EDITS[rel]):
            assert text.count(r["new"]) >= 1, f"E51_BEFORE: edit not found in {rel[-60:]}: {r['new'][:60]!r}"
            text = text.replace(r["new"], r["old"], 1)
        return text.encode("utf-8")
    return data


poslib.Source.read = _read
import pos

if __name__ == "__main__":
    pos.main()
