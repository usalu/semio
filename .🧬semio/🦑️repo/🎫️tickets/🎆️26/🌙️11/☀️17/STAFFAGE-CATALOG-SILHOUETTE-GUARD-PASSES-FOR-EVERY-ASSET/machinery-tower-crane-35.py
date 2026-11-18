"""Tower crane 1:50 front: the volume chord dots are squares tilted by 20 degrees, so each chord dot is a pen stroke along that tilt (the left one 11 mm long). The left jib chord dot is a 13 mm long stroke from x -0.752 m to x -0.7385 m (it contains the volume chord point, so the chord keeps its own pen width against the diagonals meeting there): the tiled guard drops a pen stroke from a tile whose box edge lies within the pen of the stroke point (box edge x -0.7487 m), and the chord pen of 0.14 m reaches into it. The drawn chord is 12 mm wider on that side."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
d.setv(sp.child(d.shape(50, "front", "jib-chord-stroke"), "d"), "M 0.7405 45.1402 L 0.7395 45.1398 M -0.7504 45.1363 L -0.74 45.14")
d.commit()
