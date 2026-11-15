import sys
from author_07_lib import *
from author_22_lib import *
import author_07_lib as L
import importlib.util
spec = importlib.util.spec_from_file_location("c7", f"{L.T}/author-07-cyclist.py")
c7 = importlib.util.module_from_spec(spec); spec.loader.exec_module(c7)

SC = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
NOTES = {
 50: (d("1:50 body: the draft solids repaired against the 1:50 drawings and the real object. Limbs, torso, neck and head are lofts with elliptic sections fitted laterally to the posed front and plan drawings (a thigh is not a cylinder, a torso is wider at the shoulders), helmet and hair as separate shells, gloves and shoes as lofts, the bicycle as tubes of true diameter, tyres as 36-segment rings of lateral oval sections, rims, hubs, axles, cranks, chainring and chain, fenders as thin arcs. Spokes are drawn as texture only and are not modelled. Thermal: every region has solids; multi-layer clothing regions are nested shells (innermost first, real thickness), the outer layer is the body solid itself; the breath emitters and the plume are small solids at their anchors. Structure: bone members with core sections, joints at the rig joints, masses at pelvis, torso, head and thighs, every frame tube, fork, bar, rim and crank as a member.",
         "1:50-Körper: die Entwurfskörper wurden gegen die 1:50-Zeichnungen und das reale Objekt repariert. Gliedmaßen, Rumpf, Hals und Kopf sind Lofts mit elliptischen Querschnitten, seitlich an Front- und Grundriss der Pose angepasst (ein Oberschenkel ist kein Zylinder, ein Rumpf ist an den Schultern breiter), Helm und Haar als eigene Schalen, Handschuhe und Schuhe als Lofts, das Fahrrad aus Rohren wahren Durchmessers, die Reifen als 36-teilige Ringe mit ovalem Querschnitt, dazu Felgen, Naben, Achsen, Kurbeln, Kettenblatt und Kette, die Schutzbleche als dünne Bögen. Speichen sind nur Textur und nicht modelliert. Thermik: jede Region hat Körper; mehrschichtige Bekleidungsregionen sind verschachtelte Schalen (innerste zuerst, wahre Dicke), die äußere Schicht ist der Körper selbst; die Atem-Emitter und die Fahne sind kleine Körper an ihren Ankern. Tragwerk: Knochenglieder mit Kernquerschnitt, Gelenke an den Rig-Gelenken, Massen an Becken, Rumpf, Kopf und Oberschenkeln, jedes Rahmenrohr, jede Gabel, Lenkerstange, Felge und Kurbel als Glied.")),
 100: (d("1:100 body modelled anew from this scale's own drawings: human bones as lofts with elliptic sections fitted to the front and plan drawings, bicycle as the drawn tyres, hubs, fenders, bars, saddle, cranks and pedals; spokes, zips and thumbs are below the 15 mm feature or drawn only as texture and are left out. Thermal: one skin solid per region and body solid, areas fitted to areaM2 (30 %); layers are not nested at this scale. Structure: bone members with core sections, joints at hips, knees, ankles, shoulders and elbows, masses at pelvis, torso, head and thighs, bicycle tubes as members.",
         "Der Körper 1:100 ist aus den eigenen Zeichnungen dieses Maßstabs neu modelliert: Körperknochen als Lofts mit elliptischen Querschnitten nach Front- und Grundriss, das Fahrrad als gezeichnete Reifen, Naben, Schutzbleche, Lenker, Sattel, Kurbeln und Pedale; Speichen, Reißverschlüsse und Daumen liegen unter dem Merkmal von 15 mm oder sind nur Textur und entfallen. Thermik: eine Hautschale je Region und Körperteil, Flächen auf areaM2 abgestimmt (30 %); Schichten werden in diesem Maßstab nicht verschachtelt. Tragwerk: Knochenglieder mit Kernquerschnitt, Gelenke an Hüfte, Knie, Knöchel, Schulter und Ellbogen, Massen an Becken, Rumpf, Kopf und Oberschenkeln, Fahrradrohre als Glieder.")),
 200: (d("1:200 body modelled anew against this scale's drawings: merged forearm-and-hand and head masses, single stem-and-bars solids, hubs as discs; features below 30 mm (zips, thumbs, ears) are merged. Thermal skins fitted to areaM2 (30 %), structure with members, four-joint set and masses.",
         "Der Körper 1:200 ist neu gegen die Zeichnungen dieses Maßstabs modelliert: Unterarm und Hand sowie der Kopf sind zu Massen zusammengefasst, Vorbau und Lenker als einzelne Körper, Naben als Scheiben; Merkmale unter 30 mm (Reißverschlüsse, Daumen, Ohren) sind verschmolzen. Thermische Hüllen auf areaM2 abgestimmt (30 %), Tragwerk mit Gliedern, vier Gelenken und Massen.")),
 500: (d("1:500 body modelled anew as merged masses: head with helmet, torso, seat, limbs as single lofts, bicycle as tyres, frame triangles, fork and bars-and-saddle. Everything below 75 mm is merged into its neighbour. Thermal skins fitted to areaM2 (30 %), structure keeps every load-bearing limb and frame member with two joints per leg and arm.",
         "Der Körper 1:500 ist neu als zusammengefasste Massen modelliert: Kopf mit Helm, Rumpf, Sitz, Gliedmaßen als einzelne Lofts, Fahrrad als Reifen, Rahmendreiecke, Gabel sowie Lenker und Sattel. Alles unter 75 mm ist in den Nachbarn verschmolzen. Thermische Hüllen auf areaM2 abgestimmt (30 %), das Tragwerk behält jedes tragende Glied und Rahmenteil mit zwei Gelenken je Bein und Arm.")),
}

import author_22_bike as bk, json as _j
B50 = _j.load(open(f"{L.T}/🗑️generated/author-22/cyclist.v50.json"))["volumes"]["50"]["body"]

def build(scale):
    if scale == 50:
        B = c7.build50()
        B.drop('glove-thumb-l'); B.drop('glove-thumb-r')
        return B
    B = Builder("cyclist", scale)
    humanize(B, skip={"jacket-zip-1", "jacket-zip-2", "jacket-reflective", "glove-thumb-l", "glove-thumb-r"} if scale > 50 else ())
    B100 = build(100) if scale in (200, 500) else None
    if scale in (200, 500):
        f = {200: 0.0156, 500: 0.0386}[scale]
        take = {200: {"hand-l": "glove-l", "hand-r": "glove-r"},
                500: {"upper-arm-l": "sleeve-l", "upper-arm-r": "sleeve-r", "forearm-l": "forearm-l", "forearm-r": "forearm-r", "hand-l": "glove-l", "hand-r": "glove-r", "neck": "neck"}}[scale]
        for nid, sid in take.items():
            if B.has(nid):
                continue
            c = copy.deepcopy(B100.get(sid)); c["id"] = nid
            if nid.startswith("hand"):
                for q in c["sections"]:
                    q["rx"] *= {200: 1.45, 500: 1.3}[scale]; q["ry"] *= {200: 1.45, 500: 1.3}[scale]
            c = bk.floor_any(c, f)
            B.add(c, B100.reg[sid])
    for s in bk.bike(scale, B50):
        reg = "tyres" if "tyre" in s["id"] else ("saddle" if s["id"] == "saddle" else "bike-metal")
        B.add(s, reg)
    if scale == 100:
        for sid in ("hair-tail-1", "hair-tail-2", "hair-tail-3"):
            bk.floor_any(B.get(sid), 0.0079)
    if scale == 200:
        for sid in ("forearm-hand-l", "forearm-hand-r"):
            B.reg[sid] = ["jacket"]
    if scale == 500:
        for sid in ("forearm-hand-r-1", "forearm-hand-r-2"):
            if B.has(sid):
                B.reg[sid] = ["jacket"]
        B.reg["head-helmet"] = ["face-skin"]
        B.reg["torso-mass"] = ["jacket"]
        head = B.get("head-helmet")
        h = copy.deepcopy(head); h["id"] = "helmet-head"; h["material"] = B100.get("helmet")["material"]; h["tone"] = B100.get("helmet")["tone"]
        h["sections"] = h["sections"][1:]
        for q in h["sections"]:
            q["rx"] *= 1.12; q["ry"] *= 1.12
        B.add(h, ["helmet"])
        tail = copy.deepcopy(B100.get("hair-tail-1")); tail["id"] = "hair-tail"; tail["sections"] = tail["sections"][:5]
        B.add(bk.floor_any(tail, 0.0386), ["hair"])
        for s in B.body:
            bk.floor_any(s, 0.0327 if s["bone"] == "frame" else 0.0386)
    return B

if __name__ == "__main__":
    for sc in SC:
        B = build(sc)
        print(snap(B, [x['id'] for x in B.body if x['kind'] == 'loft' and (sc == 50 or not x['id'].startswith('hair'))], ('grow=60 max=0.2' if sc != 50 else '')).strip()[-300:])
        B.structure = human_structure(B, c7.HUMAN_MEMBERS, c7.JOINTS[sc], c7.MASSES)
        if sc == 50:
            for sid in ("tube-seat", "tube-top", "tube-down", "tube-head", "tube-chainstay-l", "tube-chainstay-r", "tube-seatstay-l", "tube-seatstay-r", "fork-l", "fork-r", "stem", "seatpost", "handlebar-l", "handlebar-r"):
                B.structure.append({"id": f"member-{sid}", "of": sid, "member": "frame", "part": "member"})
            B.structure += [{"id": "member-rear-tyre", "of": "rear-tyre", "member": "rear-wheel", "part": "member"}, {"id": "member-front-tyre", "of": "front-tyre", "member": "front-wheel", "part": "member"},
                            {"id": "joint-rear-hub", "of": "rear-hub", "member": "rear-wheel", "part": "joint"}, {"id": "joint-front-hub", "of": "front-hub", "member": "front-wheel", "part": "joint"},
                            {"id": "member-crank-arm-r", "of": "crank-arm-r", "member": "crank-r", "part": "member"}, {"id": "member-crank-arm-l", "of": "crank-arm-l", "member": "crank-l", "part": "member"}]
        for s in ([] if sc == 50 else B.body):
            if s["bone"] in ("frame",) and s["role"] == "body" and not s["id"].startswith(("saddle", "headlamp")) and "fender" not in s["id"] and "grip" not in s["id"]:
                B.structure.append({"id": f"member-{s['id']}", "of": s["id"], "member": "frame", "part": "member"})
            if s["id"] in ("rear-tyre", "front-tyre"):
                B.structure.append({"id": f"member-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
            if s["id"].startswith("crank-arm"):
                B.structure.append({"id": f"member-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
        B.decisions = [NOTES[sc]]
        extra = [emitter_solid(B, em, anc, dia, reg, fl) for em, anc, dia, reg, fl in (("breath-nose","nose",0.0095,"face-skin",False),("breath-mouth","mouth",0.0124,"face-skin",False),("plume","crown",0.2,"jacket",True))]
        if sc == 500:
            for cb in ("crank-r", "crank-l"):
                B.structure.append({"id": f"member-{cb}", "kind": "capsule", "bone": cb, "p0": [0, 0, 0.0], "p1": [0.17, 0, 0.0], "r0": 0.0386, "r1": 0.0386, "member": cb, "part": "member", "material": "steel", "role": "body", "tone": 0.45})
        fails, dropped = finish_thermal(B, extra)
        print(sc, len(B.body), len(B.thermal), len(B.structure), 'dropped', sorted(dropped), 'area', fails)
