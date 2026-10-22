#!/usr/bin/env python3
"""🧰️ Third staged set (graph image budget, background reclaim, status without walking on top of staged2). Dry run is the default; --write applies; --recreate-graph also recreates the derived graph image after writing (required once, because the image is born with its declared size)."""
import hashlib, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import apply, patches, recreate  # noqa: E402
from paths import J, NEW3, ROOT, TARGET  # noqa: E402

NEW_CAPACITY = 6442450944
def configure():
    patches.STAGED = J(HERE, "staged3")
    patches.EDITORS = {**patches.EDITORS3, "catalog": patches.EDITORS2["catalog"]}
    apply.NEW_FILES = NEW3
    apply.TARGET = TARGET
    apply.BASES = J(HERE, "bases3.json")
    apply.BACKUP = J(HERE, "backup3")
    apply.MANIFEST = J(apply.BACKUP, "manifest.json")


configure()


def sha(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def record():
    import json
    rows = {key: sha(J(ROOT, TARGET[key])) for key in patches.EDITORS}
    for name, target in NEW3.items():
        accepted = []
        live = J(ROOT, target)
        if os.path.exists(live):
            accepted.append(sha(live))
        second = J(HERE, "staged2", name)
        if os.path.exists(second):
            accepted.append(sha(second))
        if accepted:
            rows["file:" + name] = accepted
    with open(apply.BASES, "w", encoding="utf-8") as handle:
        json.dump(rows, handle, indent=1, sort_keys=True)
        handle.write("\n")
    print("recorded %d base entr(ies) in %s" % (len(rows), apply.BASES))
    return 0


def guarded(argv):
    code, notes = 0, []
    try:
        writes, notes = apply.plan()
    except patches.Drift as error:
        print("refused: drift: %s" % error)
        return 3
    for note in notes:
        print(note)
    _, _, problems = recreate.preflight("graph")
    if problems:
        for problem in problems:
            print("refused:", problem)
        return 2
    try:
        with recreate.holding("graph"):
            _, _, problems = recreate.preflight("graph")
            if problems:
                for problem in problems:
                    print("refused:", problem)
                return 2
            code = apply.main([value for value in argv if value != "--recreate-graph"])
            if code != 0:
                return code
            code = recreate.run("graph", True, held=True)
            if code != 0:
                print("recreate failed after the write: rolling the files back")
                restored = apply.restore()
                return code if restored == 0 else restored
            return 0
    except recreate.Busy as error:
        print("refused: leases not available (nothing was written):", error)
        return 2


def main(argv):
    configure()
    if "--record" in argv:
        return record()
    if "--restore" in argv or "--write" not in argv:
        return apply.main([value for value in argv if value != "--recreate-graph"])
    directory = J(ROOT, ".cache/b/g")
    born = recreate.envelope(directory)
    if "--recreate-graph" in argv:
        return guarded(argv)
    if born is not None and born != NEW_CAPACITY:
        print("refused: the graph image was born with %s bytes but %s are declared now; pass --recreate-graph (quiet tree) or recreate it first with recreate.py graph --yes" % (born, NEW_CAPACITY))
        return 4
    return apply.main(argv)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
