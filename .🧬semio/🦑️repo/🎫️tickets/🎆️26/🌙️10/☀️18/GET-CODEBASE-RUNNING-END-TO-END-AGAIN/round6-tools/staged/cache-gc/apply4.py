#!/usr/bin/env python3
"""🧰️ Fourth staged set (incremental write-time closure proof: per-group member indexes, collector indexing, `repo cache verify --deep`) on top of staged3. Dry run is the default; --write applies; --record stores the base hashes; --restore undoes the last write. Apply apply3.py first, and before graph-parallel round 2 (which rebases its storage anchors onto this result)."""
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import apply, patches  # noqa: E402
from paths import J, NEW3, NEW4, ROOT, TARGET4  # noqa: E402

def configure():
    patches.STAGED = J(HERE, "staged4")
    patches.EDITORS = patches.EDITORS4
    apply.NEW_FILES = NEW4
    apply.TARGET = TARGET4
    apply.BASES = J(HERE, "bases4.json")
    apply.BACKUP = J(HERE, "backup4")
    apply.MANIFEST = J(apply.BACKUP, "manifest.json")


configure()


def sha(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def record():
    rows = {key: sha(J(ROOT, TARGET4[key])) for key in patches.EDITORS}
    for name, target in NEW4.items():
        accepted = []
        live = J(ROOT, target)
        if os.path.exists(live):
            accepted.append(sha(live))
        for staged in ("staged2", "staged3"):
            earlier = J(HERE, staged, name)
            if os.path.exists(earlier):
                accepted.append(sha(earlier))
        if accepted:
            rows["file:" + name] = accepted
    with open(apply.BASES, "w", encoding="utf-8") as handle:
        json.dump(rows, handle, indent=1, sort_keys=True)
        handle.write("\n")
    print("recorded %d base entr(ies) in %s" % (len(rows), apply.BASES))
    return 0


def main(argv):
    configure()
    if "--record" in argv:
        return record()
    return apply.main(argv)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
