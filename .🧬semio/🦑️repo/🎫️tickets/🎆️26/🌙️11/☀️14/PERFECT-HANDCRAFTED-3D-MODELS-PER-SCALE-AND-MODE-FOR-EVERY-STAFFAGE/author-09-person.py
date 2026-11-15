"""Generic human driver: python3 author-09-person.py <slug> [--finish|--bodies]  (see author-09.md)"""
import sys
sys.path.insert(0, ".")
from author_09_build import *

slug = sys.argv[1]
A = asset(slug)
V = seed(slug)
L = {b["id"]: b["lengthM"] for b in A["rig"]["bones"]}
EXP = {"arm": 4.0, "forearm": 4.0, "upper-arm": 4.0, "thigh": 4.0, "shank": 4.0}


def mitten(side):
    b = f"hand-{side}"
    h = L[b]
    k = h / 0.182
    r = ROT_U
    secs = [sec(-0.005, 0, 0, 0.018 * k, 0.02 * k, 3, r), sec(0.11 * h, 0, 0, 0.026 * k, 0.029 * k, 3, r), sec(0.44 * h, 0.002, 0, 0.0275 * k, 0.03 * k, 3, r), sec(0.77 * h, 0.002, 0, 0.025 * k, 0.026 * k, 3, r), sec(0.93 * h, 0.001, 0, 0.024 * k, 0.025 * k, 4.5, r), sec(h, 0.001, 0, 0.021 * k, 0.0235 * k, 4.5, r)]
    return loft(f"hand-{side}", b, "skin", 0.85, secs)


body = copy.deepcopy(V["50"]["body"])
body = [s for s in body if not s["id"].startswith("fist-")]
have = {s["id"] for s in body}
for s in body:
    if s["kind"] == "loft":
        pre = s["bone"].rsplit("-", 1)[0]
        if pre in EXP:
            for q in s["sections"]:
                q["exponent"] = EXP[pre]
        if pre in ("forearm",):
            lim = L[s["bone"]] + 0.01
            keep = [q for q in s["sections"] if q["at"][0] <= lim]
            last = copy.deepcopy(keep[-1])
            last["at"][0] = L[s["bone"]] + 0.015
            last["rx"] *= 0.92
            last["ry"] *= 0.92
            s["sections"] = keep + [last]
for side in ("l", "r"):
    if f"hand-{side}" not in have:
        body.append(mitten(side))

DROPS = {100: lambda s: s["role"] == "detail" and tiny(s, 100), 200: lambda s: s["role"] == "detail" or (s["kind"] == "ellipsoid" and s["id"] not in KEEP), 500: lambda s: False}
KEEP = {"pelvis"}


def tiny(s, scale):
    f = FEATURE[scale]
    if s["kind"] == "ellipsoid":
        return min(s["radii"]) * 2 < f
    return False


if "--finish" not in sys.argv:
    vols = {"50": {"status": "authored", "denominator": 50, "body": body, "thermal": [], "structure": [], "decisions": []}}
    for s in (100, 200, 500):
        vols[str(s)] = {"status": "authored", "denominator": s, "body": [], "thermal": [], "structure": [], "decisions": []}
    save(slug, vols)
    print(fit(slug, 50))
    vols = json.load(open(f"{T}/volumes/humans/{slug}.json"))["volumes"]
    prev = vols["50"]["body"]
    for s in (100, 200, 500):
        drop = [x["id"] for x in prev if DROPS[s](x)]
        b = coarsen_all(prev, s, drop)
        vols[str(s)]["body"] = b
        save(slug, vols)
        print(fit(slug, s))
        vols = json.load(open(f"{T}/volumes/humans/{slug}.json"))["volumes"]
        prev = vols[str(s)]["body"]
    print("bodies done")
if "--bodies" in sys.argv:
    sys.exit()

spec = {}
for t in V["50"]["thermal"]:
    if "of" in t:
        spec.setdefault(t["region"], []).append(t["of"])
for t in V["50"]["body"]:
    pass
title = A["title"]["en"]
CFG = dict(
    thermal={50: spec},
    emitter_region={"*": "face-skin" if any(r["id"] == "face-skin" for r in A["thermal"]["regions"]) else "face"},
    masses=True,
    joints={200: [b for b in member_bones(A) if b.split("-")[0] in ("upper", "arm", "thigh", "shank", "head", "neck", "torso")], 500: []},
    decisions={
        50: [D(f"{title}, 1:50: every limb, torso, head and shoe is a loft of superelliptic sections (exponent 4 for the near-rectangular sleeves and legs) fitted to the drawings in the default pose; hands are mitten lofts, the forearm sleeve ends at the cuff; small details are skin ellipsoids.", f"{title}, 1:50: Gliedmassen, Rumpf, Kopf und Schuhe sind Lofts aus superelliptischen Querschnitten (Exponent 4 für die fast rechteckigen Ärmel und Beine), in der Standardhaltung an die Zeichnungen angepasst; die Hände sind Fäustlings-Lofts, der Unterarmärmel endet an der Manschette; kleine Details sind Hautellipsoide."),
             D("Thermal: every region has solids that are the body surface itself (references), multi-layer regions are nested shells innermost layer first, each inset by the layers outside it; emitters are small solids at their anchors, the plume a thin disc at the crown.", "Thermik: jede Region hat Körper, die die Körperoberfläche selbst sind (Verweise), mehrschichtige Regionen sind verschachtelte Schalen, innerste Schicht zuerst, jede um die äusseren Schichten eingerückt; Emitter sind kleine Körper an ihren Ankern, die Fahne eine dünne Scheibe am Scheitel."),
             D("Structure: a member loft along every load-bearing bone (core section about 60 % of the clothed section, 72 % trunk and pelvis, 80 % head), joint balls at the rig joints, mass ellipsoids at the centres of mass of the heavy segments.", "Tragwerk: ein Stab-Loft entlang jedes lastabtragenden Knochens (Kernquerschnitt etwa 60 % des bekleideten Querschnitts, 72 % Rumpf und Becken, 80 % Kopf), Gelenkkugeln an den Rig-Gelenken, Massenellipsoide in den Schwerpunkten der schweren Segmente.")],
        100: [D("1:100 is modelled anew against the 1:100 drawings: loft sections reduced to those that carry the outline (7.5 mm), details below 15 mm dropped, every part refitted to the 1:100 silhouette. Thermal and structure follow the new body: references and shells of the coarser sections, member lofts with three sections.", "1:100 ist neu gegen die 1:100-Zeichnungen modelliert: Loft-Querschnitte auf die konturtragenden reduziert (7,5 mm), Details unter 15 mm entfallen, jeder Teil an die 1:100-Silhouette neu angepasst. Thermik und Tragwerk folgen dem neuen Körper: Verweise der gröberen Querschnitte, Stab-Lofts mit drei Querschnitten.")],
        200: [D("1:200 merges: ball joints and small details are absorbed by the neighbouring lofts (below 30 mm), lofts keep two or three sections; the head, hair and torso keep their materials. Thermal: one surface per region; structure: member lofts with two sections, only the main joints stay as balls.", "1:200 fasst zusammen: Kugelgelenke und kleine Details gehen in den Nachbar-Lofts auf (unter 30 mm), die Lofts behalten zwei oder drei Querschnitte; Kopf, Haar und Rumpf behalten ihre Materialien. Thermik: eine Oberfläche je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten, nur die Hauptgelenke bleiben als Kugeln.")],
        500: [D("1:500 is a generalised silhouette model: lofts of two or three sections whose smallest dimension is the 75 mm feature, everything smaller is merged into its neighbour. Thermal references follow; structure: member lofts with two sections, joints merged into the members.", "1:500 ist ein verallgemeinertes Silhouettenmodell: Lofts aus zwei oder drei Querschnitten, deren kleinste Abmessung das 75-mm-Merkmal ist, alles Kleinere geht im Nachbarn auf. Thermische Verweise folgen; Tragwerk: Stab-Lofts mit zwei Querschnitten, Gelenke in den Stäben zusammengefasst.")],
    },
)
vols = json.load(open(f"{T}/volumes/humans/{slug}.json"))["volumes"]
finish(slug, vols, CFG)
print("finished")
