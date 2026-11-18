"""Tower crane 1:200 and 1:100: jib and counter-jib drawn as the projected centre lines of their slender volume members in the elevation (bone frame) and the plan, and in the 1:100 front (machinery-strokes.ts); the old swept rectangle and polygon shapes of those members are replaced one for one."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
OLD = {
    ("elevation", 200): ["jib-chord-l", "jib-chord-r", "jib-rail-l", "jib-rail-r", "jib-diagonals-l", "jib-diagonals-r", "jib-end-root", "jib-end-tip", "counter-chord-top-l", "counter-chord-top-r", "counter-chord-bottom-l", "counter-chord-bottom-r", "counter-diagonals-l", "counter-diagonals-r"],
    ("plan", 200): ["jib-chord-l", "jib-chord-r", "jib-rail-l", "jib-rail-r", "jib-diagonals", "counter-chord-l", "counter-chord-r", "counter-diagonals"],
    ("front", 100): ["jib-frame", "counter-jib"],
}
OLD[("elevation", 100)] = OLD[("elevation", 200)]
OLD[("plan", 100)] = ["jib-chord-l", "jib-chord-r", "jib-diagonals", "counter-chord-l", "counter-chord-r", "counter-diagonals", "jib-rail-l", "jib-rail-r"]
for (view, scale), ids in OLD.items():
    res = lib.strokes(scale, view, "^(jib|counter)-", "jc")
    lib.apply(d, scale, view, res, ids, ids[0])
d.commit()
