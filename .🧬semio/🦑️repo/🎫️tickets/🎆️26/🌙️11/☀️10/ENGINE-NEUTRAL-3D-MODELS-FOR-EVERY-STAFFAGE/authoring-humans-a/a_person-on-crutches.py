from body import *
from decisions import *
A = Asset("person-on-crutches")
names = list(A.draft)
FOLD = {k for k in names if any(t in k for t in ("fold", "crease", "nostril", "jaw-line", "lower-lip", "lace", "sole-", "crutch-hole", "crutch-adjust", "crutch-cuff-gap", "boot-strap", "cuff-s-"))}
SK = FOLD | {k for k in names if k.startswith("crutch-") or k.startswith("cuff-")} | {"ear", "mouth", "thumb-l", "thumb-r", "finger-l0", "finger-l2", "finger-r0", "finger-r1", "finger-r2", "torso", "eye"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
A.add(loft_from(A.draft["torso"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.6, exp=2.5), like="torso")
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
finger_caps(A, ["finger-l0", "finger-l2"], "hand-l")
finger_caps(A, ["finger-r0", "finger-r1", "finger-r2"], "hand-r")
hd = get(A, "head")
eyes(A, u=0.095, w=0.03, rx=0.008, ry=0.006, rz=0.008, material="skin-light", tone=0.1, region="face-skin")
vm = surface_v(hd, 0.045, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.045, vm, -0.017], [0.045, vm, 0.017], 0.003, material="skin-light", role="detail", tone=0.5, region="face-skin"), like="mouth", maxden=50)
for sd, z in (("l", 1), ("r", -1)):
    A.add(ellipsoid(f"ear-{sd}", "head", [0.026, 0.012, 0.011], [0.09, 0.009, z * 0.082], material="skin-light", role="detail", tone=0.78, region="face-skin"), like="ear")
    for i, u in enumerate((0.246, 0.304, 0.363)):
        if sd == "l":
            band(A, f"boot-strap-{i}", "boot-shaft-l", u - 0.004, u + 0.004, 0.003, "boot-shell", 0.6, "boot", like="boot-strap", maxden=50)
tj = get(A, "torso")
A.add(loft("hood", "torso", [sec(0.38, 0.08, 0.05, e=2.2, at=[0.0, 0.09, 0.0]), sec(0.45, 0.1, 0.08, e=2.2, at=[0.0, 0.09, 0.0]), sec(0.5, 0.09, 0.06, e=2.2, at=[0.0, 0.09, 0.0])], material="fleece-hoodie", role="body", tone=0.34, region="hoodie"), like="hood")
deltoids(A, ["sleeve-up-l", "sleeve-up-r"])
for sd, z in (("l", 1), ("r", -1)):
    cb = -0.0 + 0.0
    A.add(capsule(f"crutch-tube-{sd}", f"crutch-{sd}", [0.0, 0.005 if sd == "r" else -0.008, z * 0.015], [0.83, 0.005 if sd == "r" else -0.008, z * 0.103], 0.0105, material="alu-anodised", role="body", tone=0.6, region="crutch-frame"), like="crutch-tube")
    A.add(capsule(f"crutch-tip-{sd}", f"crutch-{sd}", [0.83, 0.0, z * 0.1035], [0.9, 0.0, z * 0.1065], 0.02, material="rubber-black", role="body", tone=0.1, region="crutch-tips"), like="crutch-tip")
    A.add(capsule(f"crutch-grip-{sd}", f"crutch-{sd}", [0.0, -0.048, z * 0.015], [0.0, 0.059, z * 0.015], 0.016, ls=1.55, material="rubber-black", role="body", tone=0.12, region="crutch-grips"), like="crutch-grip")
    band(A, f"crutch-cuff-{sd}", f"sleeve-fore-{sd}", 0.03, 0.12, 0.02, "cuff-polymer", 0.8, "crutch-cuffs", role="body", like="crutch-cuff", maxden=100)
    A.add(capsule(f"crutch-bracket-{sd}", f"forearm-{sd}", [0.08, -0.05, 0.0], [0.28, -0.05, 0.0], 0.02, material="alu-anodised", role="body", tone=0.62, region="crutch-frame"), like="crutch-bracket")
DEC = BODY + [
    d("Each forearm crutch is built from the plan and front slant of its tube: a round aluminium tube leaning outward from the hand (0.015 m) to the tip (0.105 m), a wider rubber tip capsule, a horizontal grip capsule along the walking direction, a polymer cuff ring around the forearm and a bracket tube from the cuff to the grip, so the crutch follows the hand bone while the cuff follows the forearm.",
      "Jede Unterarmstütze ist aus der Schräge ihres Rohres in Grund- und Vorderansicht aufgebaut: ein rundes Aluminiumrohr, das von der Hand (0,015 m) nach aussen zur Spitze (0,105 m) geneigt ist, eine breitere Gummispitze als Kapsel, ein waagerechter Griff als Kapsel in Gehrichtung, ein Kunststoff-Manschettenring um den Unterarm und ein Bügelrohr von der Manschette zum Griff, sodass die Stütze dem Handknochen und die Manschette dem Unterarm folgt."),
    d("The left leg wears a walking boot (shaft loft with strap rings), the right a sneaker; the hood is a loft behind the neck; finger outlines are capsules on the glove centre line; laces, soles, adjustment holes and cuff gaps are drawing details and are not modelled.",
      "Das linke Bein trägt einen Gehstiefel (Schaft-Loft mit Riemenringen), das rechte einen Turnschuh; die Kapuze ist ein Loft hinter dem Hals; Fingerumrisse sind Kapseln auf der Handmittellinie; Schnürsenkel, Sohlen, Verstelllöcher und Manschettenspalten sind Zeichendetails und nicht modelliert."),
]
A.write(DEC)
