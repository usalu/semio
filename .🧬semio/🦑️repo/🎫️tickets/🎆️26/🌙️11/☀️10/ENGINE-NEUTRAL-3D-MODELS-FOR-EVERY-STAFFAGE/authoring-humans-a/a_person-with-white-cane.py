from body import *
from decisions import *
A = Asset("person-with-white-cane")
names = list(A.draft)
FOLD = {k for k in names if any(t in k for t in ("fold", "crease", "nostril", "jaw-line", "lower-lip", "lace", "sole-", "sole"))}
CANE = {k for k, s in A.draft.items() if s["bone"] == "cane"}
SK = FOLD | CANE | {"eye", "ear", "mouth", "cuff-l-l", "cuff-l-l-2", "cuff-r", "cuff-s-l", "cuff-s-r", "thumb-l", "thumb-r", "finger-l0", "finger-r0", "torso", "hip", "front", "sunglasses", "sunglasses-arm"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
tj = loft_from(A.draft["torso"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.6, exp=2.5)
A.add(tj, like="torso")
A.add(loft("hip", "pelvis", [sec(-0.094, 0.15, 0.11, e=2.4), sec(0.0, 0.163, 0.12, e=2.4), sec(0.14, 0.15, 0.11, e=2.4)], material="denim-jacket", role="body", tone=0.4, region="jacket"), like="hip")
hd = get(A, "head")
for sd in "lr":
    g = get(A, f"hand-{sd}")
    wc = g["sections"][len(g["sections"]) // 2]["at"][2]
    A.add(capsule(f"thumb-{sd}", f"hand-{sd}", [0.02, 0.025, wc], [0.1, 0.035, wc], 0.012, 0.01, material="skin-medium", role="body", tone=0.7, region="hands"), like="thumb")
    refinger(A, [f"finger-{sd}0"], f"hand-{sd}")
    band(A, f"cuff-s-{sd}", f"sleeve-fore-{sd}", 0.196, 0.206, 0.003, "denim-jacket", 0.5, "jacket", like="cuff-s")
    band(A, f"cuff-{sd}", f"shank-g-{sd}", 0.297, 0.307, 0.003, "denim-jeans", 0.4, "jeans", like="cuff")
eyes(A, u=0.085, w=0.03, rx=0.008, ry=0.006, rz=0.008, material="skin-medium", tone=0.1, region="face-skin")
vm = surface_v(hd, 0.035, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.035, vm, -0.017], [0.035, vm, 0.017], 0.003, material="skin-medium", role="detail", tone=0.5, region="face-skin"), like="mouth", maxden=50)
for sd, z in (("l", 1), ("r", -1)):
    A.add(ellipsoid(f"ear-{sd}", "head", [0.011, 0.024, 0.011], [0.083, 0.009, z * 0.076], material="skin-medium", role="detail", tone=0.78, region="face-skin"), like="ear")
    A.add(ellipsoid(f"lens-{sd}", "head", [0.015, 0.004, 0.028], [0.109, surface_v(hd, 0.109, z * 0.036) + 0.0005, z * 0.036], material="sunglass-polymer", role="body", tone=0.05, region="sunglasses"), like="sunglasses")
    A.add(capsule(f"sunglasses-arm-{sd}", "head", [0.114, surface_v(hd, 0.114, z * 0.064), z * 0.064], [0.114, 0.007, z * 0.072], 0.004, material="sunglass-polymer", role="detail", tone=0.1, region="sunglasses"), like="sunglasses-arm", maxden=50)
A.add(capsule("bridge", "head", [0.109, surface_v(hd, 0.109, 0.0), -0.01], [0.109, surface_v(hd, 0.109, 0.0), 0.01], 0.004, material="sunglass-polymer", role="detail", tone=0.1, region="sunglasses"), like="bridge", maxden=50)
A.add(capsule("front", "torso", [0.372, surface_v(tj, 0.372, 0.0) + 0.002, 0.0], [0.02, surface_v(tj, 0.02, 0.0) + 0.002, 0.0], 0.004, material="denim-jacket", role="detail", tone=0.15, region="jacket"), like="front", maxden=50)
deltoids(A, ["sleeve-up-l", "sleeve-up-r"])
C = "cane"
A.add(capsule("cane-shaft", C, [0.094, 0.0, 0.0], [1.226, 0.0, 0.0], 0.0095, material="cane-alu-white", role="body", tone=0.95, region="cane-shaft"), like="cane-shaft")
A.add(capsule("cane-band", C, [0.964, 0.0, 0.0], [1.226, 0.0, 0.0], 0.0102, material="cane-alu-white", role="body", tone=0.35, region="cane-shaft"), like="cane-band", maxden=100)
A.add(capsule("cane-grip", C, [-0.098, 0.0, 0.0], [0.099, 0.0, 0.0], 0.0165, material="grip-eva", role="body", tone=0.12, region="cane-grip"), like="cane-grip")
tube_path(A, "cane-strap", C, [(-0.1, 0.0), (-0.14, 0.028), (-0.2, 0.012), (-0.2, -0.012), (-0.14, -0.028), (-0.1, 0.0)], 0.004, "grip-eva", 0.3, "cane-grip", role="detail", like="cane-strap", maxden=100)
A.add(capsule("cane-tip-holder", C, [1.212, 0.0, 0.0], [1.275, 0.0, 0.0], 0.009, material="nylon-roller", role="body", tone=0.6, region="cane-tip"), like="cane-tip-holder")
A.add(revolve("cane-roller", C, [[-0.02, 0.0], [-0.02, 0.017], [0.02, 0.017], [0.02, 0.0]], axis=(0, 0, 1), material="nylon-roller", role="body", tone=0.8, region="cane-tip", transform={"translation": [1.283, 0.0, 0.0]}), like="cane-roller")
DEC = BODY + [
    d("The white cane is a round aluminium shaft with a thicker top band, an EVA grip capsule with a wrist-strap loop of tube segments, a tip holder and a rolling tip as a revolve about the axle (the draft's flat extrusions of 0.04 m width are replaced by the revolve hints of the front and plan views); it sits at 0.15 m right of the body axis on the hand bone.",
      "Der weisse Langstock ist ein runder Aluminiumschaft mit dickerem oberem Band, einer EVA-Griffkapsel mit einer Handschlaufe aus Rohrsegmenten, einem Spitzenhalter und einer Rollspitze als Rotationskörper um die Achse (die flachen 0,04 m breiten Extrusionen des Entwurfs sind durch die Rotationsvorgaben von Vorder- und Draufsicht ersetzt); er sitzt am Handknochen 0,15 m rechts der Körperachse."),
    d("The jacket and hips use the front outline widths (0.169 m and 0.163 m; the draft's hip and front-line lateral guesses came from the bag twin and are replaced); the shoulder bag is a loft on the left hip with a strap loft across the torso, the sunglasses two lens ellipsoids with a bridge and temple arms, hems and cuffs thin rings.",
      "Jacke und Hüfte nutzen die Breiten der Vorderansicht (0,169 m und 0,163 m; die Lateralschätzungen des Entwurfs für Hüfte und Frontlinie stammten vom Taschen-Zwilling und sind ersetzt); die Umhängetasche ist ein Loft an der linken Hüfte mit einem Gurt-Loft über den Rumpf, die Sonnenbrille zwei Linsen-Ellipsoide mit Steg und Bügeln, Säume und Manschetten dünne Ringe."),
]
A.write(DEC)
