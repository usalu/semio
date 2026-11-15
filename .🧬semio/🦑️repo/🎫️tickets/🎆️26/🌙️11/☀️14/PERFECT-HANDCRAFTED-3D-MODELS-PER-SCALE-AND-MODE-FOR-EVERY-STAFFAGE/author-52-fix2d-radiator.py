import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-52-asset.py")).read())


def f(a):
    for sc in ("50", "100", "200"):
        for sh in a["representations"][sc]["geometry"]["front"]:
            if sh["id"] == "radiator-outline":
                sh["filled"] = True
    a["representations"]["50"]["sections"]["elevation"]["retained"] = "near"


patch("panel-radiator", f)
