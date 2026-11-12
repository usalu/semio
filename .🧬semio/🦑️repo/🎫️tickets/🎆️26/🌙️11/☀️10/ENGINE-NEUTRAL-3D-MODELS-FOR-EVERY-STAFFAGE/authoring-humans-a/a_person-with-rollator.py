from body import *
from decisions import *
A = Asset("person-with-rollator")
names = list(A.draft)
FOLD = {k for k in names if any(t in k for t in ("fold", "crease", "nostril", "jaw-line", "lower-lip", "lace", "sole-", "cuff-s-", "spoke", "hat-nub", "glasses", "hub"))}
FRAMEISH = {k for k, s in A.draft.items() if s["bone"] in ("frame", "wheel-rear", "wheel-front")}
SK = FOLD | FRAMEISH | {"ear", "mouth", "thumb-l", "thumb-r", "finger-l0", "finger-l1", "finger-l2", "finger-r0", "finger-r1", "finger-r2", "button-0", "eye", "cuff-l", "cuff-r", "front"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
    finger_caps(A, [f"finger-{sd}{i}" for i in range(3)], f"hand-{sd}")
    band(A, f"cuff-{sd}", f"shank-g-{sd}", 0.345, 0.36, 0.003, "wool-trouser", 0.4, "trousers", like="cuff")
A.keep("front") if False else None
hd = get(A, "head")
tj = get(A, "torso")
eyes(A, u=0.095, w=0.03, rx=0.008, ry=0.006, rz=0.008, material="skin-aged", tone=0.1, region="face-skin")
vm = surface_v(hd, 0.034, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.034, vm, -0.017], [0.034, vm, 0.017], 0.003, material="skin-aged", role="detail", tone=0.5, region="face-skin"), like="mouth", maxden=50)
A.add(ellipsoid("button-0", "torso", [0.009, 0.007, 0.009], [0.115, surface_v(tj, 0.115, 0.0) + 0.001, 0.0], material="loden-wool", role="detail", tone=0.15, region="coat"), like="button-0", maxden=50)
A.add(capsule("front", "torso", [0.372, surface_v(tj, 0.372, 0.0) + 0.002, 0.0], [0.02, surface_v(tj, 0.02, 0.0) + 0.002, 0.0], 0.004, material="loden-wool", role="detail", tone=0.5, region="coat"), like="front", maxden=50)
A.add(ellipsoid("hat-nub", "head", [0.022, 0.022, 0.022], [0.22, -0.004, 0.0], material="wool-beret", role="detail", tone=0.2, region="beret"), like="hat-nub", maxden=50)
for sd, z in (("l", 1), ("r", -1)):
    A.add(ellipsoid(f"ear-{sd}", "head", [0.01, 0.023, 0.01], [0.078, 0.008, z * 0.073], material="skin-aged", role="detail", tone=0.78, region="face-skin"), like="ear")
    A.add(ellipsoid(f"glasses-lens-{sd}", "head", [0.015, 0.003, 0.02], [0.097, surface_v(hd, 0.097, z * 0.03) + 0.0005, z * 0.03], material="skin-aged", role="detail", tone=0.1, region="face-skin"), like="glasses", maxden=50)
    A.add(capsule(f"glasses-arm-{sd}", "head", [0.103, surface_v(hd, 0.103, z * 0.052), z * 0.052], [0.103, 0.006, z * 0.066], 0.003, material="skin-aged", role="detail", tone=0.1, region="face-skin"), like="glasses-arm", maxden=50)
A.add(capsule("glasses-bridge", "head", [0.097, surface_v(hd, 0.097, 0.0), -0.01], [0.097, surface_v(hd, 0.097, 0.0), 0.01], 0.003, material="skin-aged", role="detail", tone=0.1, region="face-skin"), like="glasses", maxden=50)
deltoids(A, ["sleeve-up-l", "sleeve-up-r"])
F = "frame"
FR = dict(material="alu-red-paint", role="body", tone=0.78, region="frame")
for sd, z in (("l", 1), ("r", -1)):
    A.add(capsule(f"rear-upright-{sd}", F, [0.0205, 0.12, z * 0.27], [0.0, 0.84, z * 0.27], 0.0125, **FR), like="rear-upright")
    A.add(capsule(f"front-leg-{sd}", F, [0.462, 0.505, z * 0.27], [0.5, 0.25, z * 0.27], 0.0125, **FR), like="front-leg")
    A.add(capsule(f"lower-rail-{sd}", F, [0.015, 0.26, z * 0.27], [0.5, 0.24, z * 0.27], 0.013, **FR), like="lower-rail")
    A.add(capsule(f"seat-rail-{sd}", F, [0.01, 0.495, z * 0.19], [0.48, 0.495, z * 0.19], 0.0175, **FR), like="seat-rail")
    A.add(capsule(f"front-brace-{sd}", F, [0.3, 0.3, z * 0.17], [0.479, 0.5, z * 0.17], 0.01, **FR), like="front-brace")
    A.add(capsule(f"grip-{sd}", F, [-0.14, 0.848, z * 0.29], [0.0, 0.848, z * 0.29], 0.018, material="rubber-grip", role="body", tone=0.15, region="grips"), like="grip")
    A.add(capsule(f"brake-lever-{sd}", F, [0.07, 0.84, z * 0.24], [-0.02, 0.8, z * 0.24], 0.0065, **{**FR, "tone": 0.6}), like="brake-lever")
    A.add(capsule(f"brake-cable-{sd}", F, [0.06, 0.8, z * 0.26], [0.035, 0.15, z * 0.26], 0.003, **{**FR, "role": "detail", "tone": 0.5}), like="brake-cable", maxden=50)
    A.add(capsule(f"caster-fork-{sd}", F, [0.5, 0.3, z * 0.27], [0.52, 0.1, z * 0.27], 0.01, **FR), like="caster-fork")
A.add(capsule("handlebar", F, [-0.005, 0.8525, -0.29], [-0.005, 0.8525, 0.29], 0.0125, **{**FR, "tone": 0.4}), like="handlebar")
A.add(capsule("front-bar", F, [0.48, 0.5, -0.27], [0.48, 0.5, 0.27], 0.0125, **{**FR, "tone": 0.4}), like="front-bar")
box_of(A, "seat", corner=0.008)
box_of(A, "backrest", corner=0.008)
for sd, z in (("l", 1), ("r", -1)):
    A.add(capsule(f"backrest-arm-{sd}", F, [0.012, 0.655, z * 0.2], [0.01, 0.655, z * 0.27], 0.008, **{**FR, "role": "detail"}), like="backrest", maxden=100)
A.add({**A.draft["basket"], "from": -0.19, "to": 0.19}, like="basket")
A.add({**A.draft["bag-2"], "id": "bag", "from": -0.12, "to": 0.12}, like="bag-2")
tube_path(A, "bag-handle", F, [(0.2, 0.575), (0.26, 0.675), (0.32, 0.575)], 0.006, "cotton-tote", 0.5, "bag", role="detail", like="bag-handle", maxden=100)
for w, bone, R, wc, rr in (("rear", "wheel-rear", 0.1, 0.29, 0.066), ("front", "wheel-front", 0.1, 0.29, 0.066)):
    t = revolve(f"tyre-{w}", bone, [[-0.02, 0.0], [-0.02, R - 0.01], [-0.015, R - 0.003], [-0.008, R], [0.008, R], [0.015, R - 0.003], [0.02, R - 0.01], [0.02, 0.0]], axis=(0, 0, 1), material="pu-tyre", role="body", tone=0.12, region="tyres")
    t["transform"] = {"translation": [0, 0, -wc]}
    t["repeat"] = {"count": 2, "translation": [0, 0, 2 * wc]}
    A.add(t, like=f"tyre-{w}")
    r = revolve(f"rim-{w}", bone, [[-0.021, 0.0], [-0.021, rr], [0.021, rr], [0.021, 0.0]], axis=(0, 0, 1), material="alu-red-paint", role="body", tone=0.8, region="frame")
    r["transform"] = {"translation": [0, 0, -wc]}
    r["repeat"] = {"count": 2, "translation": [0, 0, 2 * wc]}
    A.add(r, like=f"rim-{w}")
    A.add(capsule(f"axle-{w}", bone, [0.0, 0.0, -0.3], [0.0, 0.0, 0.3], 0.012, material="alu-red-paint", role="detail", tone=0.3, region="frame"), like=f"hub-{w}", maxden=100)
DEC = BODY + [
    d("The rollator is a rigid frame on its own bone built from the front, plan and elevation views: rear uprights, front legs, lower rails and seat rails as paired round tubes at 0.27 m and 0.19 m, a handlebar and a front cross bar, grips and brake levers at 0.29 m and 0.24 m, a seat and backrest as rounded boxes, an extruded basket and tote bag with a tube strap; four wheels as solid tyre and rim revolves repeated across a thin axle, with caster forks on the front wheel bone.",
      "Der Rollator ist ein starrer Rahmen auf eigenem Knochen, aufgebaut aus Vorder-, Grund- und Seitenansicht: hintere Pfosten, Vorderbeine, untere Holme und Sitzholme als Rohrpaare bei 0,27 m und 0,19 m, ein Lenkbügel und eine vordere Querstange, Griffe und Bremshebel bei 0,29 m und 0,24 m, Sitz und Rückenlehne als abgerundete Quader, ein extrudierter Korb und eine Tragetasche mit Rohrgriff; vier Räder als volle Reifen- und Felgen-Rotationskörper, über eine dünne Achse wiederholt, mit Lenkgabeln am Vorderradknochen."),
    d("The draft's single central front leg, single lower rail and three-way splits of the bag and uprights come from lateral guesses and are replaced by the paired tubes of the front and plan views; wheel spokes inside the solid wheels, laces, soles, cloth folds and the glasses frame outline are drawing details: the glasses are two lens ellipsoids with a bridge and temple arms.",
      "Das einzelne mittlere Vorderbein, die einzelne untere Schiene und die Dreifachaufteilungen von Tasche und Pfosten im Entwurf stammen aus Lateralschätzungen und sind durch die Rohrpaare der Vorder- und Draufsicht ersetzt; Radspeichen in den vollen Rädern, Schnürsenkel, Sohlen, Stofffalten und die Brillenkontur sind Zeichendetails: die Brille besteht aus zwei Linsen-Ellipsoiden mit Steg und Bügeln."),
]
A.write(DEC)
