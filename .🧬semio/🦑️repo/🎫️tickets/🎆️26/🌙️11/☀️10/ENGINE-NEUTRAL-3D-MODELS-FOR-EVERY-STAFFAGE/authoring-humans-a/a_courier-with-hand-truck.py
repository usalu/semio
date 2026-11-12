from body import *
from decisions import *
A = Asset("courier-with-hand-truck")
SK = {"cargo-pocket-l-1", "cargo-pocket-l-2", "cargo-pocket-l-3", "cargo-pocket-r-1", "cargo-pocket-r-2", "cargo-pocket-r-3", "knee-patch-l-1", "knee-patch-l-2", "knee-patch-l-3",
      "knee-patch-r-1", "knee-patch-r-2", "knee-patch-r-3", "trouser-hem-l", "trouser-hem-r", "jacket-zip", "chest-pocket-1", "chest-pocket-2", "chest-pocket-3", "eye", "glove-thumb-l", "glove-thumb-r",
      "reflective-band-0", "reflective-band-1"}
humanize(A, skip=SK | {"cap"}, front={"jacket": ("torso",)})
A.add(loft_from(A.draft["cap"], lat=(-0.09, 0.09), exp=2.0), like="cap")
for sd in "lr":
    thumb(A, f"glove-thumb-{sd}", f"hand-{sd}")
tw = front_halfwidth(A, "torso", "torso")
for i in (0, 1):
    b = loft_from(A.draft[f"reflective-band-{i}"], rxfn=lambda u: tw(u) + 0.003, amax=6, exp=2.5)
    for s in b["sections"]:
        s["ry"] += 0.003
    A.add(b, like=f"reflective-band-{i}")
A.add(loft("pelvis", "pelvis", [sec(-0.085, 0.17, 0.105, e=2.4), sec(0.0, 0.18, 0.12, e=2.4), sec(0.095, 0.165, 0.11, e=2.4)], material="cotton-workwear", role="body", tone=0.5, region="trousers"), like="pelvis")
jk = get(A, "jacket")
eyes(A, u=0.085, w=0.03, rx=0.009, ry=0.006, rz=0.009, material="skin", tone=0.9)
for sd, z in (("l", 1), ("r", -1)):
    for k in (0, 1):
        pass
    patch(A, f"cargo-pocket-{sd}", f"thigh-{sd}", 0.2, 0.044, z * 0.088, [0.12, 0.088, 0.014], "cargo-pocket-l-1", "cotton-workwear", 0.1, "trousers")
    patch(A, f"knee-patch-{sd}", f"thigh-{sd}", 0.425, 0.003, z * 0.07, [0.07, 0.115, 0.012], "knee-patch-l-1", "cotton-workwear", 0.4, "trousers")
    patch(A, f"chest-pocket-{sd}", "torso", 0.32, surface_v(jk, 0.32, z * 0.105), z * 0.105, [0.08, 0.012, 0.09], "chest-pocket-1", "hivis-softshell", 0.1, "jacket") if sd == "l" else None
p0 = surface_v(jk, -0.15, 0)
p1 = surface_v(jk, 0.42, 0)
A.add(capsule("jacket-zip", "torso", [-0.15, p0 + 0.002, 0], [0.42, p1 + 0.002, 0], 0.004, material="hivis-softshell", role="detail", tone=0.1, region="jacket"), like="jacket-zip", maxden=50)
for pid in ("parcel-a", "parcel-b", "parcel-c"):
    box_of(A, pid)
for tid, pid in (("tape-a", "parcel-a"), ("tape-b", "parcel-b"), ("tape-c", "parcel-c")):
    p = get(A, pid)
    c = p["transform"]["translation"]
    sz = p["size"]
    A.add(box(tid, "parcels", [sz[0], 0.004, 0.05], [c[0], c[1] - sz[1] / 2 + 0.0015, 0], material="cardboard", role="detail", tone=0.2, region="parcels"), like=tid, maxden=50)
rail = capsule("rail", "hand-truck", [-0.08, 0.0, -0.21], [1.0, 0.0, -0.21], 0.016, material="steel-tube", role="body", tone=0.35, region="hand-truck")
rail["repeat"] = {"count": 2, "translation": [0, 0, 0.42]}
A.add(rail, like="rail")
A.keep("plate")
for i, u in enumerate((0.45, 0.8)):
    A.add(capsule(f"brace-{i}", "hand-truck", [u, 0.0, -0.21], [u, 0.0, 0.21], 0.01, material="steel-tube", role="detail", tone=0.1, region="hand-truck"), like="brace", maxden=100)
for sd, z in (("l", 0.19), ("r", -0.19)):
    A.add(capsule(f"stay-{sd}-1", "hand-truck", [0.02, 0.0, z], [0.02, -0.12, z], 0.006, material="steel-tube", role="detail", tone=0.1, region="hand-truck"), like="brace", maxden=50)
    A.add(capsule(f"stay-{sd}-2", "hand-truck", [0.02, -0.12, z], [0.06, -0.29, z], 0.006, material="steel-tube", role="detail", tone=0.1, region="hand-truck"), like="brace", maxden=50)
A.add(capsule("handle", "hand-truck", [1.045, 0.095, -0.21], [1.045, 0.095, 0.21], 0.02, material="steel-tube", role="body", tone=0.1, region="hand-truck"), like="handle")
for sd, z in (("l", 0.21), ("r", -0.21)):
    A.add(capsule(f"handle-arm-{sd}", "hand-truck", [0.985, 0.012, z], [1.04, 0.12, z], 0.011, material="steel-tube", role="body", tone=0.1, region="hand-truck"), like="handle")
A.add(capsule("axle-stay", "hand-truck", [0.115, 0.0, -0.275], [0.115, 0.0, 0.275], 0.015, material="steel-tube", role="detail", tone=0.1, region="hand-truck"), like="axle-stay", maxden=100)
tyre = revolve("tyre", "truck-lift", [[-0.025, 0.0], [-0.025, 0.1], [-0.02, 0.123], [-0.01, 0.1295], [0.0, 0.13], [0.01, 0.1295], [0.02, 0.123], [0.025, 0.1], [0.025, 0.0]], axis=(0, 0, 1), material="tyre-rubber", role="body", tone=0.85, region="tyres")
tyre["transform"] = {"translation": [0, 0, -0.275]}
tyre["repeat"] = {"count": 2, "translation": [0, 0, 0.55]}
A.add(tyre, like="tyre")
hub = revolve("hub", "truck-lift", [[-0.03, 0.0], [-0.03, 0.055], [0.03, 0.055], [0.03, 0.0]], axis=(0, 0, 1), material="tyre-rubber", role="detail", tone=0.5, region="tyres")
hub["transform"] = {"translation": [0, 0, -0.275]}
hub["repeat"] = {"count": 2, "translation": [0, 0, 0.55]}
A.add(hub, like="hub")
A.add(capsule("axle", "truck-lift", [0, 0, -0.285], [0, 0, 0.285], 0.012, material="steel-tube", role="detail", tone=0.1, region="hand-truck"), like="hub", maxden=100)
DEC = BODY + [
    d("The hand truck is a rigid assembly on its own bones: two round rails (one capsule repeated), two cross braces, a toe plate with two stay wires per side, a handle bar with two loop arms, and two pneumatic wheels as revolves about the axle (tyre and hub, repeated across the axle) on the lift bone; parcels are three boxes whose tape strips lie on their front faces.",
      "Die Sackkarre ist eine starre Baugruppe auf eigenen Knochen: zwei runde Holme (eine wiederholte Kapsel), zwei Querstreben, eine Schaufel mit je zwei Stützdrähten, ein Griffbügel mit zwei Bogenarmen und zwei Luftreifen als Rotationskörper um die Achse (Reifen und Nabe, über die Achse wiederholt) am Hubknochen; die Pakete sind drei Quader, deren Klebestreifen auf der Vorderseite liegen."),
    d("The jacket is wider than the plan trunk circle: its lateral half width follows the front outline of the jacket (0.165 to 0.20 m), which agrees with the plan shoulder ellipse; the front-only reflective bands are slightly larger than the jacket and the pelvis is a trouser loft under the jacket hem.",
      "Die Jacke ist breiter als der Rumpfkreis der Draufsicht: ihre seitliche Halbbreite folgt dem Vorderansichtsumriss der Jacke (0,165 bis 0,20 m) und stimmt mit der Schulterellipse der Draufsicht überein; die nur in der Vorderansicht gezeichneten Reflexstreifen sind etwas grösser als die Jacke, das Becken ist ein Hosen-Loft unter dem Jackensaum."),
    d("Pockets and knee patches drawn as stitched outlines are flat boxes on the lateral thigh and chest surfaces; zip and hems are thin tubes or omitted; the elevation shapes brace-1 and brace-2 (toe stays) were merged with the front cross braces by id in the draft and are authored separately here.",
      "Als Nähte gezeichnete Taschen und Kniepatches sind flache Quader auf der seitlichen Oberschenkel- und Brustfläche; Reissverschluss und Säume sind dünne Röhren oder entfallen; die Seitenansichtsformen brace-1 und brace-2 (Schaufelstützen) wurden im Entwurf mit den Quersstreben der Vorderansicht über die Id vermischt und sind hier getrennt modelliert."),
]
A.write(DEC)
