"""🎯 E51 scoped re-scan: runs the `lit.py` scanner with the given user crates (and their dependency closure as candidates) instead of the whole workspace, so a re-check after fixes takes minutes.
`--undo SUBSTR` replays the inverse of every journalled edit (GEN/e51-edits.jsonl) whose file path contains SUBSTR in memory (the repo is never written), which re-creates the defect and proves the scanner sensitive.
usage: scoped.py (--pkgs a,b,c | --all) [--tag T] [--undo SUBSTR] [--targets]"""
import collections, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
os.environ.setdefault("POS_PREFIX", "e51")
import lit
from lit import World
import poslib

args = sys.argv[1:]
opt = lambda k, d=None: args[args.index(k) + 1] if k in args else d
if "--all" in args:
    _s = poslib.Source(None)
    _m = poslib.Manifests(_s)
    _m.build_index()
    pkgs = sorted(_m.by_name)
else:
    pkgs = opt("--pkgs").split(",")
tag = opt("--tag", "scoped")
undo = opt("--undo")
if undo:
    EDITS = collections.defaultdict(list)
    for line in open(poslib.GEN + "/e51-edits.jsonl", encoding="utf-8"):
        r = json.loads(line)
        if undo in r["rel"]:
            EDITS[r["rel"]].append(r)
    orig = poslib.Source.read

    def _read(self, rel):
        data = orig(self, rel)
        if self.rev is None and data is not None and rel in EDITS:
            text = data.decode("utf-8")
            for r in reversed(EDITS[rel]):
                assert text.count(r["new"]) >= 1, f"undo: edit not found in {rel[-60:]}"
                text = text.replace(r["new"], r["old"], 1)
            return text.encode("utf-8")
        return data

    poslib.Source.read = _read
    print("undoing", sum(len(v) for v in EDITS.values()), "journalled edits in memory over", len(EDITS), "files")
w = World(pkgs, targets="--targets" in args, dep_closure=True)
hits = w.scan(set(pkgs) & set(w.crates), None)
print(f"{len(w.crates)} crates loaded; users {len(set(pkgs) & set(w.crates))}; hits {len(hits)}; ambiguous {len(w.amb)}; unnameable {len(w.unn)}")
by = collections.Counter((lit.short(h["user"]), lit.short(h["defs"][0]["owner"]), h["name"]) for h in hits)
for k, n in by.most_common(80):
    print(f"  {n:3} {k[0]} -> {k[1]} {k[2]}")
json.dump(dict(pkgs=pkgs, hits=hits, amb=w.amb, unn=w.unn), open(f"{poslib.GEN}/e51-{tag}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
