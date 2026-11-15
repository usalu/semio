import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-52-asset.py")).read())


def f(a):
    g = a["representations"]["50"]["geometry"]
    if not any(s["id"] == "end-cap-far" for s in g["elevation"]):
        i = [s["id"] for s in g["elevation"]].index("convector-rear")
        g["elevation"].insert(i, {"id": "end-cap-far", "bone": "radiator", "role": "detail", "filled": True, "region": "panel-rear", "material": "painted-steel", "kind": "poly", "points": [[0, -0.022], [0, -0.078], [0.585, -0.078], [0.585, -0.022]], "closed": True, "penMm": 0.1})
    for sc in ("50", "100"):
        for sh in a["representations"][sc]["geometry"]["front"]:
            if sh["id"] == "valve-head":
                sh["y"] = 0.058


patch("panel-radiator", f)
