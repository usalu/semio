import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-52-asset.py")).read())


def f(a):
    for r in a["thermal"]["regions"]:
        if r["id"] in ("glass-centre", "glass-edge", "mullion", "transom"):
            r["layers"] = list(reversed(r["layers"]))


patch("curtain-wall-bay", f)
