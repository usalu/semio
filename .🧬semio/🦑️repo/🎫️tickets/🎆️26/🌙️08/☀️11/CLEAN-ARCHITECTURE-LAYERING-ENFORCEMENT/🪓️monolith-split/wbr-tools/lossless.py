"""Lossless proof on the LIVE tree: stripped non-empty line multiset of the snapshot target versus the union of all installed files.
Prints removed/added line classes so every difference is accounted for."""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *

out = []
for name, t in TARGETS.items():
    if len(sys.argv) > 1 and name not in sys.argv[1:]:
        continue
    orig = lines_of(read(snap_dir(name) + "/" + t["file"]))
    files = json.loads(read(f"{W}/{name}/installed.json"))["files"]
    new = []
    for rel in files:
        new.extend(lines_of(read(ROOT + "/" + rel)))
    gone, added = diff_multiset(orig, new)

    def cls(l):
        if re.match(r"^(pub(\([^)]*\))? )?(mod \w+( \{|;)|use [\w:{},\s]+;)$", l) or l.startswith("#[path") or l.startswith("#[cfg"):
            return "wiring (mount, use, cfg)"
        if l.startswith("//!"):
            return "file docstring"
        if "pub(super)" in l:
            return "widened to pub(super)"
        if l == "}":
            return "closing brace of lifted inline mod"
        if l.startswith("include"):
            return "include! path re-based"
        if l.startswith("// #endregion") or l.startswith("//#endregion"):
            return "region marker"
        return "other"

    ga, ad = collections.Counter(), collections.Counter()
    for l, n in gone.items():
        ga[cls(l)] += n
    for l, n in added.items():
        ad[cls(l)] += n
    print(f"[{name}] original={len(orig)} lines, new files={len(files)}, new total={len(new)}")
    print(f"   removed (multiset, stripped, non-empty): {sum(gone.values())} {dict(ga)}")
    print(f"   added   (multiset, stripped, non-empty): {sum(added.values())} {dict(ad)}")
    for l, n in sorted(gone.items()):
        if cls(l) in ("other", "region marker"):
            print(f"   - removed {n}x {l[:150]}")
    for l, n in sorted(added.items()):
        if cls(l) in ("other", "region marker"):
            print(f"   + added   {n}x {l[:150]}")
