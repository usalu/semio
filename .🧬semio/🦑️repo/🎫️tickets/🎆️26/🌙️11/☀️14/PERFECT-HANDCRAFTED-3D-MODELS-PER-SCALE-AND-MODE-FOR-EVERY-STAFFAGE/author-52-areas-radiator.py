import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-52-asset.py")).read())


def f(a):
    for r in a["thermal"]["regions"]:
        if r["id"] == "convector":
            r["areaBasis"] = "exchange"
        if r["id"] == "top-grille":
            r["areaM2"] = 0.14


patch("panel-radiator", f)
