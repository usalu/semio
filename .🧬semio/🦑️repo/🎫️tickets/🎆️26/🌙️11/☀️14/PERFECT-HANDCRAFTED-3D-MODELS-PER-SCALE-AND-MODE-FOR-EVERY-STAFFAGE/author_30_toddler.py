import sys, copy
from author_30_person import *

CAT, SLUG = "humans", "parent-carrying-toddler"
ORDER = ["face", "gloves", "jeans", "boots", "coat-torso", "coat-sleeves", "scarf", "beanie", "toddler-face", "toddler-suit", "toddler-boots", "toddler-mittens", "toddler-hat"]
DEFAULT_REGION = {"torso": "coat-torso", "pelvis": "coat-torso", "forearm-l": "coat-sleeves", "forearm-r": "coat-sleeves", "toddler-arm-n": "toddler-suit", "toddler-arm-f": "toddler-suit", "toddler-head": "toddler-face", "head": "face"}
NEST = {"coat-torso", "coat-sleeves", "jeans", "boots", "toddler-suit", "toddler-boots", "toddler-hat", "beanie"}
BIG = ("head", "toddler-head", "torso", "upper-arm", "forearm", "thigh", "shank", "foot", "pelvis", "toddler-torso", "toddler-thigh", "toddler-shank", "toddler-arm")


def mats_of(a):
    return {r["id"]: r["material"] for r in a["thermal"]["regions"]}


def build_all(scale):
    a = load_asset(CAT, SLUG)
    P = Prof30(SLUG, scale, pose=True)
    body = person_solids(P, scale, a, ORDER, mats_of(a), DEFAULT_REGION, tol=0.4 if scale == 50 else 0.5, toddler=True, skip_ids=("neck",), stroke_half={50: None, 100: None, 200: None, 500: 0.04}[scale])
    regs = {s["id"]: s.pop("_r", None) for s in body}
    return body, regs


def regmap_of(body, regs, scale):
    m = {}
    for s in body:
        r = regs.get(s["id"])
        if not r:
            continue
        nest = r in NEST and scale == 50 and any(s["bone"].startswith(b) for b in BIG)
        m.setdefault(r, []).append((s["id"], "nest" if nest else "outer"))
    return m


def cap_of(head, sid, region, mat, frac=0.42, grow=1.04):
    if head["kind"] == "loft":
        us = [q["at"][0] for q in head["sections"]]
        u0, u1 = min(us), max(us)
        rv = max(q["ry"] for q in head["sections"])
        rw = max(q["rx"] for q in head["sections"])
        h = ellipsoid(sid, head["bone"], mat, 0.3, "body", [u1 - frac * (u1 - u0), 0.0, 0.0], [frac * (u1 - u0) * 1.05, rv * grow, rw * grow])
    else:
        h = copy.deepcopy(head)
        h["id"] = sid
        h["radii"] = [r5(x * grow) for x in h["radii"]]
    h["region"] = region
    return h


def end_blob(limb, sid, region, mat, ft):
    sc = limb["sections"][-1] if limb["kind"] == "loft" else None
    if sc is None:
        return None
    r = max(0.55 * ft + 0.001, max(sc["rx"], sc["ry"]) * 1.05)
    return {"id": sid, "bone": limb["bone"], "material": mat, "role": "body", "tone": 0.3, "kind": "ellipsoid", "radii": [r5(r * 1.2), r5(r), r5(r)], "transform": {"translation": [r5(sc["at"][0] + 0.5 * r), sc["at"][1], sc["at"][2]]}, "region": region}


def ring_of(torso, sid, region, mat, ft, r_extra=0.02, length=0.12):
    sc = torso["sections"][-1]
    ln = max(length, 1.2 * ft)
    mk = lambda u: {"at": [r5(u), sc["at"][1], sc["at"][2]], "rx": r5(min(sc["rx"], 0.11) + r_extra), "ry": r5(min(sc["ry"], 0.11) + r_extra), "exponent": 2.3, "rotation": Q_U}
    return {"id": sid, "bone": torso["bone"], "material": mat, "role": "body", "tone": 0.5, "kind": "loft", "sections": [mk(sc["at"][0] - ln), mk(sc["at"][0] + 0.005)], "capStart": True, "capEnd": True, "region": region}


def coarse_extras(a, body, rm, scale):
    ft = FEATURE[str(scale)]
    mats = mats_of(a)
    by = {b["id"]: b for b in body}
    out = []
    if "scarf" not in rm and "torso" in by:
        out.append(ring_of(by["torso"], "th-scarf", "scarf", mats["scarf"], ft))
    if "toddler-hat" not in rm and "toddler-head" in by:
        out.append(cap_of(by["toddler-head"], "th-toddler-hat", "toddler-hat", mats["toddler-hat"]))
    if "gloves" not in rm:
        for sd in "lr":
            f = by.get(f"forearm-{sd}")
            if f:
                out.append(end_blob(f, f"th-hand-{sd}", "gloves", mats["gloves"], ft))
    if "toddler-mittens" not in rm:
        for sd in "nf":
            f = by.get(f"toddler-arm-{sd}")
            if f:
                out.append(end_blob(f, f"th-toddler-mitten-{sd}", "toddler-mittens", mats["toddler-mittens"], ft))
    return [o for o in out if o]


def thermal(a, body, regs, scale):
    rm = regmap_of(body, regs, scale)
    extra = emitters(a, scale, region_default={"parent": "face", "toddler": "toddler-face"}) + coarse_extras(a, body, rm, scale)
    return thermal_from_map(a, scale, body, rm, extra)


def structure(a, body, regs, scale):
    mem = member_bones(CAT, SLUG, scale)
    jb = {50: [b for b in mem if b.startswith(("upper-arm", "forearm", "thigh", "shank"))] + ["torso", "head"], 100: [b for b in mem if b.startswith(("upper-arm", "forearm", "thigh", "shank"))]}.get(scale, [])
    ms = {50: ["torso", "pelvis", "thigh-l", "thigh-r", "toddler-torso"], 100: ["torso", "pelvis", "thigh-l", "thigh-r", "toddler-torso"], 200: ["torso", "thigh-l", "thigh-r", "toddler-torso"], 500: ["torso", "toddler-torso"]}[scale]
    main = {b: [b] for b in mem}
    ms = [m for m in ms if m in {x["id"] for x in body}]
    return person_structure(a, scale, body, mem, (), main, joints=[j for j in jb if j in {x["bone"] for x in body}], masses=ms, kk={"torso": 0.5, "head": 0.5, "toddler-torso": 0.5})


DEC = {
    50: [
        ("Body at 1:50: parent and toddler are modelled as two figures of lofts sampled from this scale's own elevation, front and plan in the standing-carry pose (bone frames of the pose, not the rest pose); bones with two garments carry one loft per garment (head and beanie, torso and scarf, toddler head and hat, toddler shank and boot); the toe lofts are flat because the elevation draws the whole boot on the foot.", "Körper bei 1:50: Elternteil und Kleinkind sind als zwei Figuren aus Lofts modelliert, deren Schnitte den eigenen Seiten-, Vorder- und Draufsichten dieses Massstabs in der Trage-Pose folgen (Knochenrahmen der Pose, nicht der Ruhepose); Knochen mit zwei Kleidungsstücken tragen je einen Loft (Kopf und Mütze, Rumpf und Schal, Kleinkindkopf und Hut, Kleinkind-Unterschenkel und Stiefel); die Zehen-Lofts sind flach, weil die Seitenansicht den ganzen Stiefel am Fuss zeichnet."),
        ("Thermal at 1:50: multi-layer regions (coat, sleeves, jeans, boots, beanie, toddler suit, hat, boots) are nested shells, one per layer, the outer layer references the body loft; skin and single-layer regions reference the lofts directly; emitters of both figures are small tagged solids at their anchors. Several body-model regions (scarf, gloves, jeans, boots, toddler face, suit, mittens, boots) expose a different area than declared because the drawn garments are smaller or larger than the declared surface; the numbers are in the report.", "Thermik bei 1:50: mehrschichtige Regionen (Mantel, Ärmel, Jeans, Stiefel, Mütze, Kleinkind-Anzug, Hut, Stiefel) sind ineinander liegende Schalen, eine je Schicht, die äusserste Schicht verweist auf den Körper-Loft; Haut- und einschichtige Regionen verweisen direkt auf die Lofts; die Emitter beider Figuren sind kleine markierte Körper an ihren Ankern. Mehrere Körpermodell-Regionen (Schal, Handschuhe, Jeans, Stiefel, Kleinkindgesicht, -anzug, -fäustlinge, -stiefel) zeigen eine andere Fläche als deklariert, weil die gezeichneten Kleidungsstücke kleiner oder grösser als die deklarierte Fläche sind; die Zahlen stehen im Bericht."),
        ("Structure at 1:50: member cores along every load-bearing bone of the parent and the toddler, joints at the limb joints, neck and waist, masses at trunk, pelvis, thighs and the toddler trunk.", "Tragwerk bei 1:50: Stabkerne entlang jedes tragenden Knochens von Elternteil und Kleinkind, Gelenke an den Gliedergelenken, Hals und Taille, Massen an Rumpf, Becken, Oberschenkeln und dem Rumpf des Kleinkinds."),
    ],
    100: [
        ("Body at 1:100: the same two-figure construction regenerated from the 1:100 drawings with at most seven sections per bone.", "Körper bei 1:100: dieselbe Zwei-Figuren-Konstruktion, aus den 1:100-Zeichnungen mit höchstens sieben Schnitten je Knochen neu erzeugt."),
        ("Thermal at 1:100: one surface solid per part referencing the body; the scarf ring is a real solid because this scale draws no scarf.", "Thermik bei 1:100: ein Flächenkörper je Teil mit Verweis auf den Körper; der Schalring ist ein echter Körper, weil dieser Massstab keinen Schal zeichnet."),
        ("Structure at 1:100: members, joints and masses as at 1:50.", "Tragwerk bei 1:100: Stäbe, Gelenke und Massen wie bei 1:50."),
    ],
    200: [
        ("Body at 1:200: merged masses of at most five sections per bone from the 1:200 drawings for both figures.", "Körper bei 1:200: zusammengefasste Massen aus höchstens fünf Schnitten je Knochen aus den 1:200-Zeichnungen für beide Figuren."),
        ("Thermal at 1:200: every region has a solid; scarf ring and toddler mittens are small real solids because the scale draws none; only the total exposed area is compared.", "Thermik bei 1:200: jede Region hat einen Körper; Schalring und Kleinkind-Fäustlinge sind kleine echte Körper, weil der Massstab keine zeichnet; verglichen wird nur die Gesamtfläche."),
        ("Structure at 1:200: members per load-bearing bone, masses at trunk, thighs and the toddler trunk.", "Tragwerk bei 1:200: Stäbe je tragenden Knochen, Massen an Rumpf, Oberschenkeln und dem Rumpf des Kleinkinds."),
    ],
    500: [
        ("Body at 1:500: parent and toddler as lofts of at most five sections regenerated from the 1:500 drawings; hands and the far toddler arm have no drawing at this scale and are carried by the thermal-only blobs and by member capsules of the structure.", "Körper bei 1:500: Elternteil und Kleinkind als Lofts aus höchstens fünf Schnitten, aus den 1:500-Zeichnungen neu erzeugt; Hände und der ferne Kleinkindarm haben bei diesem Massstab keine Zeichnung und werden von rein thermischen Körpern und den Stabkapseln des Tragwerks getragen."),
        ("Thermal at 1:500: every region has a solid (scarf ring, gloves, toddler hat and mittens are small real solids because the scale draws no such part); only the total exposed area is compared.", "Thermik bei 1:500: jede Region hat einen Körper (Schalring, Handschuhe, Kleinkindhut und -fäustlinge sind kleine echte Körper, weil der Massstab solche Teile nicht zeichnet); verglichen wird nur die Gesamtfläche."),
        ("Structure at 1:500: members per load-bearing bone including the bones without a body solid at this scale (capsules from the bone length), a mass at each trunk.", "Tragwerk bei 1:500: Stäbe je tragenden Knochen einschliesslich der Knochen ohne Körper in diesem Massstab (Kapseln aus der Knochenlänge), eine Masse an jedem Rumpf."),
    ],
}


def run(scale, parts=("body", "thermal", "structure")):
    a = load_asset(CAT, SLUG)
    body, regs = build_all(scale)
    if "body" in parts:
        save_lists(CAT, SLUG, scale, body=body)
    d = load(CAT, SLUG)["volumes"][str(scale)]
    body = d["body"]
    if "thermal" in parts:
        save_lists(CAT, SLUG, scale, thermal=thermal(a, body, regs, scale))
    if "structure" in parts:
        save_lists(CAT, SLUG, scale, structure=structure(a, body, regs, scale))
    if scale in DEC:
        save_lists(CAT, SLUG, scale, decisions=[{"en": e, "de": d_} for e, d_ in DEC[scale]])
    print(fails(CAT, SLUG, scale))


if __name__ == "__main__":
    run(int(sys.argv[1]), tuple(sys.argv[2:]) or ("body", "thermal", "structure"))
