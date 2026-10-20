#!/usr/bin/env python3
"""🔬 Overlay launcher for the unchanged POS typeck oracle (`--emit=metadata` over a one-crate union with a stub layer).

It reads an overlay tree instead of the checkout (`poslib.ROOT`), indexes manifests with a directory walk (the overlay is not a git repository) and writes only below the sandbox and the generated folder.

usage: oracle.py run <overlay-root> <family.json> <label> [--cfgs lib,test] [--mutate "<pkg>::<rel-file>@@old@@new"]
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.abspath(HERE + "/../..")
POS = os.path.abspath(TICKET + "/../../../🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c5b-tools/pos-tools")
GEN = TICKET + "/🗑️generated/gran-value/oracle"
WORK = "/private/tmp/gran-value/oracle-work"


def walk_index(self, force=False):
    """🗂️ Manifest index by directory walk below the overlay root."""
    import poslib
    idx = {}
    for dp, dn, fn in os.walk(poslib.ROOT):
        dn[:] = sorted(d for d in dn if d not in ("node_modules", "target", "dist", ".cache"))
        if "Cargo.toml" in fn:
            rel = os.path.relpath(os.path.join(dp, "Cargo.toml"), poslib.ROOT)
            data = self.src.read(rel)
            name = poslib.parse_toml(data.decode("utf-8", "replace")).get("package", {}).get("name") if data else None
            if name:
                idx.setdefault(name.strip().strip('"'), rel)
    self.by_name = idx


def main():
    a = sys.argv[1:]
    if len(a) < 4 or a[0] != "run":
        print(__doc__)
        return 2
    root, family, label = os.path.abspath(a[1]), a[2], a[3]
    os.makedirs(GEN, exist_ok=True)
    os.makedirs(WORK, exist_ok=True)
    os.environ["POS_PREFIX"] = "gv"
    sys.path.insert(0, POS)
    import poslib
    poslib.ROOT, poslib.GEN, poslib.SCRATCH = root, GEN, WORK
    poslib.SANDBOX_BASE = WORK + "/gv-work"
    os.makedirs(poslib.SANDBOX_BASE, exist_ok=True)
    poslib.Manifests.CACHE = GEN + "/gv-manifest-index-" + label + ".json"
    poslib.Manifests.build_index = walk_index
    import pos
    spec = json.load(open(family, encoding="utf-8"))
    args = {"mutate": [], "label": label, "no_base": True}
    i = 4
    while i < len(a):
        if a[i] == "--cfgs":
            args["cfgs"] = a[i + 1].split(",")
            i += 2
        elif a[i] == "--mutate":
            args["mutate"].append(a[i + 1])
            i += 2
        else:
            raise SystemExit("bad arg " + a[i])
    pos.run_family(spec, args)


if __name__ == "__main__":
    sys.exit(main())
