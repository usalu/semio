from lib import *

sc = sys.argv[1]
for i in sys.argv[2:]:
    doc, _ = read_doc(i)
    a = doc["asset"]
    w = rest_world(a)
    print("==", i, a["dimensions"])
    for s in a["representations"][sc]["geometry"]["elevation"]:
        if s["role"] != "body" or not s["filled"]:
            continue
        ox, oy, ang = w[s["bone"]]
        c, sn = math.cos(math.radians(ang)), math.sin(math.radians(ang))
        pts = [(ox + px * c - py * sn, oy + px * sn + py * c) for px, py in shape_pts(s, 6)]
        xs = [p[0] for p in pts]
        ys = [p[1] for p in pts]
        print("  %-18s %-12s x[%6.2f,%6.2f] y[%6.2f,%6.2f] c(%5.2f,%5.2f) r(%4.2f,%4.2f) z%s t%s" % (s["id"], s["bone"], min(xs), max(xs), min(ys), max(ys), (min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2, (max(xs) - min(xs)) / 2, (max(ys) - min(ys)) / 2, s["z"], s.get("tone")))
