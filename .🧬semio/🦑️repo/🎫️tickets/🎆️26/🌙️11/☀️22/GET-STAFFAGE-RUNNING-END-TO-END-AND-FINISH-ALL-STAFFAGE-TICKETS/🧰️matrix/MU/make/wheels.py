"""🛞️ Ops writer of the spinning wheels of `delivery-van`, `tram`, `used-timpani-pair` and `electric-kick-scooter`: a flat front or plan tyre on a spinning wheel bone becomes a line whenever the wheel stands a quarter turn off; the tyre is a body of revolution about the lateral axis, so its front and plan picture is carried by the bone that carries the wheel and does not spin (the mallets of the timpani, which the strike swings, become solids of their elevation shape at 1:500; the decision of `cargo-bike` and `city-bus`, `multiview-MC.md` section 6): `python3 make/wheels.py <id>…` writes the ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *
CARRIER = {"delivery-van": lambda b: {"wheel-f": "axle-f", "wheel-r": "axle-r"}.get(b), "tram": lambda b: {"ws-f1": "bogie-f", "ws-f2": "bogie-f", "ws-r1": "bogie-r", "ws-r2": "bogie-r"}.get(b), "used-timpani-pair": lambda b: b.replace("wheel-", "castor-") if b.startswith("wheel-") else None, "electric-kick-scooter": lambda b: {"wheel-f": "fork", "wheel-r": "deck"}.get(b)}
SOLIDIFY = {"used-timpani-pair": [{"scales": ["500"], "bones": ["mallet-a", "mallet-b"]}]}
for slug in sys.argv[1:]:
    a, ops = asset(slug), []
    carriers = {CARRIER[slug](b["id"]) for b in a["rig"]["bones"]} - {None}
    for scale in scales(a):
        for view in ("front", "plan"):
            for s in a["representations"][scale]["geometry"][view]:
                to = CARRIER[slug](s["bone"])
                if to is not None and "solid" not in s and s["kind"] not in ("member", "loft"):
                    ops.append({"op": "shape-set", "scale": scale, "view": view, "id": s["id"], "set": {"bone": to}})
                elif s["bone"] in carriers and s["id"].startswith(("tyre", "wheel-l", "wheel-r")):
                    ops.append({"op": "shape-set", "scale": scale, "view": view, "id": s["id"], "set": {"bone": s["bone"]}})
    write(slug, ops, **({"solidify": SOLIDIFY[slug]} if slug in SOLIDIFY else {}))
