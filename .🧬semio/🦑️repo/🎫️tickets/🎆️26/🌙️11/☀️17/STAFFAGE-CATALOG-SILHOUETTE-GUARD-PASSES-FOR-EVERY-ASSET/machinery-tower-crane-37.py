"""Tower crane: wide chord strokes carry the 2 mm paper pen the validation allows (pen-units 0.03 to 2 mm): the volume chord is drawn at the followed pen width, so drawing and volume stay equal (1:100 elevation and plan chords 0.2 m, 1:50 front chord dots 0.1 m)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for sc, view, sid in ((100, "elevation", "jib-chord-top-stroke"), (100, "elevation", "jib-chord-bottom-stroke"), (100, "plan", "jib-chord-top-stroke"), (100, "plan", "jib-chord-bottom-stroke"), (50, "front", "jib-chord-stroke"), (50, "front", "jib-rail-stroke"), (50, "front", "counter-chord-bottom-stroke"), (50, "front", "counter-chord-top-stroke")):
    d.setv(sp.child(d.shape(sc, view, sid), "penMm"), 2.0)
d.commit()
