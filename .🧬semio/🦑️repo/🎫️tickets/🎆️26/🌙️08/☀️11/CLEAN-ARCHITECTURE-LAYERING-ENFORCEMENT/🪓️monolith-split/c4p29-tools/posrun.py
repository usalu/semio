"""🔬 C4P29 oracle launcher: runs the unchanged round-2 typeck oracle (pos-tools/pos.py, POS_PREFIX=c4p29) with a read overlay, so a STAGED tree can be checked before anything is written to the repo.
A repo-relative file that exists under $C4P29_OVERLAY is read from there; every other file is read from the working tree exactly as pos.py does. Nothing is written to the repo.
Glob emulation (sandbox view only): the stub generator of the oracle derives module aliases and trait positions from the `use` statements of the SAME file, so a child that says `use super::*;` hides the
parent's imports from it. For a staged child whose text carries `use super::*;` the reader therefore appends the parent's top-level `use` items right after that line (rustc itself still resolves the glob).
usage: C4P29_OVERLAY=<dir> POS_PREFIX=c4p29 python3 posrun.py run <family.json> --cfgs lib,test --label L --no-base [--no-glob-emulation]"""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
POS = HERE + "/../pos-tools"
sys.path.insert(0, POS)
sys.path.insert(0, HERE)
import poslib

overlay = os.environ.get("C4P29_OVERLAY", "")
emulate = "--no-glob-emulation" not in sys.argv
if not emulate:
    sys.argv.remove("--no-glob-emulation")
_orig = poslib.Source.read
import injlib


def read(self, rel):
    if overlay and self.rev is None:
        if rel in self.cache:
            return self.cache[rel]
        p = os.path.join(overlay, rel)
        if os.path.isfile(p):
            with open(p, "rb") as f:
                data = f.read()
            if emulate and rel.endswith("🦀️.rs") and b"use super::*;" in data:
                data, _ = injlib.inject(overlay, rel, data)
            self.cache[rel] = data
            return data
    return _orig(self, rel)


poslib.Source.read = read
import pos

sys.argv = ["pos.py"] + sys.argv[1:]
pos.main()
