import collections, os, subprocess, sys
R = "/Users/ueli/Documents/semio"
F = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
E = R + "/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine"
M = R + "/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✏️mesh-edit"
J = R + "/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json"
cases = [("lowpoly-engine", E, ["⚠️errors"]), ("mesh-edit", M, ["🎯️element-picks"]), ("layout-json", J, ["✅️json-validation"])]
def ms(paths):
    c = collections.Counter()
    for p in paths:
        for l in open(p, encoding="utf-8").read().split("\n"):
            if l.strip():
                c[l.strip()] += 1
    return c
ok = True
for name, d, kids in cases:
    before = ms([F + f"/lwr-tools/orig/{name}.rs"])
    after = ms([d + "/🦀️.rs"] + [d + "/" + k + "/🦀️.rs" for k in kids])
    removed = before - after
    added = after - before
    print(f"== {name}: removed {sum(removed.values())} added {sum(added.values())}")
    for l, n in sorted(removed.items()):
        print(f"   - {n}x {l[:170]}")
    for l, n in sorted(added.items()):
        print(f"   + {n}x {l[:170]}")
