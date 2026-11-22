"""📏 Measures the 1:50 candidate against the reference table: bass outline, depths, lengths, bridge, strings, cover, stool and masses."""
import json, math
d = json.load(open("🔣️.json"))["asset"]
b = {s["id"]: s for s in d["volumes"]["50"]["body"]}
def ext(s):
    if s["kind"] == "polyhedron":
        v = s["vertices"]; return [min(p[i] for p in v) for i in range(3)], [max(p[i] for p in v) for i in range(3)]
    if s["kind"] == "extrude" and s["plane"] == "xy":
        p = s["profile"]; return [min(a for a, _ in p), min(c for _, c in p), s["from"]], [max(a for a, _ in p), max(c for _, c in p), s["to"]]
lo, hi = ext(b["body"]); tlo, thi = ext(b["top"])
print(f"body length {hi[0]-lo[0]:.3f}  lower bout {hi[1]-lo[1]:.3f}  ribs+back depth {hi[2]-lo[2]:.3f}  arch {-(tlo[2]+0.100):.3f}")
import importlib.util, sys
spec = importlib.util.spec_from_file_location("m", "author-double-bass.py"); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
o = m.outline(50)
for u in (0.27, 0.575, 0.88): print(f"bout width at u={u}: {2*m.half_at(o,u):.3f}")
sp = m.string_span("d", 50)[1]; print(f"string length bridge-nut {math.dist((sp[0],sp[2]),(sp[3],sp[5])):.3f}")
print(f"total length socket..scroll {m.SCROLL_TOP - 0 + 0.0:.3f} + socket 0.025; bridge height above top {(-0.2835)-(-0.1212):.4f}")
print(f"fingerboard length {m.NUT_U - m.FB_END:.3f}; after-length {m.BRIDGE_U - (m.TAIL_U[1]-0.010):.3f}")
print("dims", d["dimensions"]); print("bone masses", {x['id']: x.get('mass',{}).get('kg') for x in d['rig']['bones'] if 'mass' in x}, "sum", round(sum(x['mass']['kg'] for x in d['rig']['bones'] if 'mass' in x),3))
m.main()
print("seat top", m.SEAT_Y + 0.060, "delta", round(m.DELTA, 2), "pivot", m.Q_PIVOT)
