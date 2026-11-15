import sys
sys.path.insert(0, ".")
from author_09_build import *

slug = "skateboarder"
A = asset(slug)
V = seed(slug)
LIMBS = ("upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r", "thigh-l", "thigh-r", "shank-l", "shank-r", "hoodie")


def mitten(side):
    b = f"hand-{side}"
    r = ROT_U
    secs = [sec(-0.005, 0, 0, 0.018, 0.02, 3, r), sec(0.02, 0, 0, 0.026, 0.029, 3, r), sec(0.08, 0.002, 0, 0.0275, 0.03, 3, r), sec(0.14, 0.002, 0, 0.025, 0.026, 3, r), sec(0.17, 0.001, 0, 0.024, 0.025, 4.5, r), sec(0.182, 0.001, 0, 0.021, 0.0235, 4.5, r)]
    return loft(f"hand-{side}", b, "skin", 0.85, secs)


def ear(side):
    s = 1 if side == "l" else -1
    return ellipsoid(f"ear-{side}", "head", "skin", 0.86, [0.027, 0.012, 0.0075], [0.115, 0.008, s * 0.078])


body = V["50"]["body"]
body = [s for s in body if not s["id"].startswith("fist-")]
for s in body:
    if s["kind"] == "loft" and s["id"] in LIMBS:
        for q in s["sections"]:
            q["exponent"] = 4.0
    if s["id"] == "shoe-l" or s["id"] == "shoe-r":
        for q in s["sections"]:
            q["exponent"] = 3.2
for s in body:
    if s["id"] == "deck":
        for q in s["sections"]:
            x = abs(q["at"][0])
            q["at"][1] = {0.405: 0.089, 0.3726: 0.062}.get(x, q["at"][1])
            q["exponent"] = 6
        s["sections"][0]["at"][1] = s["sections"][-1]["at"][1] = 0.089
        s["sections"][0]["ry"] = s["sections"][-1]["ry"] = 0.0135
        s["sections"][1]["ry"] = s["sections"][-2]["ry"] = 0.0135
for s in body:
    if s["id"] == "nose":
        s["transform"]["translation"] = [0.1536, -0.1, 0]
        s["radii"] = [0.016, 0.013, 0.011]
for s in body:
    if s["id"].startswith("wheel-") and s["kind"] == "revolve":
        s["profile"] = [[-0.022, 0.022], [-0.0195, 0.0265], [0.0195, 0.0265], [0.022, 0.022]]
        s["origin"] = [0, 0, 0.0975]
        s["repeat"] = {"count": 2, "translation": [0, 0, -0.195]}
    if s["id"].startswith("axle-"):
        s["p0"], s["p1"] = [0, 0, -0.0975], [0, 0, 0.0975]
for s in body:
    if s["id"].startswith("forearm-"):
        keep = [q for q in s["sections"] if q["at"][0] <= 0.2731 + 0.01]
        last = copy.deepcopy(keep[-1])
        last["at"][0] = 0.2731 + 0.015
        last["rx"] *= 0.92
        last["ry"] *= 0.92
        s["sections"] = keep + [last]
body += [mitten("l"), mitten("r"), ear("l"), ear("r")]
order = ["deck", "truck-f", "truck-r", "wheel-f", "wheel-r"]
body = [s for s in body if s["id"] not in order and not s["id"].startswith("axle-")] + [next(x for x in V["50"]["body"] if x["id"] == i) for i in order]

DROP = {
    100: ["eye-l", "eye-r"],
    200: ["ear-l", "ear-r", "nose", "shoulders", "shoulder-l", "shoulder-r", "elbow-l", "elbow-r", "knee-l", "knee-r", "hood"],
    500: [],
}
if "--finish" not in sys.argv:
    vols = {"50": {"status": "authored", "denominator": 50, "body": body, "thermal": [], "structure": [], "decisions": []}}
    for s in (100, 200, 500):
        vols[str(s)] = {"status": "authored", "denominator": s, "body": [], "thermal": [], "structure": [], "decisions": []}
    save(slug, vols)
    print(fit(slug, 50))
    vols = json.load(open(f"{T}/volumes/humans/{slug}.json"))["volumes"]
    prev = vols["50"]["body"]
    for s in (100, 200, 500):
        b = coarsen_all(prev, s, DROP.get(s, ()))
        if s == 500:
            b = [x for x in b if x["id"] not in ("wheel-f", "wheel-r", "truck-f", "truck-r")]
            for side in ("f", "r"):
                b.append(box(f"wheel-{side}", f"wheel-{side}", "polyurethane", 0.9, [0.0975, 0.0775, 0.075], [0, 0.0115, 0.0975], repeat={"count": 2, "translation": [0, 0, -0.195]}))
        vols[str(s)]["body"] = b
        save(slug, vols)
        print(fit(slug, s))
        vols = json.load(open(f"{T}/volumes/humans/{slug}.json"))["volumes"]
        prev = vols[str(s)]["body"]
print("bodies done")
if "--bodies" in sys.argv:
    sys.exit()
SPEC = {
    "face": ["head", "=nose", "=ear-l", "=ear-r"],
    "beanie": ["beanie"],
    "hoodie-torso": ["hoodie", "hood", "shoulders"],
    "hoodie-sleeves": ["upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r"],
    "hands": ["hand-l", "hand-r"],
    "trousers": ["thigh-l", "thigh-r", "shank-l", "shank-r"],
    "shoes": ["shoe-l", "shoe-r"],
    "deck": ["deck"],
    "trucks": ["truck-f", "truck-r"],
    "wheels": ["wheel-f", "wheel-r"],
}


def struct_extra(a, body, scale):
    ids = {s["id"] for s in body}
    f = FEATURE[scale]
    out = [struct_ref("member-board", "deck", "board")]
    for side in ("f", "r"):
        if f"truck-{side}" in ids:
            out.append(struct_ref(f"member-truck-{side}", f"truck-{side}", "board"))
        out.append(struct_ref(f"member-wheel-{side}", f"wheel-{side}", f"wheel-{side}", "joint"))
    return out


CFG = dict(
    thermal={50: SPEC, 500: {**SPEC, "trucks": ["=wheel-f"]}},
    emitter_region={"*": "face"},
    skip_members=("board",),
    masses=True,
    struct_extra=struct_extra,
    joints={200: ["upper-arm-l", "upper-arm-r", "thigh-l", "thigh-r", "shank-l", "shank-r", "head"], 500: []},
    decisions={
        50: [
            D("Detailed model. Body: every limb, torso, head and shoe is a loft of superelliptic sections (exponent 4 for the near-rectangular sleeves and trouser legs) fitted to the drawings in the cruising pose; hands are mitten lofts instead of fists; ears, nose and eyes are small skin ellipsoids; the deck is a loft with raised kicktails, the trucks are boxes and the wheels revolves.", "Detailliertes Modell. Körper: Gliedmassen, Rumpf, Kopf und Schuhe sind Lofts aus superelliptischen Querschnitten (Exponent 4 für die fast rechteckigen Ärmel und Hosenbeine), in der Fahrhaltung an die Zeichnungen angepasst; die Hände sind Fäustlings-Lofts statt Fäusten; Ohren, Nase und Augen sind kleine Hautellipsoide; das Deck ist ein Loft mit hochgezogenen Kicktails, die Achsen sind Quader, die Räder Rotationskörper."),
            D("Thermal: every region has solids; the multi-layer regions (beanie, hoodie torso and sleeves, trousers, shoes) are nested shells, innermost layer first, each inset by the thicknesses of the layers outside it; the outer layer is the body solid itself (reference) where its surface equals the region area and a scaled copy where the drawn body overstates it. Nose, mouth and plume emitters are small solids at their anchors.", "Thermik: jede Region hat Körper; die mehrschichtigen Regionen (Mütze, Kapuzenpulli Rumpf und Ärmel, Hose, Schuhe) sind verschachtelte Schalen, innerste Schicht zuerst, jede um die Dicken der äusseren Schichten eingerückt; die äussere Schicht ist der Körper selbst (Verweis), wo seine Fläche der Regionsfläche entspricht, sonst eine skalierte Kopie, wo der gezeichnete Körper sie überschätzt. Nasen-, Mund- und Fahnenemitter sind kleine Körper an ihren Ankern."),
            D("Structure: every load-bearing bone is a member loft along its bone with a core section (about 60 % of the clothed limb section, 72 % in trunk and pelvis, 80 % in the head); joint balls sit at the rig joints, mass ellipsoids at the centres of mass of the heavy segments; the board is carried by its deck and the two trucks, each axle and wheel hub is a joint.", "Tragwerk: jeder lastabtragende Knochen ist ein Stab-Loft entlang seines Knochens mit einem Kernquerschnitt (etwa 60 % des bekleideten Gliedmassenquerschnitts, 72 % in Rumpf und Becken, 80 % im Kopf); Gelenkkugeln sitzen an den Rig-Gelenken, Massenellipsoide in den Schwerpunkten der schweren Segmente; das Brett wird von Deck und den beiden Achsen getragen, jede Achse und Radnabe ist ein Gelenk."),
        ],
        100: [
            D("1:100 is modelled anew against the 1:100 drawings: the sections of every loft are reduced to the few that carry the outline (tolerance 7.5 mm), eyes are dropped (below 15 mm), and every part is refitted to the 1:100 silhouette.", "1:100 ist neu gegen die 1:100-Zeichnungen modelliert: die Querschnitte jedes Lofts sind auf die wenigen reduziert, die die Kontur tragen (Toleranz 7,5 mm), die Augen entfallen (unter 15 mm), und jeder Teil ist an die 1:100-Silhouette neu angepasst."),
            D("Thermal: one shell per region (the outer surface; the thin clothing layers are merged into it because they are below 15 mm), nested layers are no longer separated. Structure: member lofts with three sections, joint balls kept at all rig joints.", "Thermik: eine Schale je Region (die äussere Oberfläche; die dünnen Bekleidungsschichten sind darin zusammengefasst, da sie unter 15 mm liegen), verschachtelte Schichten werden nicht mehr getrennt. Tragwerk: Stab-Lofts mit drei Querschnitten, Gelenkkugeln an allen Rig-Gelenken."),
        ],
        200: [
            D("1:200 merges: ears, nose, hood, shoulder, elbow and knee balls are absorbed into the limb and torso lofts (below 30 mm or inside the neighbouring section); lofts keep two or three sections; the head, beanie and torso keep their separate materials.", "1:200 fasst zusammen: Ohren, Nase, Kapuze, Schulter-, Ellbogen- und Kniekugeln gehen in den Gliedmassen- und Rumpf-Lofts auf (unter 30 mm oder innerhalb des Nachbarquerschnitts); die Lofts behalten zwei oder drei Querschnitte; Kopf, Mütze und Rumpf behalten ihre eigenen Materialien."),
            D("Thermal: one solid per region, deck, trucks and wheels remain. Structure: member lofts with two sections; only the shoulder, hip, knee and neck joints stay as balls, the other joints are merged into the overlapping members.", "Thermik: ein Körper je Region, Deck, Achsen und Räder bleiben. Tragwerk: Stab-Lofts mit zwei Querschnitten; nur Schulter-, Hüft-, Knie- und Halsgelenke bleiben als Kugeln, die übrigen Gelenke gehen in den überlappenden Stäben auf."),
        ],
        500: [
            D("1:500 is a generalised silhouette model: lofts of two or three sections whose smallest dimension is 75 mm; the pelvis ball is absorbed, trucks and wheels are merged into one 75 mm wheel block per axle (the drawings enlarge the wheels to 100 mm the same way); the deck is as thick as the smallest feature allows.", "1:500 ist ein verallgemeinertes Silhouettenmodell: Lofts aus zwei oder drei Querschnitten, deren kleinste Abmessung 75 mm beträgt; die Beckenkugel geht auf, Achsen und Räder sind je Achse zu einem 75-mm-Radblock zusammengefasst (die Zeichnungen vergrössern die Räder ebenso auf 100 mm); das Deck ist so dick, wie es das kleinste Merkmal erlaubt."),
            D("Thermal: the region solids cannot be smaller than 75 mm, so trucks and wheels share the wheel blocks and the areas are matched by scaling the 75 mm solids within the allowed tolerance. Structure: member lofts with two sections, joints merged into the members.", "Thermik: die Regionskörper können nicht kleiner als 75 mm sein, daher teilen sich Achsen und Räder die Radblöcke, und die Flächen werden durch Skalieren der 75-mm-Körper innerhalb der Toleranz getroffen. Tragwerk: Stab-Lofts mit zwei Querschnitten, Gelenke in den Stäben zusammengefasst."),
        ],
    },
)
vols = json.load(open(f"{T}/volumes/humans/{slug}.json"))["volumes"]
finish(slug, vols, CFG)
print("finished")
