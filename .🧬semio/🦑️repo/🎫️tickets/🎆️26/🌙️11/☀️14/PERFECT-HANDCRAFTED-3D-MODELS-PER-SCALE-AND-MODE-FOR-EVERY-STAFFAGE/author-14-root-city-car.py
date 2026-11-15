from author_14_root import patch

def tp(o):
    o["x1"] = -1.925

def tpp(o):
    if o.get("x1") is not None:
        o["x0"], o["x1"] = -1.9, -1.925

print(patch("city-car", [(sc, "elevation", "tail-pipe", tp) for sc in ("50", "100")] + [(sc, "plan", "tail-pipe", tpp) for sc in ("50", "100")]))
