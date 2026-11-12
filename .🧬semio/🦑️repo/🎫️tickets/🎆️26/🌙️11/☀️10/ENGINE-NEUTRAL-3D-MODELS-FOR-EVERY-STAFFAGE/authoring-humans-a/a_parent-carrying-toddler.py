from body import *
from decisions import *
A = Asset("parent-carrying-toddler")
SK = {"eye", "head", "scarf", "coat"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
hd = loft_from(A.draft["head"], rxfn=pfn([(0.0, 0.048), (0.04, 0.052), (0.08, 0.068), (0.15, 0.074), (0.26, 0.03)]), amax=1.6, exp=2.0)
A.add(hd, like="head")
A.add(loft_from(A.draft["coat"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.4, exp=2.5), like="coat")
A.add(loft_from(A.draft["scarf"], lat=(-0.1, 0.1), exp=2.2), like="scarf")
A.add(capsule("neck", "torso", [0.42, 0.0, 0.0], [0.5, 0.0, 0.0], 0.05, material="skin", role="body", tone=0.86, region="face"), like="neck")
eyes(A, u=0.174, w=0.03, rx=0.006, ry=0.007, rz=0.006, material="skin", tone=0.1, region="face")
A.add(ellipsoid("nose", "head", [0.015, 0.009, 0.008], [0.13, surface_v(hd, 0.13, 0.0) + 0.003, 0.0], material="skin", role="detail", tone=0.8, region="face"), like="nose", maxden=50)
vm = surface_v(hd, 0.075, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.075, vm, -0.016], [0.075, vm, 0.016], 0.003, material="skin", role="detail", tone=0.5, region="face"), like="mouth", maxden=50)
for sd in "lr":
    A.add(capsule(f"glove-{sd}", f"hand-{sd}", [0.0, 0.0, 0.0], [0.17, 0.0, 0.0], 0.03, 0.026, ls=0.75, material="leather-glove", role="body", tone=0.5, region="gloves"), like="hand")
    A.add(capsule(f"thumb-{sd}", f"hand-{sd}", [0.03, 0.028, 0.0], [0.1, 0.03, 0.0], 0.012, 0.009, material="leather-glove", role="body", tone=0.5, region="gloves"), like="thumb")
deltoids(A, ["upper-arm-l", "upper-arm-r"])
T = lambda sid: A.draft[sid]
A.add(loft_from(T("toddler-suit"), exp=2.4), like="toddler-suit")
A.add(loft_from(T("toddler-head"), exp=2.0), like="toddler-head")
A.add(loft_from(T("toddler-hat"), exp=2.0), like="toddler-hat")
for n in ("thigh", "shank"):
    A.add(loft_from(T(f"toddler-{n}-n"), exp=2.2), like=f"toddler-{n}-n")
A.add(loft_from(T("toddler-boot-n"), exp=2.4), like="toddler-boot-n")
A.add(loft_from(T("toddler-arm-n"), exp=2.2), like="toddler-arm-n")
shift_copy(A, "toddler-thigh-n", "toddler-thigh-f", "toddler-thigh-f", 0.1)
shift_copy(A, "toddler-shank-n", "toddler-shank-f", "toddler-shank-f", 0.1)
shift_copy(A, "toddler-boot-n", "toddler-boot-f", "toddler-shank-f", 0.1)
A.add(loft_from(T("toddler-arm-n"), lat=(0.083, 0.157), aspect=1.0, ramp=0, exp=2.2, id="toddler-arm-f", bone="toddler-arm-f"), like="toddler-arm-n")
for sd, bone, w in (("n", "toddler-arm-n", -0.035), ("f", "toddler-arm-f", 0.12)):
    A.add(ellipsoid(f"toddler-mitten-{sd}", bone, [0.03, 0.03, 0.03], [0.215, 0.0, w], material="knit-hat", role="body", tone=0.35, region="toddler-mittens"), like="toddler-mitten")
DEC = BODY + [
    d("The toddler is a second small body on its own bones (torso, head, hat, near and far arm, near and far thigh, shank and boot); the far limbs are hidden in the side view and are authored from the plan and front views as copies of the near outlines shifted by 0.10 m (legs) and re-centred at 0.12 m (arm), with mittens as ellipsoids.",
      "Das Kleinkind ist ein zweiter kleiner Körper auf eigenen Knochen (Rumpf, Kopf, Mütze, naher und ferner Arm, naher und ferner Oberschenkel, Unterschenkel und Stiefel); die fernen Glieder sind in der Seitenansicht verdeckt und aus Draufsicht und Vorderansicht als um 0,10 m verschobene (Beine) beziehungsweise bei 0,12 m neu zentrierte (Arm) Kopien der nahen Umrisse modelliert, die Fäustlinge sind Ellipsoide."),
    d("The parent's hands exist only in the front and plan views (the elevation forearm outline already runs to the fingertips): gloves are tapering flat capsules with a thumb tube, the neck is a capsule between coat and head, and the scarf is a round loft narrower than the coat.",
      "Die Hände des Elternteils existieren nur in Vorder- und Draufsicht (der Unterarmumriss der Seitenansicht reicht schon bis zu den Fingerspitzen): die Handschuhe sind verjüngte flache Kapseln mit einer Daumenröhre, der Hals ist eine Kapsel zwischen Mantel und Kopf, der Schal ein runder Loft schmaler als der Mantel."),
]
A.write(DEC)
