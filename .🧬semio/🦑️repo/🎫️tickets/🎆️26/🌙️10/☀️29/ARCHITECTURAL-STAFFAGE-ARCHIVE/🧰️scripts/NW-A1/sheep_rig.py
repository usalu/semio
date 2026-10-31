from quad import *

P = dict(
    foot="hoof",
    hind=[(0.21, -64), (0.22, -108), (0.15, -82), (0.062, -52)],
    fore=[(0.17, -62), (0.17, -115), (0.20, -85), (0.14, -90), (0.06, -53)],
    hind_sole_x=0.07, fore_sole_x=0.62,
    sacrum_dx=0.08, thorax_x=0.36, neck_x=0.64,
    neck=(0.31, 14), head=(0.22, -48), jaw=((0.07, -0.045), 0.12, -4), ear=((0.03, 0.04), 0.09, -20),
    tail_root=(-0.03, 0.06), tail=[(0.1, -100), (0.09, -95)],
    lim=dict(pelvis=(-25, 25), lumbar=(-15, 20), thorax=(-12, 12), neck=(-85, 55), head=(-70, 55), jaw=(-30, 2), ear=(-40, 50), tail=(-60, 60),
             thigh=(-60, 100), shank=(-110, 60), metatarsus=(-100, 175), foot=(-60, 50), scapula=(-35, 35), humerus=(-120, 70), forearm=(-40, 130), metacarpus=(-160, 180)),
    titles={},
)
if __name__ == "__main__":
    P["titles"] = {k: T(k, k) for k in ["pelvis", "lumbar", "thorax", "neck", "head", "jaw", "ear-n", "ear-f", "tail-1", "tail-2"]}
    P["titles"]["hind"] = [(a, a) for a in "abcd"]
    P["titles"]["fore"] = [(a, a) for a in "abcde"]
    sk = build_rig(P)
    for b in sk.bones:
        if b["id"].endswith("-hf") or b["id"].endswith("-ff"):
            continue
        print(b["id"], [round(v, 3) for v in sk.w0[b["id"]]], "->", [round(v, 3) for v in sk.end(b["id"])], round(sk.aw[b["id"]], 1))
