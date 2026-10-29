# [scratch] rest-pose forward kinematics of the horse rig and sole-anchor local coordinates
import sys, math, json, importlib.util
sys.path.insert(0, "..")
import kin
exec(open("gen-rig.py").read().split("out =")[0].replace("print(sum", "(sum"))
def sole(absdeg, L, half):
    a = math.radians(absdeg)
    tx, ty = L*math.cos(a), L*math.sin(a)
    wx, wy = tx - half, ty
    c, s = math.cos(-a), math.sin(-a)
    return wx*c - wy*s, wx*s + wy*c
if __name__ == "__main__":
    print("hind sole", sole(-50, 0.09, 0.06), "fore sole", sole(-52, 0.095, 0.065))
    anchors = []
    for s in "nf":
        hx, hy = sole(-50, .09, .06); fx, fy = sole(-52, .095, .065)
        anchors += [{"id": f"contact-h{s}", "bone": f"hoof-h{s}", "x": hx, "y": hy}, {"id": f"contact-f{s}", "bone": f"hoof-f{s}", "x": fx, "y": fy}]
    a = kin.Asset({"rig": {"bones": rows}, "anchors": anchors})
    w = a.fk({})
    for k in ("pelvis","thorax","neck-1","head","scapula-fn","humerus-fn","radius-fn","cannon-fn","pastern-fn","hoof-fn","thigh-hn","shank-hn","cannon-hn","pastern-hn","hoof-hn","tail-1","tail-4","ear-n"):
        print(k, [round(v,3) for v in w[k][4:]], round(kin.ang(w[k]),1))
    for k in a.anchors: print(k, [round(v,3) for v in a.anchor(w,k)])
