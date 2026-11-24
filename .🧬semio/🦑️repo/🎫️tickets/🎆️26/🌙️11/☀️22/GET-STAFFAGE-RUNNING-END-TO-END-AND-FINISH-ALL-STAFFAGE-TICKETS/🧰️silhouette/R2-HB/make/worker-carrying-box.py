"""📦️ Volume part of the repair of `worker-carrying-box` 1:50, written as ops/worker-carrying-box.hand.json: the 20 kg carton was an empty shell of 7.5 mm board, so the side elevation saw through both hand holes (a hole of 325 cells in the body where the drawing shows the dark hand hole on the closed carton: 107.4 mm). The carton gets its load: one block of packed goods inside the board, 60 mm clear of both end walls so that the fingers in the hand holes keep their room, a member of the right carton half like the shell. `python3 make/worker-carrying-box.py`"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import lib

asset = lib.asset_of("worker-carrying-box")
shell = next(s for s in asset["volumes"]["50"]["body"] if s["id"] == "box")
inner = shell["holes"][0]
load = {"id": "box-load", "kind": "extrude", "bone": "box-r", "material": "cardboard", "role": "body", "tone": 0.6, "plane": "xy", "profile": inner, "from": round(shell["from"] + 0.06, 4), "to": round(shell["to"] - 0.06, 4)}
ops = [
    {"op": "solid-add", "scale": "50", "after": "box-end-l", "solid": load},
    {"op": "solid-add", "scale": "50", "list": "structure", "after": "st-box-end-l", "solid": {"id": "st-box-load", "of": "box-load", "member": "box-r", "part": "member"}},
]
lib.write_ops("worker-carrying-box", ops, "hand")
