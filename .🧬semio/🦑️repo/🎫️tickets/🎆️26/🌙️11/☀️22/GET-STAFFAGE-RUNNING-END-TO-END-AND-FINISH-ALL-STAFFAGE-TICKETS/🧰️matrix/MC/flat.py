"""🧭️ Static view of what a pose or clip moves in an asset document and how the three views draw it: `python3 flat.py <asset.json> [scale]` prints the bones that any pose, clip track or the root turns or shifts (with the largest angle), and per band and view the shapes of every bone with kind, flatness (solid, followTip or flat), fill and local box, grouped per bone so that a part one view lacks shows."""
import json, sys
sys.path.insert(0, __import__("os").path.dirname(__file__))
from lib import box
a = json.load(open(sys.argv[1]))["asset"]
want = sys.argv[2] if len(sys.argv) > 2 else None
moved = {}
def note(bone, off, where):
    for k, v in off.items():
        if isinstance(v, (int, float)) and abs(v) > 1e-9:
            cur = moved.setdefault(bone, {}).setdefault(k, [0, where])
            if abs(v) > abs(cur[0]):
                moved[bone][k] = [v, where]
for p in a["poses"]:
    for b, off in p.get("bones", {}).items():
        note(b, off, p["id"])
    if "root" in p:
        note("<root>", p["root"], p["id"])
for c in a.get("clips", []):
    for t in c.get("tracks", []):
        for k in t.get("keys", []):
            note(t.get("bone", "<root>"), {t.get("property", t.get("channel", "?")): k.get("value", 0)} if "value" in k else {x: y for x, y in k.items() if x not in ("timeS", "t", "ease", "easing")}, "clip " + c["id"])
    for k in c.get("keys", []):
        for b, off in (k.get("bones") or {}).items():
            note(b, off, "clip " + c["id"])
        if "root" in k:
            note("<root>", k["root"], "clip " + c["id"])
print("moved:")
for b, m in moved.items():
    print("  ", b, {k: (round(v[0], 3), v[1]) for k, v in m.items()})
parents = {b["id"]: b.get("parent") for b in a["rig"]["bones"]}
for key, rep in a["representations"].items():
    if "geometry" not in rep or (want and key != want):
        continue
    print("== 1:" + key)
    for bone in parents:
        rows = []
        for view in ("elevation", "front", "plan"):
            for s in rep["geometry"][view]:
                if s["bone"] != bone:
                    continue
                try:
                    b = box(s)
                    ext = "x[%.3f,%.3f] y[%.3f,%.3f]" % tuple(b)
                except Exception as e:
                    ext = "?"
                kind = ("solid:" + s["solid"]["kind"][:3] + ":" + s["solid"].get("sweep", s["solid"].get("axis", "")) + (" z[%s,%s]" % (s["solid"].get("z0"), s["solid"].get("z1")) if "z0" in s["solid"] else "")) if "solid" in s else ("followTip" if s.get("followTip") else "flat")
                rows.append(f"    {view[:4]} {s['id']:<28} {s['kind']:<7} {'fill' if s.get('filled') else 'line'} {kind:<34} {ext} z{s.get('z')}")
        if rows:
            print("  bone", bone, "<-", parents[bone])
            print("\n".join(rows))
