from wl import *

def wood(R, A, fine):
    o = member(R, A, ["root-plate", "trunk", "trunk-upper", "leader", "leader-top"], "trunk-bark", z=2, tone=0.35, rs=1.0, radii=[0.70, 0.46, 0.41, 0.34, 0.22, 0.09], zones=[0, 0, 0.9, 1.3, 1.2, 0], ids=["root", "bole", "bole-upper", "leader", "leader-top"])
    for s in "we":
        o += member(R, A, [f"limb-{s}-low", f"br-{s}-low"], "limb-bark", z=1, tone=0.4, back=0.5, radii=[0.28, 0.2, 0.07])
        o += member(R, A, [f"limb-{s}-mid", f"br-{s}-mid"], "limb-bark", z=1, tone=0.4, back=0.4, radii=[0.24, 0.17, 0.07])
        if fine:
            o += member(R, A, [f"br-leader-{s}"], "limb-bark", z=1, tone=0.4, back=0.3, radii=[0.14, 0.06])
            o += member(R, A, [f"br-low-{s}-up"], "limb-bark", z=1, tone=0.4, back=0.3, radii=[0.12, 0.05])
    return o

def design(R, A):
    res = {}
    for k, fine in (("200", False), ("100", True)):
        keep = prune(A["representations"][k]["geometry"]["elevation"], ["leaf-sun", "leaf-shade"], ["fol-"])
        res[k] = wood(R, A, fine) + keep
        if k == "200": xf(R, res[k], "core-fol", 1.04)
    return res
