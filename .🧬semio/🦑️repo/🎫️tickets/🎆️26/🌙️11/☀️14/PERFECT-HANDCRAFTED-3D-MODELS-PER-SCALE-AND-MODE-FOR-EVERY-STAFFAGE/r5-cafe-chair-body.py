"""🪵 Round 5, furniture/cafe-chair: the round-3 handcrafted body with the round-5 corrections stated by hand.

At 1:100, 1:200 and 1:500 the domed end caps merge into the tubes (as the decisions of those scales say), so each post runs to the top of its cap, 8 mm further along the post axis; at 1:50 the cap stays its own solid.
@see ./r3-cafe-chair-body.py
@see ./r5-brief.md
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_cafe_chair_body", os.path.join(HERE, "r3-cafe-chair-body.py"))
R3 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(R3)
globals().update({k: v for k, v in vars(R3).items() if not k.startswith("__")})

CAPPED_TOP = K.add(K.POST_TOP, K.POST_DIR, K.CAP_H)


def capped(solids):
    """🎩 The posts of a coarse scale run on through the merged cap: the last section moves from the tube top to the cap top."""
    out = []
    for s in solids:
        if s["id"].startswith("post-"):
            s = dict(s, sections=[dict(sec) for sec in s["sections"]])
            side = 1 if s["id"].endswith("-l") else -1
            top = K.local("back", (CAPPED_TOP[0], CAPPED_TOP[1], side * K.POST_Z))
            s["sections"][-1]["at"] = [K.r6(c) for c in top]
        out.append(s)
    return out


def body100():
    return capped(R3.body100())


def body200():
    return capped(R3.body200())


def body500():
    return capped(R3.body500())
