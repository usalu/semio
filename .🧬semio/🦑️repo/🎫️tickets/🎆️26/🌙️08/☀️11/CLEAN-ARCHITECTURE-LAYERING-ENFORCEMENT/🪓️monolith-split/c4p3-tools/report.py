"""📊️ Sizes, path lengths, docstrings and sibling emoji of the eight split targets and their children."""
import os, sys, unicodedata
R = "/Users/ueli/Documents/semio/"
P = R + "✏️s/🔌️plugins/"
STD = "🏅️standards/🔖️1/🪆️subsets/✳️any/"
BIM = P + "🏙️bim/🗿️artifacts/🏢️model/" + STD
ARC = P + "🏛️architect/🗿️artifacts/🏛️program/" + STD
CAD = P + "📐️cad/🗿️artifacts/📐️cad/" + STD
targets = {
 "bim-mutations": BIM + "🧬️schema/🧬️mutations/🟦️.ts",
 "arch-schema": ARC + "🧬️schema/🟦️.ts",
 "arch-diff": ARC + "🧬️schema/🔺️diff/🟦️.ts",
 "cad-actions": CAD + "✏️editor/⚙️engine/🎬️actions/🟦️.ts",
 "cad-repl": CAD + "✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx",
 "cad-geom": CAD + "✏️editor/⚙️engine/📺️renderer/🧲️geometry-targets/🟦️.tsx",
 "cad-artifact": CAD + "✏️editor/⚙️engine/🗿️artifact/🟦️.ts",
 "bim-ifc": BIM + "🧪️tests/🏗️export-bim-1-ifc/🐍️.py",
}
bad = 0
for key, path in targets.items():
    d = os.path.dirname(path)
    base = os.path.basename(path)
    rows = [("<parent>", path)]
    for name in sorted(os.listdir(d)):
        child = os.path.join(d, name, base)
        if os.path.isfile(child) and not name.startswith("🦠"):
            rows.append((name, child))
        elif os.path.isfile(os.path.join(d, name, base[:-1])):
            rows.append((name, os.path.join(d, name, base[:-1])))
    print("==", key)
    for name, f in rows:
        text = open(f, encoding="utf-8").read()
        n = text.count("\n")
        rel = f[len(R):]
        blen = len(rel.encode("utf-8"))
        first = text.split("\n", 1)[0]
        emo = ""
        print("  %-34s %5d lines  path %3d bytes" % (name, n, blen))
        if blen > 240:
            bad += 1; print("    PATH TOO LONG")
print("path problems", bad)
