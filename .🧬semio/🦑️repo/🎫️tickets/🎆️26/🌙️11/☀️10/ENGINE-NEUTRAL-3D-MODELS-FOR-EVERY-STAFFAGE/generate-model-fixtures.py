import json, math, os, sys
root = sys.argv[1]
QUALITY = {"chordM": 0.0005, "minSegments": 24, "maxSegments": 128}
def T(en, de): return {"en": en, "de": de}
MAT = {"id": "skin", "title": T("Skin", "Haut"), "baseColor": [0.8, 0.7, 0.6, 1], "metallic": 0, "roughness": 0.8, "emissive": [0, 0, 0], "alphaMode": "opaque", "doubleSided": False, "unlit": False}
def node(id, parent=None, t=(0, 0, 0), r=(0, 0, 0, 1), s=(1, 1, 1), **kw):
    return {"id": id, "parent": parent, "translation": list(t), "rotation": list(r), "scale": list(s), **kw}
def doc(id, solids, nodes=None, **kw):
    d = {"version": "semio.model/v1", "id": id, "title": T(id, id), "nodes": nodes or [node("root")], "solids": solids, "meshes": [], "skins": [], "materials": [MAT], "variants": [], "poses": [], "animations": []}
    d.update(kw); return d
def solid(id, kind, **kw): return {"id": id, "kind": kind, "node": "root", "material": "skin", **kw}
def write(case, data):
    p = os.path.join(root, case); os.makedirs(p, exist_ok=True)
    open(os.path.join(p, "🔣️.json"), "w").write(json.dumps(data, indent=1, ensure_ascii=False) + "\n")
def case(name, d, volume, bounds, tol=0.02, closed=True, **extra):
    return {"name": name, "document": d, "expected": {"volume": volume, "bounds": bounds, "closed": closed, **extra}, "volumeRelativeTolerance": tol, "boundsTolerance": 0.002 if tol > 1e-6 else 1e-6}

def box_rounded(a, b, c, r):
    x, y, z = a - 2 * r, b - 2 * r, c - 2 * r
    return x*y*z + 2*r*(x*y + y*z + x*z) + math.pi*r*r*(x + y + z) + 4/3*math.pi*r**3
def rotY(p, a): return [p[0]*math.cos(a) + p[2]*math.sin(a), p[1], -p[0]*math.sin(a) + p[2]*math.cos(a)]

write("📦️box", {"quality": QUALITY, "cases": [
  case("box", doc("box", [solid("b", "box", size=[0.4, 0.6, 0.8])]), 0.192, {"min": [-0.2, -0.3, -0.4], "max": [0.2, 0.3, 0.4]}, tol=1e-6, triangles=12),
  case("rounded-box", doc("rounded", [solid("b", "box", size=[0.4, 0.6, 0.8], cornerRadius=0.1)]), box_rounded(0.4, 0.6, 0.8, 0.1), {"min": [-0.2, -0.3, -0.4], "max": [0.2, 0.3, 0.4]}, tol=0.01),
  case("transformed-box", doc("transformed", [solid("b", "box", size=[1, 2, 3], transform={"translation": [1, 0, 0], "rotation": [0, math.sin(math.pi/4), 0, math.cos(math.pi/4)]})]), 6, {"min": [-0.5, -1, -0.5], "max": [2.5, 1, 0.5]}, tol=1e-6),
  case("mirrored-box", doc("mirrored", [solid("b", "box", size=[1, 1, 1], transform={"scale": [-1, 1, 1], "translation": [0, 0, 2]})]), 1, {"min": [-0.5, -0.5, 1.5], "max": [0.5, 0.5, 2.5]}, tol=1e-6),
]})
write("🥚️ellipsoid", {"quality": QUALITY, "cases": [
  case("ellipsoid", doc("ellipsoid", [solid("e", "ellipsoid", radii=[0.3, 0.5, 0.2])]), 4/3*math.pi*0.3*0.5*0.2, {"min": [-0.3, -0.5, -0.2], "max": [0.3, 0.5, 0.2]}, tol=0.01),
  case("sphere", doc("sphere", [solid("e", "ellipsoid", radii=[0.25, 0.25, 0.25])]), 4/3*math.pi*0.25**3, {"min": [-0.25]*3, "max": [0.25]*3}, tol=0.01),
]})
def capsule_vol(r0, r1, d):
    if abs(r0 - r1) < 1e-12: return math.pi*r0*r0*d + 4/3*math.pi*r0**3
    sb = (r0 - r1)/d; cb = math.sqrt(1 - sb*sb)
    z0, z1 = r0*sb, d + r1*sb
    h = z1 - z0; p0, p1 = r0*cb, r1*cb
    frustum = math.pi*h/3*(p0*p0 + p0*p1 + p1*p1)
    H0 = r0 + z0; H1 = d + r1 - z1
    return frustum + math.pi*H0*H0*(3*r0 - H0)/3 + math.pi*H1*H1*(3*r1 - H1)/3
write("💊️capsule", {"quality": QUALITY, "cases": [
  case("capsule", doc("capsule", [solid("c", "capsule", p0=[0, 0, 0], p1=[0, 1, 0], r0=0.2, r1=0.2)]), capsule_vol(0.2, 0.2, 1), {"min": [-0.2, -0.2, -0.2], "max": [0.2, 1.2, 0.2]}, tol=0.01),
  case("tapered-capsule", doc("tapered", [solid("c", "capsule", p0=[0, 0, 0], p1=[0, 1, 0], r0=0.3, r1=0.1)]), capsule_vol(0.3, 0.1, 1), {"min": [-0.3, -0.3, -0.3], "max": [0.3, 1.1, 0.3]}, tol=0.01),
  case("lateral-scale", doc("lateral", [solid("c", "capsule", p0=[0, 0, 0], p1=[0, 1, 0], r0=0.2, r1=0.2, lateralScale=0.5)]), capsule_vol(0.2, 0.2, 1)*0.5, {"min": [-0.1, -0.2, -0.2], "max": [0.1, 1.2, 0.2]}, tol=0.01),
  case("cone-tip", doc("cone", [solid("c", "capsule", p0=[0, 0, 0], p1=[0, 1, 0], r0=0, r1=0.2)]), capsule_vol(0.0, 0.2, 1), {"min": [-0.2, -0.0, -0.2], "max": [0.2, 1.2, 0.2]}, tol=0.02),
]})
def frustums(profile):
    v = 0
    for (a0, r0), (a1, r1) in zip(profile, profile[1:]): v += math.pi*(a1 - a0)/3*(r0*r0 + r0*r1 + r1*r1)
    return v
vase = [[0, 0], [0.2, 0.3], [0.7, 0.15], [1, 0]]
circle = [[0.1*math.sin(2*math.pi*k/48), 0.3 + 0.1*math.cos(2*math.pi*k/48)] for k in range(48)]
circle_area = 0.5*sum(circle[k][0]*circle[(k+1)%48][1] - circle[(k+1)%48][0]*circle[k][1] for k in range(48))
write("🌀️revolve", {"quality": QUALITY, "cases": [
  case("cylinder", doc("cylinder", [solid("r", "revolve", profile=[[0, 0.2], [1, 0.2]], axis=[0, 1, 0], origin=[0, 0, 0])]), math.pi*0.04, {"min": [-0.2, 0, -0.2], "max": [0.2, 1, 0.2]}, tol=0.01),
  case("vase", doc("vase", [solid("r", "revolve", profile=vase, axis=[0, 1, 0], origin=[0, 0, 0])]), frustums(vase), {"min": [-0.3, 0, -0.3], "max": [0.3, 1, 0.3]}, tol=0.01, lateralArea=True),
  case("half-sweep", doc("half", [solid("r", "revolve", profile=vase, axis=[0, 1, 0], origin=[0, 0, 0], sweepRad=math.pi)]), frustums(vase)/2, {"min": [-0.3, 0, -0.3], "max": [0.3, 1, 0]}, tol=0.01),
  case("torus", doc("torus", [solid("r", "revolve", profile=circle + [circle[0]], axis=[0, 1, 0], origin=[0, 0, 0])]), 2*math.pi*0.3*abs(circle_area)*(1 + 0), {"min": [-0.4, -0.1, -0.4], "max": [0.4, 0.1, 0.4]}, tol=0.005),
]})
sq = [[0, 0], [1, 0], [1, 1], [0, 1]]
hole = [[0.25, 0.25], [0.25, 0.75], [0.75, 0.75], [0.75, 0.25]]
L = [[0, 0], [1, 0], [1, 0.4], [0.4, 0.4], [0.4, 1], [0, 1]]
write("🏗️extrude", {"quality": QUALITY, "cases": [
  case("square-with-hole", doc("hole", [solid("x", "extrude", profile=sq, holes=[hole], plane="xy", **{"from": 0, "to": 0.5})]), 0.75*0.5, {"min": [0, 0, 0], "max": [1, 1, 0.5]}, tol=1e-6),
  case("l-shape-yz", doc("lshape", [solid("x", "extrude", profile=L, plane="yz", **{"from": -1, "to": 1})]), 0.64*2, {"min": [-1, 0, 0], "max": [1, 1, 1]}, tol=1e-6, oracle="ExtrudeGeometry"),
  case("tapered", doc("taper", [solid("x", "extrude", profile=[[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]], plane="zx", **{"from": 0, "to": 2}, taperTo=[1, 0.5])]), 2/3*(1 + 0.5 + 0.25), {"min": [-0.5, 0, -0.5], "max": [0.5, 2, 0.5]}, tol=1e-6),
  case("pyramid", doc("pyramid", [solid("x", "extrude", profile=[[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]], plane="xy", **{"from": 0, "to": 1}, taperTo=[1, 0])]), 1/3, {"min": [-0.5, -0.5, 0], "max": [0.5, 0.5, 1]}, tol=1e-6),
  case("collinear-midpoints", doc("midpoints", [solid("x", "extrude", profile=[[0, 0], [0.5, 0], [1, 0], [1, 0.5], [1, 1], [0.5, 1], [0, 1], [0, 0.5]], plane="xy", **{"from": 0, "to": 0.5})]), 0.5, {"min": [0, 0, 0], "max": [1, 1, 0.5]}, tol=1e-6, oracle="ExtrudeGeometry"),
  case("near-duplicate-points", doc("neardup", [solid("x", "extrude", profile=[[0, 0], [1, 0], [1, 1e-5], [1, 1], [0, 1]], plane="xy", **{"from": 0, "to": 0.5})]), 0.5, {"min": [0, 0, 0], "max": [1, 1, 0.5]}, tol=1e-6),
  case("closed-loop-profile", doc("loop", [solid("x", "extrude", profile=[[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]], plane="xy", **{"from": 0, "to": 0.5})]), 0.5, {"min": [0, 0, 0], "max": [1, 1, 0.5]}, tol=1e-6, oracle="ExtrudeGeometry"),
  case("two-aligned-holes", doc("aligned", [solid("x", "extrude", profile=[[-2, 0.06], [2, 0.06], [2, 1.26], [-2, 1.26]], holes=[[[-1.965, 0.095], [-0.015, 0.095], [-0.015, 1.225], [-1.965, 1.225]], [[0.015, 0.095], [1.965, 0.095], [1.965, 1.225], [0.015, 1.225]]], plane="xy", **{"from": -0.005, "to": 0.04})]), (4 * 1.2 - 2 * 1.95 * 1.13) * 0.045, {"min": [-2, 0.06, -0.005], "max": [2, 1.26, 0.04]}, tol=1e-6),
  case("hole-with-collinear-points", doc("holemid", [solid("x", "extrude", profile=sq, holes=[[[0.25, 0.25], [0.25, 0.5], [0.25, 0.75], [0.75, 0.75], [0.75, 0.25]]], plane="xy", **{"from": 0, "to": 0.5})]), 0.75*0.5, {"min": [0, 0, 0], "max": [1, 1, 0.5]}, tol=1e-6),
]})
sec = lambda z, rx=0.2, ry=0.2, e=None: {"at": [0, 0, z], "rx": rx, "ry": ry, **({"exponent": e} if e else {})}
def vsub(a, b): return [a[i] - b[i] for i in range(3)]
def vdot(a, b): return sum(a[i] * b[i] for i in range(3))
def vcross(a, b): return [a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0]]
def vunit(a):
    l = math.sqrt(vdot(a, a)); return [x / l for x in a]
def qrot(q, v):
    x, y, z, w = q
    t = [2 * (y*v[2] - z*v[1]), 2 * (z*v[0] - x*v[2]), 2 * (x*v[1] - y*v[0])]
    return [v[0] + w*t[0] + (y*t[2] - z*t[1]), v[1] + w*t[1] + (z*t[0] - x*t[2]), v[2] + w*t[2] + (x*t[1] - y*t[0])]
def qaxis(axis, a): return [axis[0]*math.sin(a/2), axis[1]*math.sin(a/2), axis[2]*math.sin(a/2), math.cos(a/2)]
def path_frame(t):
    x = [1 - t[0]*t[0], -t[0]*t[1], -t[0]*t[2]]
    if math.sqrt(vdot(x, x)) < 1e-6: x = [-t[2]*t[0], -t[2]*t[1], 1 - t[2]*t[2]]
    x = vunit(x)
    return x, vcross(t, x)
def lsec(at, rx=0.3, ry=0.1, q=None): return {"at": at, "rx": rx, "ry": ry, **({"rotation": q} if q else {})}
def loft_case(name, secs, volume=None, closed=True, **extra):
    n = len(secs)
    frames = []
    for i, s_ in enumerate(secs):
        if "rotation" in s_: x, y = qrot(s_["rotation"], [1, 0, 0]), qrot(s_["rotation"], [0, 1, 0])
        else: x, y = path_frame(vunit(vsub(secs[min(n - 1, i + 1)]["at"], secs[max(0, i - 1)]["at"])))
        frames.append({"x": x, "y": y})
    lo, hi = [1e9] * 3, [-1e9] * 3
    for s_, f in zip(secs, frames):
        for k in range(3):
            half = math.hypot(s_["rx"] * f["x"][k], s_["ry"] * f["y"][k])
            lo[k] = min(lo[k], s_["at"][k] - half); hi[k] = max(hi[k], s_["at"][k] + half)
    return case(name, doc(name, [solid("l", "loft", sections=secs, capStart=True, capEnd=True)]), volume, {"min": lo, "max": hi}, tol=0.01, closed=closed, frames=frames, **extra)
ELL = math.pi * 0.3 * 0.1
write("🧵️loft", {"quality": QUALITY, "cases": [
  case("cylinder", doc("cyl", [solid("l", "loft", sections=[sec(0), sec(0.5), sec(1)], capStart=True, capEnd=True)]), math.pi*0.04, {"min": [-0.2, -0.2, 0], "max": [0.2, 0.2, 1]}, tol=0.01, oracle="CylinderGeometry"),
  case("frustum", doc("frustum", [solid("l", "loft", sections=[sec(0, 0.3, 0.3), sec(1, 0.1, 0.1)], capStart=True, capEnd=True)]), math.pi/3*(0.09 + 0.03 + 0.01), {"min": [-0.3, -0.3, 0], "max": [0.3, 0.3, 1]}, tol=0.01, oracle="CylinderGeometry"),
  case("superellipse", doc("super", [solid("l", "loft", sections=[sec(0, 0.3, 0.2, 4), sec(1, 0.3, 0.2, 4)], capStart=True, capEnd=True)]), 4*0.3*0.2*(math.gamma(1.25)**2/math.gamma(1.5)), {"min": [-0.3, -0.2, 0], "max": [0.3, 0.2, 1]}, tol=0.01),
  case("reversed-direction", doc("rev", [solid("l", "loft", sections=[sec(1), sec(0)], capStart=True, capEnd=True)]), math.pi*0.04, {"min": [-0.2, -0.2, 0], "max": [0.2, 0.2, 1]}, tol=0.01),
  case("open-tube", doc("tube", [solid("l", "loft", sections=[sec(0), sec(1)], capStart=False, capEnd=False)]), None, {"min": [-0.2, -0.2, 0], "max": [0.2, 0.2, 1]}, tol=0.01, closed=False),
  loft_case("along-x", [lsec([0, 0, 0]), lsec([0.5, 0, 0]), lsec([1, 0, 0])], ELL),
  loft_case("along-y", [lsec([0, 0, 0]), lsec([0, 1, 0])], ELL),
  loft_case("diagonal", [lsec([0, 0, 0]), lsec([0.3, 0.4, 0]), lsec([0.6, 0.8, 0])], ELL),
  loft_case("diagonal-circle", [lsec([0, 0, 0], 0.2, 0.2), lsec([0.3, 0.4, 0], 0.2, 0.2), lsec([0.6, 0.8, 0], 0.2, 0.2)], math.pi * 0.04, oracle="CylinderGeometry"),
  loft_case("bent-path", [lsec([0, 0, 0]), lsec([0, 0, 1]), lsec([0, 1, 1])]),
  loft_case("mixed-twist", [lsec([0, 0, 0]), lsec([0, 0, 0.5], q=qaxis([0, 0, 1], math.pi / 6)), lsec([0, 0, 1])]),
  loft_case("mixed-consistent", [lsec([0, 0, 0]), lsec([0, 1, 0], q=qaxis([1, 0, 0], -math.pi / 2)), lsec([0, 2, 0])], 2 * ELL),
]})
b = solid("b", "box", size=[0.2, 0.2, 0.2], repeat={"count": 4, "translation": [0.5, 0, 0]})
b2 = solid("b", "box", size=[0.2, 0.2, 0.2], transform={"translation": [1, 0, 0]}, repeat={"count": 6, "rotationAxis": [0, 1, 0], "rotationRad": math.pi/3})
corners = [[rotY([1 + sx*0.1, sy*0.1, sz*0.1], k*math.pi/3) for sx in (-1, 1) for sy in (-1, 1) for sz in (-1, 1)] for k in range(6)]
flat = [c for cs in corners for c in cs]
RB = {"min": [min(c[i] for c in flat) for i in range(3)], "max": [max(c[i] for c in flat) for i in range(3)]}
def step_s(p):
    q = rotY([p[0] - 0.5, p[1], p[2]], 1.0)
    return [q[0] + 0.5, q[1] + 1, q[2]]
cs = [[0, 0, 0]]
for _ in range(2): cs.append(step_s(cs[-1]))
SB = {"min": [min(c[i] for c in cs) - 0.1 for i in range(3)], "max": [max(c[i] for c in cs) + 0.1 for i in range(3)]}
centers = [rotY([1, 0, 0], k*math.pi/3) for k in range(6)]
b3 = solid("b", "ellipsoid", radii=[0.1, 0.1, 0.1], repeat={"count": 3, "translation": [0, 1, 0], "rotationAxis": [0, 1, 0], "rotationRad": 1.0, "origin": [0.5, 0, 0]})
write("🔁️repeat", {"quality": QUALITY, "cases": [
  case("translation-steps", doc("rep-t", [b]), 4*0.008, {"min": [-0.1, -0.1, -0.1], "max": [1.6, 0.1, 0.1]}, tol=1e-6, triangles=48),
  case("rotation-steps", doc("rep-r", [b2]), 6*0.008, RB, tol=1e-6, copyCenters=centers),
  case("sphere-steps", doc("rep-s", [b3]), 3*4/3*math.pi*0.001, SB, tol=0.02),
]})

def qz(a): return [0, 0, math.sin(a/2), math.cos(a/2)]
def qy(a): return [0, math.sin(a/2), 0, math.cos(a/2)]
def qx(a): return [math.sin(a/2), 0, 0, math.cos(a/2)]
rev = lambda axis, lo, hi: {"type": "revolute", "axis": axis, "min": lo, "max": hi}
chain = [node("root"), node("upper", "root", t=(0, 1, 0), joint=rev([0, 0, 1], -1.0, 1.0)), node("lower", "upper", t=(0, 1, 0), joint=rev([0, 0, 1], -1.0, 1.0))]
skin_doc = doc("skinning", [
    solid("limb", "capsule", p0=[0, 0, 0], p1=[0, 2, 0], r0=0.2, r1=0.2, weights=[{"node": "upper", "weight": 1}, {"node": "lower", "weight": 3}]),
    {**solid("hand", "ellipsoid", radii=[0.1, 0.1, 0.1]), "node": "lower"},
    solid("rigid-root", "box", size=[0.2, 0.2, 0.2]),
], nodes=chain, poses=[{"id": "bent", "title": T("Bent", "Gebeugt"), "nodes": {"upper": {"rotation": qz(0.5)}, "lower": {"rotation": qz(-1.2)}}}])
bent_lower_pos = [-math.sin(0.5), 1 + math.cos(0.5), 0]
write("🦴️skinning", {"quality": QUALITY, "pose": "bent", "document": skin_doc, "expected": {
    "jointOrder": ["upper", "lower"], "weights": [0.25, 0.75, 0, 0], "skinnedMeshes": 1, "rigidMeshes": ["root", "lower"],
    "lowerWorldPosition": bent_lower_pos, "lowerWorldAngleZ": -0.5, "issues": [{"code": "joint-limit", "path": "/nodes/2/joint"}], "tolerance": 1e-9,
    "inverseBindLowerTranslation": [0, -2, 0]}})

mech = [
  node("root"),
  node("arm", "root", t=(0, 0, 0), joint=rev([0, 1, 0], -1.0, 1.0)),
  node("tip", "arm", t=(1, 0, 0)),
  node("slider", "root", t=(0, 0.5, 0), joint={"type": "prismatic", "axis": [1, 0, 0], "min": 0, "max": 0.3}),
  node("ball", "root", t=(0, 0, 1), joint={"type": "spherical", "axis": [0, 0, 1], "max": 0.5, "twistMin": -0.3, "twistMax": 0.3}),
  node("locked", "root", t=(0, 2, 0), joint={"type": "fixed", "axis": [0, 1, 0]}),
]
clip = lambda id, peak: {"id": id, "title": T(id, id), "durationS": 2, "loop": True, "channels": [
  {"target": {"node": "arm", "path": "rotation"}, "interpolation": "linear", "times": [0, 1, 2], "values": qy(0) + qy(peak) + qy(0)},
  {"target": {"node": "slider", "path": "translation"}, "interpolation": "step", "times": [0, 1], "values": [0, 0.5, 0, 0.2, 0.5, 0]},
  {"target": {"material": "skin", "path": "baseColor"}, "interpolation": "linear", "times": [0, 2], "values": [0, 0, 0, 1, 1, 1, 1, 1]},
], "events": [{"t": 1, "id": "peak"}]}
write("🎞️pose-animation", {"document": doc("mechanism", [], nodes=mech, animations=[clip("wave", 0.8), clip("overshoot", 1.5)]),
  "samples": [
    {"animation": "wave", "timeS": 0.5, "armAngleY": 0.4, "sliderX": 0.0, "baseColor": [0.25, 0.25, 0.25, 1.0], "issues": []},
    {"animation": "wave", "timeS": 1.0, "armAngleY": 0.8, "sliderX": 0.2, "baseColor": [0.5, 0.5, 0.5, 1.0], "issues": []},
    {"animation": "wave", "timeS": 1.5, "armAngleY": 0.4, "sliderX": 0.2, "baseColor": [0.75, 0.75, 0.75, 1.0], "issues": []},
    {"animation": "wave", "timeS": 2.5, "armAngleY": 0.4, "sliderX": 0.0, "baseColor": [0.25, 0.25, 0.25, 1.0], "issues": []},
    {"animation": "wave", "timeS": -0.5, "armAngleY": 0.4, "sliderX": 0.2, "baseColor": [0.75, 0.75, 0.75, 1.0], "issues": []},
    {"animation": "overshoot", "timeS": 1.0, "armAngleY": 1.0, "sliderX": 0.2, "baseColor": [0.5, 0.5, 0.5, 1.0], "issues": ["joint-limit"]},
  ],
  "tipWorld": {"description": "tip = arm rotated about +Y by the angle, then (1,0,0)", "formula": "[cos(a), 0, -sin(a)]"},
  "joints": {
    "armBeyondLimit": {"angleY": 1.4, "clampedAngleY": 1.0},
    "sliderBeyondLimit": {"x": 0.9, "clampedX": 0.3, "slideNegative": -0.2, "clampedNegativeX": 0.0},
    "ballTwist": {"twistAboutZ": 0.6, "clampedTwist": 0.3},
    "ballSwing": {"swingAboutX": 0.9, "clampedSwing": 0.5}
  },
  "tolerance": 1e-9})

base = doc("valid", [solid("b", "box", size=[1, 1, 1], weights=[{"node": "arm", "weight": 1}]), solid("l", "loft", sections=[sec(0), sec(1)], capStart=True, capEnd=True)], nodes=[node("root"), node("arm", "root", t=(0, 1, 0), joint=rev([0, 0, 1], -1, 1))],
          poses=[{"id": "rest", "title": T("Rest", "Ruhe"), "nodes": {"arm": {"rotation": qz(0.1)}}}],
          animations=[{"id": "idle", "title": T("Idle", "Leerlauf"), "durationS": 1, "loop": True, "channels": [{"target": {"node": "arm", "path": "rotation"}, "interpolation": "linear", "times": [0, 1], "values": qz(0) + qz(0.5)}], "events": []}],
          variants=[{"id": "plain", "title": T("Plain", "Schlicht"), "bindings": {"skin": "skin"}}],
          meshes=[{"id": "m", "primitives": [
              {"mode": "triangles", "material": "skin", "positions": [0, 0, 0, 1, 0, 0, 0, 1, 0], "indices": [0, 1, 2, 0, 2, 1], "solids": [{"id": "a", "first": 0, "count": 3}, {"id": "b", "first": 3, "count": 3}]},
              {"mode": "lines", "material": "skin", "positions": [0, 0, 0, 1, 0, 0, 0, 1, 0], "indices": [0, 1, 1, 2], "solids": [{"id": "a", "first": 0, "count": 2}, {"id": "b", "first": 2, "count": 2}]}]}])
ex = lambda profile, holes=None: {"id": "l", "kind": "extrude", "node": "root", "material": "skin", "profile": profile, "plane": "xy", "from": 0, "to": 1, **({"holes": holes} if holes else {})}
rv = lambda profile: {"id": "l", "kind": "revolve", "node": "root", "material": "skin", "profile": profile, "axis": [0, 1, 0], "origin": [0, 0, 0]}
S = lambda path, value: {"op": "set", "path": path, "value": value}
R = lambda path: {"op": "remove", "path": path}
V = lambda name, ops, codes, schema=True: {"name": name, "ops": ops, "expectCodes": codes, "schemaValid": schema}
write("✅️validation", {"base": base, "cases": [
  V("valid", [], []),
  V("bad-version", [S("/version", "semio.model/v2")], ["bad-version"], False),
  V("bad-slug", [S("/variants/0/id", "Plain Look")], ["bad-slug"], False),
  V("duplicate-id", [S("/solids/1/id", "b")], ["duplicate-id"]),
  V("unknown-parent", [S("/nodes/1/parent", "ghost")], ["unknown-parent"]),
  V("parent-after-child", [S("/nodes/0/parent", "arm")], ["parent-order"]),
  V("non-unit-quaternion", [S("/nodes/1/rotation", [0, 0, 0, 2])], ["non-unit-quaternion"]),
  V("zero-scale", [S("/nodes/1/scale", [1, 0, 1])], ["zero-scale"]),
  V("joint-axis-zero", [S("/nodes/1/joint/axis", [0, 0, 0])], ["joint-axis"]),
  V("joint-limits-inverted", [S("/nodes/1/joint/min", 2)], ["joint-limits"]),
  V("solid-unknown-node", [S("/solids/0/node", "ghost")], ["unknown-node"]),
  V("solid-unknown-material", [S("/solids/0/material", "ghost")], ["unknown-material"]),
  V("weights-too-many", [S("/solids/0/weights", [{"node": "arm", "weight": 1}] * 5)], ["bad-weights"], False),
  V("weights-zero-sum", [S("/solids/0/weights", [{"node": "arm", "weight": 0}])], ["bad-weights"]),
  V("box-negative-size", [S("/solids/0/size", [1, -1, 1])], ["invalid-dimension"]),
  V("box-corner-radius-too-large", [S("/solids/0/cornerRadius", 0.9)], ["invalid-dimension"]),
  V("loft-single-section", [S("/solids/1/sections", [sec(0)])], ["invalid-dimension"], False),
  V("extrude-bowtie", [S("/solids/1", ex([[0, 0], [1, 1], [1, 0], [0, 1]]))], ["self-intersection"]),
  V("extrude-closed-loop-valid", [S("/solids/1", ex([[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]))], []),
  V("extrude-spike", [S("/solids/1", ex([[0, 0], [2, 0], [1, 0], [1, 1], [0, 1]]))], ["self-intersection"]),
  V("extrude-vertex-touches-edge", [S("/solids/1", ex([[0, 0], [2, 0], [1, 1], [2, 2], [0, 2], [1, 1]]))], ["self-intersection"]),
  V("extrude-collinear-points-valid", [S("/solids/1", ex([[0, 0], [0.5, 0], [1, 0], [1, 0.5], [1, 1], [0, 1]]))], []),
  V("extrude-hole-valid", [S("/solids/1", ex(sq, [hole]))], []),
  V("extrude-hole-crossing-outline", [S("/solids/1", ex(sq, [[[0.5, 0.5], [1.5, 0.5], [1.5, 0.9], [0.5, 0.9]]]))], ["bad-holes"]),
  V("extrude-hole-outside", [S("/solids/1", ex(sq, [[[2, 2], [3, 2], [3, 3], [2, 3]]]))], ["bad-holes"]),
  V("extrude-hole-touches-outline", [S("/solids/1", ex(sq, [[[0, 0.5], [0.5, 0.3], [0.5, 0.7]]]))], ["bad-holes"]),
  V("extrude-holes-overlap", [S("/solids/1", ex(sq, [[[0.2, 0.2], [0.6, 0.2], [0.6, 0.6], [0.2, 0.6]], [[0.4, 0.4], [0.8, 0.4], [0.8, 0.8], [0.4, 0.8]]]))], ["bad-holes"]),
  V("extrude-hole-in-hole", [S("/solids/1", ex(sq, [[[0.1, 0.1], [0.9, 0.1], [0.9, 0.9], [0.1, 0.9]], [[0.4, 0.4], [0.6, 0.4], [0.6, 0.6], [0.4, 0.6]]]))], ["bad-holes"]),
  V("extrude-hole-bowtie", [S("/solids/1", ex(sq, [[[0.3, 0.3], [0.7, 0.7], [0.7, 0.3], [0.3, 0.7]]]))], ["self-intersection"]),
  V("revolve-closed-loop-valid", [S("/solids/1", rv([[0.4, 0.4], [0.6, 0.4], [0.6, 0.6], [0.4, 0.6], [0.4, 0.4]]))], []),
  V("revolve-axis-touch-valid", [S("/solids/1", rv([[0, 0], [1, 0.5], [2, 0], [3, 0.5], [4, 0]]))], []),
  V("revolve-crossing", [S("/solids/1", rv([[0, 0.1], [2, 0.5], [2, 0.2], [0, 0.6]]))], ["self-intersection"]),
  V("revolve-closed-loop-bowtie", [S("/solids/1", rv([[0.4, 0.4], [0.6, 0.6], [0.6, 0.4], [0.4, 0.6], [0.4, 0.4]]))], ["self-intersection"]),
  V("revolve-fold-back", [S("/solids/1", rv([[0, 0.3], [1, 0.3], [0.5, 0.3]]))], ["self-intersection"]),
  V("revolve-crosses-end-disc", [S("/solids/1", rv([[1, 0.5], [2, 0.5], [2, 0.2], [0.5, 0.2], [0.5, 0.3]]))], ["self-intersection"]),
  V("solids-gap", [S("/meshes/0/primitives/0/solids", [{"id": "a", "first": 0, "count": 3}, {"id": "b", "first": 4, "count": 2}])], ["bad-solids"]),
  V("solids-short", [S("/meshes/0/primitives/0/solids", [{"id": "a", "first": 0, "count": 3}])], ["bad-solids"]),
  V("solids-partial-triangle", [S("/meshes/0/primitives/0/solids", [{"id": "a", "first": 0, "count": 2}, {"id": "b", "first": 2, "count": 4}])], ["bad-solids"]),
  V("solids-partial-line", [S("/meshes/0/primitives/1/solids", [{"id": "a", "first": 0, "count": 3}, {"id": "b", "first": 3, "count": 1}])], ["bad-solids"]),
  V("solids-bad-id", [S("/meshes/0/primitives/0/solids/0/id", "Not A Slug")], ["bad-solids"], False),
  V("solids-zero-count", [S("/meshes/0/primitives/0/solids", [{"id": "a", "first": 0, "count": 0}, {"id": "b", "first": 0, "count": 6}])], ["bad-solids"], False),
  V("repeat-zero", [S("/solids/0/repeat", {"count": 0})], ["bad-repeat"], False),
  V("channel-values-mismatch", [S("/animations/0/channels/0/values", [0, 0, 0, 1])], ["bad-channel"]),
  V("channel-times-not-increasing", [S("/animations/0/channels/0/times", [1, 0])], ["bad-times"]),
  V("channel-beyond-duration", [S("/animations/0/channels/0/times", [0, 3])], ["bad-times"]),
  V("duration-zero", [S("/animations/0/durationS", 0)], ["bad-duration"], False),
  V("pose-unknown-node", [S("/poses/0/nodes", {"ghost": {}})], ["unknown-node"]),
  V("variant-unknown-material", [S("/variants/0/bindings", {"skin": "ghost"})], ["unknown-material"]),
  V("color-out-of-range", [S("/materials/0/baseColor", [2, 0, 0, 1])], ["bad-color"]),
]})

def edge_case(name, d, angle, expected=None): return {"name": name, "document": d, "angleDeg": angle, "expectedSegments": expected}
write("📏️feature-edges", {"quality": QUALITY, "cases": [
  edge_case("box-hard-edges", doc("e-box", [solid("b", "box", size=[1, 1, 1])]), 30, 12),
  edge_case("two-boxes", doc("e-boxes", [solid("b", "box", size=[1, 1, 1], repeat={"count": 2, "translation": [3, 0, 0]})]), 30, 24),
  edge_case("sphere-smooth", doc("e-sphere", [solid("e", "ellipsoid", radii=[0.5, 0.5, 0.5])]), 60, 0),
  edge_case("cylinder-caps", doc("e-cyl", [solid("r", "revolve", profile=[[0, 0.2], [1, 0.2]], axis=[0, 1, 0], origin=[0, 0, 0])]), 30, None),
  edge_case("rounded-box", doc("e-rbox", [solid("b", "box", size=[1, 1, 1], cornerRadius=0.1)]), 60, 0),
  edge_case("open-tube-boundary", doc("e-tube", [solid("l", "loft", sections=[sec(0), sec(1)], capStart=False, capEnd=False)]), 30, None),
]})

CLOTH = {**MAT, "id": "cloth", "title": T("Cloth", "Stoff")}
bx = lambda id, node="root", material="skin", t=(0, 0, 0), **kw: {**solid(id, "box", size=[1, 1, 1], transform={"translation": list(t)}, **kw), "node": node, "material": material}
prov_nodes = [node("root"), node("arm", "root", t=(0, 1, 0), joint=rev([0, 0, 1], -1, 1))]
prov_solids = [
  bx("b-one"), bx("b-many", t=(0, 3, 0), repeat={"count": 3, "translation": [3, 0, 0]}), bx("cloth-box", material="cloth", t=(0, 6, 0)),
  bx("arm-box", node="arm", t=(0, 9, 0)),
  bx("skinned-a", t=(0, 0, 3), weights=[{"node": "arm", "weight": 1}]), bx("skinned-b", t=(3, 0, 3), weights=[{"node": "arm", "weight": 1}]),
  bx("skinned-cloth", material="cloth", t=(6, 0, 3), weights=[{"node": "arm", "weight": 1}]),
]
BOX_INDICES = 36
write("🧬️provenance", {"quality": QUALITY, "angleDeg": 30, "document": doc("provenance", prov_solids, nodes=prov_nodes, materials=[MAT, CLOTH]), "expected": {
  "boxIndices": BOX_INDICES, "boxEdgeSegments": 12,
  "meshes": {
    "root": [{"material": "skin", "solids": [{"id": "b-one", "first": 0, "count": BOX_INDICES}, {"id": "b-many", "first": BOX_INDICES, "count": 3 * BOX_INDICES}]}, {"material": "cloth", "solids": [{"id": "cloth-box", "first": 0, "count": BOX_INDICES}]}],
    "arm": [{"material": "skin", "solids": [{"id": "arm-box", "first": 0, "count": BOX_INDICES}]}],
    "skinned": [{"material": "skin", "solids": [{"id": "skinned-a", "first": 0, "count": BOX_INDICES}, {"id": "skinned-b", "first": BOX_INDICES, "count": BOX_INDICES}]}, {"material": "cloth", "solids": [{"id": "skinned-cloth", "first": 0, "count": BOX_INDICES}]}],
  },
  "edges": {
    "root": [{"material": "skin", "solids": [{"id": "b-one", "first": 0, "count": 24}, {"id": "b-many", "first": 24, "count": 72}]}, {"material": "cloth", "solids": [{"id": "cloth-box", "first": 0, "count": 24}]}],
    "arm": [{"material": "skin", "solids": [{"id": "arm-box", "first": 0, "count": 24}]}],
    "skinned": [{"material": "skin", "solids": [{"id": "skinned-a", "first": 0, "count": 24}, {"id": "skinned-b", "first": 24, "count": 24}]}, {"material": "cloth", "solids": [{"id": "skinned-cloth", "first": 0, "count": 24}]}],
  }}})

def shoelace(r): return sum(r[i][0] * r[(i + 1) % len(r)][1] - r[(i + 1) % len(r)][0] * r[i][1] for i in range(len(r))) / 2
def poly_case(name, outer, holes=()):
    outer = list(outer) if shoelace(outer) > 0 else list(reversed(outer))
    holes = [list(h) if shoelace(h) < 0 else list(reversed(h)) for h in holes]
    n = len(outer) + sum(len(h) for h in holes)
    return {"name": name, "outer": outer, "holes": holes, "expected": {"area": shoelace(outer) + sum(shoelace(h) for h in holes), "triangles": n + 2 * len(holes) - 2, "vertices": n}}
e = 1e-9
write("🔺️polygon", {"areaTolerance": 1e-9, "cases": [
  poly_case("square-midpoints", [[0, 0], [0.5, 0], [1, 0], [1, 0.5], [1, 1], [0.5, 1], [0, 1], [0, 0.5]]),
  poly_case("square-near-duplicates", [[0, 0], [1, 0], [1 + e, 0], [1, 1], [0, 1], [0, 1 - e]]),
  poly_case("l-shape-collinear-runs", [[0, 0], [1, 0], [2, 0], [2, 0.5], [2, 1], [1, 1], [1, 2], [1, 3], [0, 3], [0, 2], [0, 1]]),
  poly_case("triangle-collinear-edge", [[0, 0], [1, 0], [2, 0], [3, 0], [1.5, 2]]),
  poly_case("collinear-then-reflex", [[0, 0], [2, 0], [4, 0], [4, 2], [3, 1], [2, 2], [1, 1], [0, 2], [0, 1]]),
  poly_case("hole-with-collinear-points", [[0, 0], [2, 0], [2, 2], [0, 2]], [[[0.5, 0.5], [1, 0.5], [1.5, 0.5], [1.5, 1.5], [0.5, 1.5]]]),
  poly_case("two-holes-near-duplicates", [[0, 0], [4, 0], [4, 2], [0, 2]], [[[0.5, 0.5], [1.5, 0.5], [1.5 + e, 0.5], [1.5, 1.5], [0.5, 1.5]], [[2.5, 0.5], [3.5, 0.5], [3.5, 1.5], [2.5, 1.5], [2.5, 1.0]]]),
  poly_case("two-aligned-rectangular-holes", [[-2, 0.06], [2, 0.06], [2, 1.26], [-2, 1.26]], [[[-1.965, 0.095], [-0.015, 0.095], [-0.015, 1.225], [-1.965, 1.225]], [[0.015, 0.095], [1.965, 0.095], [1.965, 1.225], [0.015, 1.225]]]),
  poly_case("three-aligned-holes-one-row", [[0, 0], [10, 0], [10, 4], [0, 4]], [[[1, 1], [3, 1], [3, 3], [1, 3]], [[4, 1], [6, 1], [6, 3], [4, 3]], [[7, 1], [9, 1], [9, 3], [7, 3]]]),
  poly_case("aligned-holes-two-rows", [[0, 0], [10, 0], [10, 10], [0, 10]], [[[1, 1], [4, 1], [4, 4], [1, 4]], [[6, 1], [9, 1], [9, 4], [6, 4]], [[1, 6], [4, 6], [4, 9], [1, 9]], [[6, 6], [9, 6], [9, 9], [6, 9]]]),
  poly_case("small-scale-collinear", [[0, 0], [0.005, 0], [0.01, 0], [0.01, 0.005], [0.01, 0.01], [0, 0.01]]),
  *[poly_case(f"float-noise-collinear-{i + 1}", ring) for i, ring in enumerate(json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "polygon-old-failures.json"))))],
]})
