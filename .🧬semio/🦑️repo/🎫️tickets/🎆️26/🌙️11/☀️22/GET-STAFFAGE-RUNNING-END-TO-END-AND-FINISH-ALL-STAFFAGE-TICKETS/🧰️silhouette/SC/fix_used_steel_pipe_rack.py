"""🧰️ Repair of `used-steel-pipe-rack` at 1:50: the two L 50 x 5 ties and the two load straps were declared `slender`; a tie is then compared as a stroke that runs half a width (25 mm) beyond the columns it is welded between (24.4 mm in the front elevation and the plan) and, seen end-on in the side elevation, as a stroke of no direction (46 mm). At 1:50 they are drawn at their true section and are compared as the sections they are. Idempotent: python3 fix_used_steel_pipe_rack.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "used-steel-pipe-rack"
text, doc = sc.load(ID)
for name in ("tie-0", "tie-1", "strap-0", "strap-1"):
    sc.solid(doc, 50, name).pop("slender", None)
sc.save(ID, text, doc)
