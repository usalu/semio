from body import *
from decisions import *
A = Asset("kneeling-gardener")
SK = {"eye", "knee-patch-l", "clog-strap-l", "trug", "trug-weeds", "trug-rim", "trowel-blade", "head", "hat-brim", "hat-crown", "pad", "shirt"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
A.keep("pad")
hd = loft_from(A.draft["head"], rxfn=pfn([(-0.01, 0.05), (0.05, 0.052), (0.08, 0.06), (0.12, 0.071), (0.151, 0.072), (0.2, 0.065), (0.256, 0.03)]), amax=1.6, exp=2.0)
A.add(hd, like="head")
A.add(loft_from(A.draft["shirt"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.4, exp=2.5), like="shirt")
A.add(loft("pelvis", "pelvis", [sec(-0.07, 0.15, 0.1, e=2.4), sec(0.0, 0.17, 0.12, e=2.4), sec(0.094, 0.165, 0.115, e=2.4)], material="canvas", role="body", tone=0.5, region="trousers"), like="pelvis")
A.add(loft_from(A.draft["hat-crown"], lat=(-0.104, 0.104), exp=2.0), like="hat-crown")
A.add(revolve("hat-brim", "head", [[0.181, 0.0], [0.181, 0.225], [0.1855, 0.235], [0.2235, 0.235], [0.228, 0.225], [0.228, 0.0]], axis=(1, 0, 0), material="straw", role="body", tone=0.9, region="hat"), like="hat-brim")
band(A, "hat-band", "hat-crown", 0.228, 0.242, 0.003, "straw", 0.3, "hat", like="hat-band", maxden=100)
eyes(A, u=0.17, w=0.03, rx=0.009, ry=0.007, rz=0.009, material="skin", tone=0.1, region="face")
for sd, z in (("l", 1), ("r", -1)):
    vs = surface_v(hd, 0.191, z * 0.03) + 0.002
    A.add(capsule(f"brow-{sd}", "head", [0.191, vs, z * 0.012], [0.19, vs + 0.003, z * 0.05], 0.004, material="skin", role="detail", tone=0.2, region="face"), like="brow", maxden=50)
    A.add(ellipsoid(f"ear-{sd}", "head", [0.024, 0.012, 0.009], [0.147, 0.014, z * 0.08], material="skin", role="body", tone=0.86, region="face"), like="ear")
A.add(ellipsoid("nose", "head", [0.015, 0.009, 0.008], [0.152, surface_v(hd, 0.152, 0.0) + 0.003, 0.0], material="skin", role="detail", tone=0.8, region="face"), like="nose", maxden=50)
vm = surface_v(hd, 0.1, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.1, vm, -0.017], [0.1, vm, 0.017], 0.003, material="skin", role="detail", tone=0.5, region="face"), like="mouth", maxden=50)
deltoids(A, ["upper-arm-l", "upper-arm-r"])
mirror(A, "thigh-l", "thigh-r", "thigh-r")
mirror(A, "shank-l", "shank-r", "shank-r")
mirror(A, "clog-l", "clog-r", "foot-r")
for sd, z, sh in (("l", 1, "shank-l"), ("r", -1, "shank-r")):
    A.add(ellipsoid(f"knee-patch-{sd}", sh, [0.034, 0.04, 0.012], [0.05, 0.0, z * 0.056], material="canvas", role="detail", tone=0.35, region="trousers"), like="knee-patch", maxden=100)
    band(A, f"clog-strap-{sd}", f"clog-{sd}", -0.025, -0.005, 0.003, "rubber-clog", 0.2, "clogs", like="clog-strap", maxden=100)
A.add(revolve("trug", "trug", [[0.0, 0.0], [0.0, 0.19], [0.21, 0.25], [0.21, 0.235], [0.02, 0.175], [0.02, 0.0]], axis=(0, 1, 0), material="polyethylene", role="body", tone=0.72, region="trug"), like="trug")
A.add(ellipsoid("trug-weeds", "trug", [0.2, 0.16, 0.19], [0.0, 0.152, 0.0], material="polyethylene", role="body", tone=0.45, region="trug"), like="trug-weeds")
for i, (u, w0, w1) in enumerate(((-0.12, -0.091, -0.07), (0.05, -0.026, -0.02), (-0.03, 0.04, 0.052), (0.1, 0.09, 0.117))):
    A.add(capsule(f"weed-stalk-{i}", "trug", [u, 0.21, w0], [u, 0.3, w1], 0.004, material="polyethylene", role="detail", tone=0.2, region="trug"), like="weed-stalk", maxden=50)
A.add(loft_from(A.draft["trowel-blade"], lat=(-0.034, 0.034), exp=2.2), like="trowel-blade")
DEC = BODY + [
    d("The far (right) leg is drawn only in the front and plan views, hidden in the side view; it is authored as the mirror of the near leg's lofts (thigh, shank, clog, knee patch, strap) on its own bones, so each pose moves it independently.",
      "Das ferne (rechte) Bein ist nur in Vorder- und Draufsicht gezeichnet und in der Seitenansicht verdeckt; es ist als Spiegelbild der Loft-Körper des nahen Beins (Oberschenkel, Unterschenkel, Clog, Kniepolster, Riemen) auf eigenen Knochen modelliert, sodass jede Pose es unabhängig bewegt."),
    d("The straw hat is a crown loft plus a flat brim revolve about the head axis (radius 0.235 m, 0.047 m thick), the trug a hollow revolve bucket with a floor and a weed heap ellipsoid with four stalks, the pad a rounded box, the trowel a lofted scoop; knee patches are flat ellipsoids on the outer knee.",
      "Der Strohhut besteht aus einem Kronen-Loft und einer flachen Krempe als Rotationskörper um die Kopfachse (Radius 0,235 m, 0,047 m dick), der Trog ist ein hohler Rotationseimer mit Boden samt Unkrautbeet-Ellipsoid mit vier Halmen, das Kissen ein abgerundeter Quader, die Schaufel ein gelofteter Löffel; die Kniepolster sind flache Ellipsoide am äusseren Knie."),
]
A.write(DEC)
