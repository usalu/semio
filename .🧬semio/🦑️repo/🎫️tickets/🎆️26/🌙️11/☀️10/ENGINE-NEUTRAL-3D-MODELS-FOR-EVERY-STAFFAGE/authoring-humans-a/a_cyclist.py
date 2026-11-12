from body import *
from decisions import *
A = Asset("cyclist")
SK = {"hair-tail-1", "hair-tail-2", "hair-tail-3", "helmet-vent-1", "helmet-strap-1", "helmet-strap-2", "eye", "jacket-reflective", "glove-thumb-l", "glove-thumb-r", "fingers-l-0", "fingers-l-1", "fingers-l-2",
      "fingers-r-0", "fingers-r-1", "fingers-r-2", "jacket", "helmet"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
A.add(loft_from(A.draft["jacket"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.6, exp=2.5), like="jacket")
A.add(loft_from(A.draft["helmet"], lat=(-0.1, 0.1), exp=2.0), like="helmet")
for sd in "lr":
    thumb(A, f"glove-thumb-{sd}", f"hand-{sd}", glove=f"glove-{sd}")
    g = get(A, f"glove-{sd}")
    wc = g["sections"][len(g["sections"]) // 2]["at"][2]
    for i, v in enumerate((-0.015, 0.0, 0.016)):
        s = A.draft[f"fingers-{sd}-{i}"]
        A.add(capsule(f"fingers-{sd}-{i}", f"hand-{sd}", [0.072, v, wc], [0.132, v, wc], 0.006, material=s["material"], role="detail", tone=s["tone"], region="gloves"), like="fingers", maxden=50)
tail = loft_from(A.draft["hair-tail-2"], lat=(-0.03, 0.03), exp=2.0, id="hair-tail")
A.add(tail, like="hair-tail")
eyes(A, u=0.061, w=0.03, rx=0.009, ry=0.006, rz=0.009, material="skin", tone=0.2)
jk = get(A, "jacket")
band(A, "jacket-reflective", "jacket", 0.147, 0.171, 0.003, "softshell", 0.9, "jacket", like="jacket-reflective")
vent = capsule("helmet-vent", "head", [0.165, 0.062, -0.03], [0.184, -0.035, -0.03], 0.005, material="eps-helmet", role="detail", tone=0.2, region="helmet")
vent["repeat"] = {"count": 3, "translation": [0, 0, 0.03]}
A.add(vent, like="helmet-vent", maxden=50)
for i, (p0, p1) in enumerate((([0.059, 0.042], [0.001, 0.013]), ([0.001, 0.013], [-0.023, -0.025]))):
    st = capsule(f"helmet-strap-{i + 1}", "head", [p0[0], p0[1], -0.05], [p1[0], p1[1], -0.05], 0.005, material="eps-helmet", role="detail", tone=0.2, region="helmet")
    st["repeat"] = {"count": 2, "translation": [0, 0, 0.1]}
    A.add(st, like="helmet-strap", maxden=50)
deltoids(A, ["sleeve-l", "sleeve-r"])
# bike
BM = dict(material="steel", role="body", tone=0.2, region="bike-metal")
def tube(id, p0, p1, r, w0=0.0, w1=None, like=None, **kw):
    A.add(capsule(id, "frame", [p0[0], p0[1], w0], [p1[0], p1[1], w0 if w1 is None else w1], r, **{**BM, **kw}), like=like or id)
tube("tube-seat", (0.45, 0.3), (0.275, 0.874), 0.016)
tube("tube-top", (0.275, 0.874), (0.955, 0.86), 0.016)
tube("tube-down", (0.45, 0.3), (0.99, 0.76), 0.018)
tube("tube-head", (0.99, 0.76), (0.955, 0.875), 0.022)
for sd, z in (("l", 1), ("r", -1)):
    tube(f"tube-chainstay-{sd}", (0.45, 0.3), (0.0, 0.34), 0.0095, z * 0.035, z * 0.065, like="tube-chainstay")
    tube(f"tube-seatstay-{sd}", (0.275, 0.874), (0.0, 0.34), 0.0095, z * 0.02, z * 0.065, like="tube-seatstay")
    tube(f"fork-{sd}", (0.99, 0.76), (1.05, 0.34), 0.011, z * 0.045, z * 0.05, like="fork")
tube("stem", (0.955, 0.875), (0.915, 1.0), 0.014)
tube("seatpost", (0.275, 0.874), (0.255, 0.925), 0.011)
for sd, z in (("l", 1), ("r", -1)):
    tube(f"handlebar-{sd}", (0.925, 1.0), (0.8, 1.05), 0.014, z * 0.03, z * 0.307, like="handlebar")
    tube(f"grip-{sd}", (0.80, 1.05), (0.718, 1.058), 0.018, z * 0.307, like="grip")
A.add(capsule("handlebar-cross", "frame", [0.925, 1.0, -0.03], [0.925, 1.0, 0.03], 0.012, **BM), like="handlebar")
sad = loft_from(A.draft["saddle"], lat=(-0.078, 0.078), exp=2.6, rxfn=front_halfwidth(A, "frame", "saddle", "plan"), amax=3.0)
sad["bone"] = "frame"
A.add(sad, like="saddle")
A.add(ellipsoid("headlamp", "frame", [0.02, 0.02, 0.02], [1.01, 0.91, 0.0], **{**BM, "role": "detail", "tone": 0.7}), like="headlamp", maxden=100)
tube("headlamp-bracket", (0.958, 0.878), (1.005, 0.905), 0.006, role="detail", like="headlamp")
A.add({**A.draft["front-fender"], "profile": A.draft["front-fender"]["profile"]}, like="front-fender")
A.add({**A.draft["rear-fender-2"], "id": "rear-fender"}, like="rear-fender")
A.add(capsule("chain-top", "frame", [0.45, 0.381, -0.05], [0.0, 0.381, -0.05], 0.005, **{**BM, "role": "detail", "tone": 0.2}), like="chain", maxden=50)
A.add(capsule("chain-bottom", "frame", [0.45, 0.219, -0.05], [0.0, 0.299, -0.05], 0.005, **{**BM, "role": "detail", "tone": 0.2}), like="chain", maxden=50)
A.add(capsule("bottom-bracket", "frame", [0.45, 0.3, -0.045], [0.45, 0.3, 0.045], 0.02, **BM), like="tube-seat", maxden=100)
for w, bone, x in (("rear", "rear-wheel", 0.0), ("front", "front-wheel", 0.0)):
    ring_wheel(A, f"{w}-tyre", bone, 0.331, 0.009, 2.67, 36, "rubber", 0.12, "tyres", like=f"{w}-tyre")
    ring_wheel(A, f"{w}-rim", bone, 0.317, 0.005, 2.0, 36, "steel", 0.75, "bike-metal", role="detail", like=f"{w}-rim")
    spokes(A, f"{w}-spoke", bone, 0.025, 0.314, 16, 0.001, "steel", 0.75, "bike-metal")
    A.add(revolve(f"{w}-hub", bone, [[-0.035, 0.0], [-0.035, 0.018], [-0.03, 0.025], [0.03, 0.025], [0.035, 0.018], [0.035, 0.0]], axis=(0, 0, 1), material="steel", role="body", tone=0.6, region="bike-metal"), like=f"{w}-hub")
    A.add(capsule(f"axle-{w}", bone, [0, 0, -0.07], [0, 0, 0.07], 0.006, material="steel", role="body", tone=0.75, region="bike-metal"), like=f"axle-{w}")
A.add(revolve("rear-cog", "rear-wheel", [[-0.005, 0.0], [-0.005, 0.041], [0.005, 0.041], [0.005, 0.0]], axis=(0, 0, 1), material="steel", role="detail", tone=0.5, region="bike-metal", transform={"translation": [0, 0, -0.04]}), like="rear-cog")
A.add(revolve("chainring", "crank-r", [[-0.0025, 0.0], [-0.0025, 0.081], [0.0025, 0.081], [0.0025, 0.0]], axis=(0, 0, 1), material="steel", role="body", tone=0.55, region="bike-metal", transform={"translation": [0, 0, -0.053]}), like="chainring")
A.add(capsule("chainring-mark", "crank-r", [0.03, 0.0, -0.057], [0.075, 0.0, -0.057], 0.005, **{**BM, "role": "detail", "tone": 0.2}), like="chainring-mark", maxden=50)
A.keep("crank-arm-r")
A.keep("crank-arm-l")
for sd, z in (("r", -1), ("l", 1)):
    A.add(box(f"pedal-{sd}", f"pedal-{sd}", [0.104, 0.016, 0.095], [0, 0, z * 0.1425], corner=0.004, **{**BM, "tone": 0.15}), like="pedal")
    A.add(capsule(f"pedal-spindle-{sd}", f"pedal-{sd}", [0, 0, z * 0.088], [0, 0, z * 0.1], 0.006, **{**BM, "role": "detail"}), like="pedal", maxden=100)
DEC = BODY + [
    d("The bicycle is authored from the elevation tube end points and the plan lateral positions: the diamond frame is round tubes on the centre plane, chain stays, seat stays and fork legs are pairs at their plan offsets, the handlebar sweeps back to grips at 0.307 m, the saddle is a loft whose width follows the plan outline.",
      "Das Fahrrad ist aus den Rohrendpunkten der Seitenansicht und den seitlichen Lagen der Draufsicht aufgebaut: der Diamantrahmen besteht aus runden Rohren in der Mittelebene, Kettenstreben, Sitzstreben und Gabelholme sind Paare mit ihren Abständen aus der Draufsicht, der Lenker schwingt zurück zu den Griffen bei 0,307 m, der Sattel ist ein Loft, dessen Breite dem Umriss der Draufsicht folgt."),
    d("Tyres and rims are rings made of 36 chord capsules repeated about the axle (a revolve cannot make a hollow torus under the volume rules), with a 20-spoke wheel, a flanged hub revolve, an axle, a rear cog and a chainring disc on the crank; the draft's duplicated down tubes, rear-fender triples and lateral-scale hacks are replaced.",
      "Reifen und Felgen sind Ringe aus 36 Sehnenkapseln, um die Achse wiederholt (ein Rotationskörper kann unter den Volumenregeln keinen hohlen Torus bilden), dazu ein Rad mit 20 Speichen, eine Nabe als Rotationskörper mit Flanschen, eine Achse, ein hinteres Ritzel und eine Kettenblattscheibe an der Kurbel; die verdoppelten Unterrohre, die dreiteiligen Schutzbleche und die Lateralskalen-Behelfe des Entwurfs sind ersetzt."),
    d("The rider's legs keep the lateral offsets of the authored front and plan hints (knees splayed outward, shanks about 0.055 m outside the thigh axis), because the 2D views draw them so; the three hair strands drawn in the elevation are one ponytail loft (the plan draws one capsule).",
      "Die Beine des Fahrers behalten die seitlichen Versätze der gezeichneten Vorder- und Draufsicht (Knie nach aussen, Unterschenkel etwa 0,055 m ausserhalb der Oberschenkelachse), weil die 2D-Ansichten sie so zeichnen; die drei in der Seitenansicht gezeichneten Haarsträhnen sind ein Zopf-Loft (die Draufsicht zeichnet eine Kapsel)."),
]
A.write(DEC)
