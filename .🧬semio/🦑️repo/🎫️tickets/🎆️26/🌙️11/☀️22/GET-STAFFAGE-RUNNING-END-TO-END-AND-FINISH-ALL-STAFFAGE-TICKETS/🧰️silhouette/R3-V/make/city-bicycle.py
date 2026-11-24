"""🚲️ Writes the repair ops of `city-bicycle` (ops/city-bicycle.json): the head lamp in the 1:50 plan. The body is a bowl that widens from 10 mm radius at the bracket to 39 mm at the lens; the drawing was two cylinders (22 and 39 mm) whose wide one began 22 mm behind the place where the bowl is that wide and closed the two openings between fork crown, fork blades, mudguard and lamp. The lamp is drawn as four cylinders that follow the bowl (18, 31, 36 and 39 mm radius). The plan also left out three bars of the body: the two pedal spindles between the pedal blocks (the drawn pedal was one open frame, the body two) and the crank axle, which closes the 2 mm slit between bottom bracket shell and chain case; they are drawn below the parts that hide them."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, write

SLUG = "city-bicycle"
a = asset(SLUG)
O, A = (0.215, 0.075), (0.422618, 0.906308)
at = lambda t: {"x": round(O[0] + t * A[0], 5), "y": round(O[1] + t * A[1], 5), "z": 0}
STEPS = (("head-lamp-back", -0.034, -0.023, 0.036), ("head-lamp-neck", -0.023, -0.008, 0.062), ("head-lamp-bowl", -0.008, 0.012, 0.072), ("head-lamp", 0.012, 0.027, 0.078))
like = shape(a, 50, "plan", "head-lamp-back")
ops, after = [], "head-lamp-back"
for sid, t0, t1, d in STEPS:
    s = {**like, "id": sid, "from": at(t0), "to": at(t1), "section": {"kind": "circle", "d": d}}
    if sid == "head-lamp":
        s["z"] = shape(a, 50, "plan", "head-lamp")["z"]
    ops.append({"op": "shape-add", "scale": "50", "view": "plan", "after": after, "shape": s})
    after = sid
DRIVE = {"role": "detail", "filled": True, "tone": 0.7, "region": "drivetrain", "material": "chrome", "penMm": 0.13}
first = "stand-bracket"
bar = lambda sid, bone, z, z0, z1, d: {"op": "shape-add", "scale": "50", "view": "plan", "after": first, "shape": {"kind": "member", "id": sid, "bone": bone, "z": z, **DRIVE, "from": {"x": 0, "y": 0, "z": z0}, "to": {"x": 0, "y": 0, "z": z1}, "section": {"kind": "circle", "d": d}}}
ops.append(bar("crank-axle", "crank", 8.2, -0.078, 0.078, 0.017))
ops.append(bar("pedal-spindle-n", "pedal-n", 8.4, -0.198, -0.09, 0.012))
ops.append(bar("pedal-spindle-f", "pedal-f", 8.6, 0.09, 0.198, 0.012))
write(SLUG, ops)
print(f"{SLUG}: {len(ops)} ops")
