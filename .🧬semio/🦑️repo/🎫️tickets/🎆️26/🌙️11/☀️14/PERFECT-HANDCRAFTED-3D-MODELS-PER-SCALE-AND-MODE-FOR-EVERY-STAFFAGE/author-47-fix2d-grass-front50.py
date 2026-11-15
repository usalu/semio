"""Author 47: miscanthus 1:50 front view. The two blades of a mirror pair (-1 at z<0, -2 at z>0) are drawn with different bands (the -1 band is 0.022 m thick, the -2 band 0.04 m, while the plan draws both 0.055 m wide); the pairs are mirror-symmetric by construction, so -1 becomes the mirror of -2 (separate b shapes) and the merged V of an a shape becomes the right half plus its mirror."""
import importlib.util, json
spec = importlib.util.spec_from_file_location("l", "author-47-lib2d.py"); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
P = l.path("vegetation", "miscanthus-grass")
t = open(P).read()

def clip_right(pts):
    out = []
    n = len(pts)
    for i in range(n):
        a, b = pts[i], pts[(i + 1) % n]
        ia, ib = a[0] >= 0, b[0] >= 0
        if ia: out.append(a)
        if ia != ib:
            u = (0 - a[0]) / (b[0] - a[0])
            out.append([0.0, a[1] + u * (b[1] - a[1])])
    return out

def sym_ring(R):
    onaxis = [i for i, p in enumerate(R) if p[0] == 0.0]
    mirrored = [[-p[0], p[1]] for p in R]
    if len(onaxis) == 2 and (onaxis[1] - onaxis[0] == 1 or (onaxis[0] == 0 and onaxis[1] == len(R) - 1)):
        i, j = onaxis
        if (i, j) == (0, len(R) - 1):
            R = R[:]
        else:
            R = R[j:] + R[:j]
            mirrored = [[-p[0], p[1]] for p in R]
        k = [m for m in mirrored[1:-1]]
        return R + k[::-1]
    return R + mirrored[::-1]

n = 0
def has(sid):
    try:
        l.shape_range(t, "50", "front", sid); return True
    except ValueError:
        return False
for b in ("l1", "l2", "l3", "r1", "r2", "r3"):
    for k in (("2", "4") if b[0] == "l" else ("3", "4")):
        for seg in ("a", "b"):
            sid = "blade-%s-%s%s" % (b, k, seg)
            if has(sid + "-1") and has(sid + "-2"):
                s2, _, _ = l.get_shape(t, "50", "front", sid + "-2")
                s1, _, _ = l.get_shape(t, "50", "front", sid + "-1")
                s1["points"] = [[-p[0], p[1]] for p in s2["points"]][::-1]
                t = l.put_shape(t, "50", "front", sid + "-1", s1); n += 1
            elif has(sid):
                s, _, _ = l.get_shape(t, "50", "front", sid)
                R = clip_right(s["points"])
                if len(R) < 3: print("skip", sid); continue
                s["points"] = [[round(x, 4), round(y, 4)] for x, y in sym_ring(R)]
                t = l.put_shape(t, "50", "front", sid, s); n += 1
                print("merged", sid)
open(P, "w").write(t)
print(n, "front shapes mirrored")
