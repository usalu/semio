"""Rest frames of a planar rig in staffage world (x ahead, y up, z left): bone origin = parent origin + x u + y v, angle accumulates, w = +z."""
import math
def rest_frames(bones):
    by = {b["id"]: b for b in bones}
    out = {}
    def go(i):
        if i in out: return out[i]
        b = by[i]
        if b.get("parent") is None:
            o = (b["x"], b["y"], 0.0); a = math.radians(b["angleDeg"])
        else:
            po, pa = go(b["parent"])
            u = (math.cos(pa), math.sin(pa)); v = (-math.sin(pa), math.cos(pa))
            o = (po[0] + b["x"] * u[0] + b["y"] * v[0], po[1] + b["x"] * u[1] + b["y"] * v[1], 0.0)
            a = pa + math.radians(b["angleDeg"])
        out[i] = (o, a)
        return out[i]
    for i in by: go(i)
    return out
def to_world(frame, p):
    (o, a) = frame
    c, s = math.cos(a), math.sin(a)
    return (o[0] + p[0] * c - p[1] * s, o[1] + p[0] * s + p[1] * c, o[2] + p[2])
