from body import *
from decisions import *
A = Asset("parent-with-stroller")
names = [k for k in A.draft]
FOLD = {k for k in names if any(t in k for t in ("fold", "crease", "nostril", "jaw-line", "lower-lip", "jacket-hem", "jeans-seam", "sole", "trouser-hem", "elbow", "shoulder-1", "shoulder-2", "eye", "brow"))}
SK = FOLD | {"head", "torso", "cuff-l", "cuff-r", "fingers-l-0", "fingers-l-1", "fingers-l-2", "fingers-r-0", "fingers-r-1", "fingers-r-2", "thumb-l", "thumb-r", "mouth-line", "forearm-l-elbow-1"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
hd = loft_from(A.draft["head"], rxfn=pfn([(0.0, 0.05), (0.045, 0.052), (0.09, 0.068), (0.156, 0.075), (0.27, 0.03)]), amax=1.6, exp=2.0)
A.add(hd, like="head")
A.add(loft_from(A.draft["torso"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.4, exp=2.5), like="torso")
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
    refinger(A, [f"fingers-{sd}-{i}" for i in range(3)], f"hand-{sd}")
    band(A, f"cuff-{sd}", f"forearm-{sd}", 0.236, 0.252, 0.003, "cotton-clothing", 0.3, "arm-clothing", like="cuff")
eyes(A, u=0.177, w=0.03, rx=0.006, ry=0.006, rz=0.008, material="skin", tone=0.1, region="head-skin")
for sd, z in (("l", 1), ("r", -1)):
    vs = surface_v(hd, 0.195, z * 0.03) + 0.002
    A.add(capsule(f"brow-{sd}", "head", [0.195, vs, z * 0.012], [0.194, vs + 0.003, z * 0.05], 0.005, material="skin", role="detail", tone=0.2, region="head-skin"), like="brow", maxden=50)
    A.add(ellipsoid(f"ear-{sd}", "head", [0.024, 0.012, 0.011], [0.157, 0.019, z * 0.079], material="skin", role="detail", tone=0.7, region="head-skin"), like="ear")
vm = surface_v(hd, 0.11, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.11, vm, -0.017], [0.11, vm, 0.017], 0.003, material="skin", role="detail", tone=0.3, region="head-skin"), like="mouth-line", maxden=50)
deltoids(A, ["upper-arm-l", "upper-arm-r"])
S = "stroller"
FR = dict(material="aluminium", role="body", tone=0.8, region="stroller-frame")
A.add(capsule("handle-grip", S, [0.0, 0.0, -0.25], [0.0, 0.0, 0.25], 0.017, material="rubber", role="body", tone=0.25, region="stroller-frame"), like="handle-grip")
for sd, z in (("l", 1), ("r", -1)):
    A.add(capsule(f"push-bar-{sd}", S, [0.03, -0.01, z * 0.24], [0.22, -0.58, z * 0.24], 0.012, **FR), like="push-bar")
    A.add(capsule(f"chassis-{sd}", S, [0.21, -0.575, z * 0.24], [0.79, -0.62, z * 0.24], 0.0125, **FR), like="chassis-rail")
    A.add(capsule(f"rear-strut-{sd}", S, [0.24, -0.58, z * 0.24], [0.28, -0.841, z * 0.28], 0.011, **FR), like="rear-strut")
    A.add(capsule(f"front-fork-{sd}", S, [0.77, -0.62, z * 0.2], [0.8, -0.92, z * 0.2], 0.01, **FR), like="front-fork")
A.add(box("footrest", S, [0.148, 0.012, 0.24], [0.67, -0.645, 0.0], corner=0.004, rot=quat_z(-19.65), **FR), like="footrest")
seat = {**A.draft["seat-fabric-1"], "id": "seat-fabric", "from": -0.11, "to": 0.11}
A.add(seat, like="seat-fabric")
box_of(A, "basket", corner=0.01)
get(A, "basket")["transform"]["translation"][2] -= 0.1925
for sd, z in (("l", 1), ("r", -1)):
    h = {**A.draft["hood"], "id": f"hood-{sd}", "from": 0.0 if z > 0 else -0.26, "to": 0.26 if z > 0 else 0.0, "taperTo": [1.0, 0.5] if z > 0 else [0.5, 1.0]}
    A.add(h, like="hood")
A.add(loft("child-body", "child", [sec(-0.04, 0.06, 0.05, "v", at=[0.0, 0, 0]), sec(0.02, 0.105, 0.075, "v", at=[0.0, 0, 0]), sec(0.14, 0.11, 0.072, "v", at=[-0.01, 0, 0]), sec(0.26, 0.09, 0.065, "v", at=[-0.02, 0, 0]), sec(0.31, 0.05, 0.05, "v", at=[-0.02, 0, 0])], material="cotton-clothing", role="body", tone=0.62, region="child-clothing"), like="child-body")
A.add(loft_from(A.draft["child-legs"], lat=(-0.085, 0.085), exp=2.2), like="child-legs")
A.add(ellipsoid("child-head", "child", [0.075, 0.08, 0.08], [-0.01, 0.34, 0.0], material="skin", role="body", tone=0.88, region="child-skin"), like="child-head")
A.add(ellipsoid("child-hat", "child", [0.085, 0.045, 0.08], [-0.01, 0.385, 0.0], material="cotton-clothing", role="body", tone=0.3, region="child-clothing"), like="child-hat")
for sd, z in (("l", 1), ("r", -1)):
    A.add(ellipsoid(f"child-eye-{sd}", "child", [0.006, 0.006, 0.006], [0.07, 0.345, z * 0.03], material="skin", role="detail", tone=0.1, region="child-skin"), like="child-eye", maxden=50)
    A.add(capsule(f"child-arm-{sd}", "child", [0.0, 0.2, z * 0.1], [0.17, 0.08, z * 0.1], 0.022, 0.018, material="cotton-clothing", role="body", tone=0.6, region="child-clothing"), like="child-arm")
for w, bone, R, hw, wc, rr in (("rear", "wheel-rear", 0.159, 0.02, 0.28, 0.115), ("front", "wheel-front", 0.08, 0.02, 0.2, 0.052)):
    t = revolve(f"tyre-{w}", bone, [[-hw, 0.0], [-hw, R - 0.012], [-hw + 0.006, R - 0.004], [-hw + 0.012, R], [hw - 0.012, R], [hw - 0.006, R - 0.004], [hw, R - 0.012], [hw, 0.0]], axis=(0, 0, 1), material="rubber", role="body", tone=0.12, region="tyres")
    t["transform"] = {"translation": [0, 0, -wc]}
    t["repeat"] = {"count": 2, "translation": [0, 0, 2 * wc]}
    A.add(t, like=f"tyre-{w}")
    r = revolve(f"rim-{w}", bone, [[-hw - 0.001, 0.0], [-hw - 0.001, rr], [hw + 0.001, rr], [hw + 0.001, 0.0]], axis=(0, 0, 1), material="rubber", role="detail", tone=0.75, region="tyres")
    r["transform"] = {"translation": [0, 0, -wc]}
    r["repeat"] = {"count": 2, "translation": [0, 0, 2 * wc]}
    A.add(r, like=f"rim-{w}")
DET = {**FR, "role": "detail"}
for sd, z in (("l", 1), ("r", -1)):
    for i, u in enumerate((0.36, 0.6)):
        A.add(capsule(f"seat-hanger-{sd}{i}", S, [u, -0.59, z * 0.24], [u, -0.59, z * 0.105], 0.006, **DET), like="seat-hanger", maxden=50)
        A.add(capsule(f"basket-strap-{sd}{i}", S, [0.31 + i * 0.34, -0.6, z * 0.24], [0.31 + i * 0.34, -0.7, z * 0.2], 0.005, **DET), like="basket-strap", maxden=50)
    A.add(capsule(f"footrest-arm-{sd}", S, [0.73, -0.665, z * 0.12], [0.77, -0.64, z * 0.2], 0.006, **DET), like="footrest", maxden=50)
    A.add(capsule(f"seat-back-hanger-{sd}", S, [0.12, -0.3, z * 0.24], [0.2, -0.3, z * 0.105], 0.006, **DET), like="seat-hanger", maxden=50)
for s_ in A.out:
    if s_["bone"] in ("stroller", "stroller-hood", "wheel-rear", "wheel-front", "child"):
        shift_w(s_, 0.1925)
DEC = BODY + [
    d("The stroller is a rigid assembly on its own bones, built from the lateral hints of the front and plan views (rails at 0.24 m, fork at 0.20 m, rear wheels at 0.28 m): round tubes for push bars, chassis rails, struts and fork legs, a footrest plank, an extruded seat sling, a box basket with straps, a canopy of two tapered extrusions (a dome, not a slab), and solid wheel revolves with rim discs repeated across the axle.",
      "Der Kinderwagen ist eine starre Baugruppe auf eigenen Knochen, aufgebaut aus den seitlichen Vorgaben von Vorder- und Draufsicht (Holme bei 0,24 m, Gabel bei 0,20 m, Hinterräder bei 0,28 m): runde Rohre für Schiebebügel, Chassisholme, Streben und Gabelholme, ein Fussbrett, eine extrudierte Sitzbespannung, ein Quaderkorb, ein Verdeck aus zwei verjüngten Extrusionen (eine Kuppel, keine Platte) und volle Radrotationskörper mit Felgenscheiben, über die Achse wiederholt."),
    d("The seated child is a loft along the child bone's normal for the body, an extruded-outline loft for the legs, ellipsoids for head and hat, and two arm tubes; the draft's lateral guesses (z 0.193) are replaced by the front-view widths (body 0.22 m, head 0.16 m).",
      "Das sitzende Kind ist ein Loft entlang der Normalen des Kindknochens für den Körper, ein Loft aus dem Umriss für die Beine, Ellipsoide für Kopf und Mütze und zwei Armröhren; die Lateralschätzungen des Entwurfs (z 0,193) sind durch die Breiten der Vorderansicht ersetzt (Körper 0,22 m, Kopf 0,16 m)."),
    d("The seat sling is 0.22 m wide (child width) because the 2D plan draws no seat; clothing creases, folds, nostrils, jaw and lip lines and the stroller hood rib and fabric seams are drawing aids and are not modelled.",
      "Die Sitzbespannung ist 0,22 m breit (Kinderbreite), weil die 2D-Draufsicht keinen Sitz zeichnet; Kleiderfalten, Knitter, Nasenlöcher, Kiefer- und Lippenlinien sowie die Verdeckstreben und Stoffnähte des Kinderwagens sind Zeichenhilfen und nicht modelliert."),
]
A.write(DEC)
