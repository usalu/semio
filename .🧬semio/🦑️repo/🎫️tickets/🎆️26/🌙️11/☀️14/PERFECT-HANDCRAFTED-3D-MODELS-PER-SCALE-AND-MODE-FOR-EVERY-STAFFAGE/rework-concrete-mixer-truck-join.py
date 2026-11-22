"""🔩 Author pass 1 (second round) on machinery/concrete-mixer-truck: the coarser scales get the carriers that hold each part in reality (rear frame sill, hopper gussets, pedestal feet, spring seats and hangers, bogie hangers and trunnion, tank and box brackets, fender stays, chute cradles, exhaust outlet, intake elbow, mirror arms, ladder foot, platform stay), each sized for the smallest feature of its scale, so body and structure are one object with a closed load path.

Usage: python3 rework-concrete-mixer-truck-join.py   (source only; publish with volumes-insert.py)
"""
import json

PATH = "volumes/machinery/concrete-mixer-truck.json"
doc = json.load(open(PATH))
V = doc["volumes"]
P, C = "paint-steel", "cast-iron"
SIDES = (("l", 1), ("r", -1))
AXLE_X = {1: 3.025, 2: 1.23, 3: -1.275, 4: -2.675}


def get(scale, sid, lst="body"):
    return next((s for s in V[str(scale)][lst] if s["id"] == sid), None)


def region_of(scale, sid):
    return next(t["region"] for t in V[str(scale)]["thermal"] if t.get("of") == sid)


def put(scale, solid, like, member=None, part="member"):
    vol = V[str(scale)]
    assert get(scale, solid["id"]) is None, (scale, solid["id"])
    vol["body"].append(solid)
    vol["thermal"].append({"id": "th-" + solid["id"], "of": solid["id"], "region": like if like in REGIONS else region_of(scale, like)})
    if member:
        vol["structure"].append({"id": "st-" + solid["id"], "of": solid["id"], "member": member, "part": part})


REGIONS = {"chassis", "running-gear"}


def box(sid, size, at, bone="chassis", tone=0.3, material=P):
    return {"id": sid, "kind": "box", "bone": bone, "material": material, "role": "detail", "tone": tone, "size": size, "transform": {"translation": at}}


def rod(sid, p0, p1, r, tone=0.3, material=P):
    return {"id": sid, "kind": "capsule", "bone": "chassis", "material": material, "role": "detail", "tone": tone, "p0": p0, "p1": p1, "r0": r, "r1": r}


def turned(sid, origin, axis, length, r, tone=0.3, material=P):
    return {"id": sid, "kind": "revolve", "bone": "chassis", "material": material, "role": "detail", "tone": tone, "profile": [[0, r], [length, r]], "axis": axis, "origin": origin}


for scale in (100, 200, 500):
    put(scale, box("rear-frame-sill", [0.22, 0.08, 1.16], [-3.51, 1.12, 0]), "rear-frame-l", "chassis")

for scale, size, at in ((100, [0.06, 0.05, 0.26], [-3.98, 3.38, 0.42]), (200, [0.08, 0.06, 0.3], [-3.95, 3.37, 0.4]), (500, [0.1, 0.08, 0.3], [-3.94, 3.36, 0.4])):
    for s, k in SIDES:
        put(scale, box(f"hopper-bracket-{s}", size, [at[0], at[1], k * at[2]]), "rear-frame-l", "chassis")
put(500, box("swivel-beam", [0.16, 0.08, 1.16], [-3.68, 1.99, 0]), "rear-frame-l", "chassis")

for scale in (100, 200):
    for s, k in SIDES:
        put(scale, box(f"rear-pedestal-foot-{s}", [1.56, 0.06, 0.42], [-2.45, 1.19, k * 0.5]), "rear-pedestal-l", "chassis")
        put(scale, box(f"bogie-hanger-{s}", [0.2, 0.2, 0.09], [-1.975, 0.76, k * 0.34]), "chassis", "chassis")
        for n in (3, 4):
            put(scale, box(f"spring-seat-{n}-{s}", [0.14, 0.05, 0.12], [0, 0.105, k * 0.5], bone=f"axle-{n}", material=C), "running-gear", f"axle-{n}")
        for i, x in ((1, -1.275), (2, -2.675)):
            put(scale, box(f"fender-rear-stay-{s}-{i}", [0.05, 0.04, 0.2], [x, 1.12, k * 0.47]), "running-gear")
        put(scale, rod(f"fender-2-stay-{s}", [1.23, 1.0, k * 0.36], [1.23, 1.12, k * 0.87], 0.02), "running-gear")
        for i, x in ((1, -1.7), (2, -2.35)):
            put(scale, box(f"spare-chute-cradle-{s}-{i}", [0.05, 0.05, 0.3], [x, 1.15, k * 0.97]), "running-gear")
    put(scale, rod("exhaust-outlet", [2.22, 0.93, -0.95], [2.37, 1.04, -0.95], 0.055, tone=0.2, material=get(scale, "exhaust-stack")["material"]), "exhaust-stack")
    put(scale, turned("air-intake-elbow", [2.28, 1.18, 0.9], [0.77104, 0.59969, 0.21418], 0.23345, 0.075, material=get(scale, "air-intake")["material"]), "air-intake")
put(100, box("battery-bracket", [0.3, 0.06, 0.13], [0.5, 0.88, 0.44]), "chassis")
put(200, box("battery-bracket", [0.3, 0.06, 0.13], [0.5, 0.88, 0.44]), "chassis")
put(200, box("silencer-bracket", [0.3, 0.06, 0.17], [2.09, 0.9, -0.455]), "chassis")
put(200, box("air-filter-bracket", [0.3, 0.06, 0.22], [2.09, 0.95, 0.48]), "chassis")
put(200, box("fuel-tank-bracket-1", [0.06, 0.06, 0.16], [0.1, 0.87, 0.44]), "chassis")
put(200, box("fuel-tank-bracket-2", [0.06, 0.06, 0.16], [-0.42, 0.87, 0.44]), "chassis")
put(200, box("air-tank-bracket", [0.08, 0.06, 0.26], [-0.09, 0.9, -0.5]), "chassis")
put(200, turned("bogie-trunnion", [-1.975, 0.72, -0.58], [0, 0, 1], 1.16, 0.055), "running-gear", "chassis")
put(100, box("oil-cooler-bracket", [0.2, 0.05, 0.06], [1.98, 1.5, 0.465]), "oil-cooler")
for s, k in SIDES:
    put(100, box(f"sun-visor-bracket-{s}", [0.07, 0.02, 0.04], [4.385, 2.9, k * 0.8]), "sun-visor")
    get(100, f"mirror-arm-{s}")["p0"] = [4.36, 2.84, k * 1.02]
    put(200, box(f"mirror-arm-{s}", [0.05, 0.05, 0.16], [4.44, 2.74, k * 1.07]), f"mirror-{s}")
    get(100, f"mud-flap-{s}")["transform"]["translation"][0] = -3.295
    get(100, f"support-roller-{s}")["profile"] = [[0, 0.12], [0.14, 0.12]]
    roller = json.loads(json.dumps(get(100, f"support-roller-{s}")))
    put(200, roller, "rear-pedestal-l")
    for n in (1, 2):
        put(200, box(f"spring-seat-{n}-{s}", [0.14, 0.06, 0.1], [0, 0.07, k * 0.34], bone=f"axle-{n}", material=C), "running-gear", f"axle-{n}")
        for i, dx in ((1, -0.73), (2, 0.73)):
            put(200, box(f"spring-hanger-{n}-{s}-{i}", [0.1, 0.14, 0.09], [round(AXLE_X[n] + dx, 3), 0.78, k * 0.34]), "chassis", "chassis")
for joint in V["200"]["structure"]:
    if joint["id"].startswith("joint-drum-roller") and joint.get("kind") == "revolve":
        joint["profile"] = [[a, round(r + 0.01, 3)] for a, r in joint["profile"]]

get(100, "tail-lamp-beam").update(size=[0.1, 0.1, 2.36], transform={"translation": [-4.38, 0.8, 0]})
get(200, "tail-lamp-beam").update(size=[0.14, 0.14, 2.36], transform={"translation": [-4.4, 0.8, 0]})
put(200, box("ladder-foot", [0.3, 0.04, 0.46], [-4.32, 0.8, 1.01]), "ladder-stile-1")
put(200, rod("platform-stay", [-3.7, 2.5, 0.56], [-3.96, 2.5, 0.8], 0.02), "platform")
put(500, box("tail-lamp-beam", [0.16, 0.1, 2.36], [-4.27, 0.8, 0]), "underrun-bar")
put(500, box("platform-stay", [0.4, 0.08, 0.08], [-3.84, 2.5, 0.66]), "platform")
put(500, box("air-filter", [0.6, 0.62, 0.72], [2.18, 0.98, 0.72], tone=0.35), "air-intake")
get(500, "silencer").update(size=[0.6, 0.76, 0.76], transform={"translation": [2.18, 0.82, -0.75]})

TEXT = {
    100: ("Every part hangs on its carrier: the rear frame stands on a sill across the subframe, the hopper on two gussets, the roller pedestals on foot plates, the bogie on hangers under the rails with spring seats on both drive axles; tank, battery, cooler and visor brackets, fender stays, chute cradles, the exhaust outlet and the intake elbow close the remaining gaps.",
          "Jedes Teil hängt an seinem Träger: Der Heckrahmen steht auf einer Schwelle quer über dem Hilfsrahmen, der Trichter auf zwei Knotenblechen, die Laufrollenböcke auf Fußplatten, das Doppelachsaggregat an Böcken unter den Längsträgern mit Federsitzen auf beiden Antriebsachsen; Konsolen für Tank, Batterie, Kühler und Sonnenblende, Kotflügelstreben, Schurrenauflagen, Auspuffkrümmer und Ansaugbogen schließen die übrigen Lücken."),
    200: ("The carriers stay as far as they are above 30 mm: sill, hopper gussets, pedestal feet, bogie trunnion with hangers, spring seats and hangers of all four axles, tank and box brackets, fender stays, chute cradles, mirror arms, ladder foot and platform stay, so the body is one object and the load runs from the drum to the tyres.",
          "Die Träger bleiben, soweit sie über 30 mm liegen: Schwelle, Trichter-Knotenbleche, Bockfüße, Pendelachse mit Böcken, Federsitze und Federböcke aller vier Achsen, Tank- und Kastenkonsolen, Kotflügelstreben, Schurrenauflagen, Spiegelarme, Leiterfuß und Podeststrebe, damit der Körper ein Gegenstand ist und die Last von der Trommel bis zu den Reifen läuft."),
    500: ("At 75 mm the carriers are merged into few masses: the rear frame on its sill with one beam under the chute swivel, the hopper on two gusset blocks, silencer and air filter as blocks that reach from the frame to the stack and the intake pipe, the ladder on the lamp beam and the platform on one stay.",
          "Bei 75 mm sind die Träger zu wenigen Massen verschmolzen: der Heckrahmen auf seiner Schwelle mit einem Balken unter dem Schurren-Drehlager, der Trichter auf zwei Knotenblöcken, Schalldämpfer und Luftfilter als Blöcke vom Rahmen bis zum Auspuffrohr und zum Ansaugrohr, die Leiter auf dem Leuchtenträger und das Podest auf einer Strebe."),
}
for scale, (en, de) in TEXT.items():
    V[str(scale)]["decisions"].append({"en": en, "de": de})
json.dump(doc, open(PATH, "w"), ensure_ascii=False, indent=1)
print("joined")
