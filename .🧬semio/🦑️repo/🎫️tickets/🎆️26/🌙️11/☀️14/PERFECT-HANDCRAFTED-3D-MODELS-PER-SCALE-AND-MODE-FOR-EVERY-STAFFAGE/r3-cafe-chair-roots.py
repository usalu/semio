"""🌱 Round 3, furniture/cafe-chair: the root corrections outside `rig`, `volumes` and the 2D shapes: summary, provenance, dimensions, materials, anchors, the tipping pivot, loads, thermal regions and assumptions, structure supports and assumptions.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_cafe_chair_kit", os.path.join(HERE, "r3-cafe-chair-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

AREAS = {"steel-tube": 0.22, "back-beech": 0.215, "seat-beech": 0.51}
TILT = 12.0
DEPTH_M = 0.44

SUMMARY = {
    "en": "A bistro café chair: a powder-coated Ø22 mm steel tube frame whose rear legs bend at the seat into the reclined back posts, two front legs and two side stretchers, a beech seat board on a beech apron frame and a ladder back of a curved beech backrest board and a lower rail; near and far legs coincide in side elevation. It can be pulled out, sat on by a 75 kg occupant and tipped back onto its two rear feet.",
    "de": "Ein Bistro-Caféstuhl: ein pulverbeschichtetes Stahlrohrgestell Ø22 mm, dessen Hinterbeine am Sitz in die geneigten Lehnenpfosten umbiegen, zwei Vorderbeine und zwei seitliche Fußstreben, ein Buchensitzbrett auf einem Buchenzargenrahmen und eine Sprossenlehne aus einem gebogenen Buchenrückenbrett und einer unteren Leiste; nahe und ferne Beine fallen im Seitenaufriss zusammen. Er lässt sich herausziehen, von einer Person mit 75 kg besetzen und auf die beiden hinteren Füße kippen.",
}

REFERENCES = [
    {"id": "thonet-214", "citation": "Thonet GmbH, bentwood chair 214, product data (width 43 cm, height 84 cm, seat height 46 cm, depth 52 cm, 2.7 kg, plastic glides), https://www.thonet.de/en/all-products/detail/214, accessed 2026-11-20."},
    {"id": "fermob-bistro", "citation": "Fermob, Bistro chair, retailer data sheets (steel frame, 420 W x 500 D x 820 H mm, seat height 460 mm), https://archipro.com.au/product/bistro-natural-chair-fermob-au, accessed 2026-11-20."},
    {"id": "cafe-vii-datasheet", "citation": "Café VII chair, technical description (steel tubes Ø16 x 2.0 mm, profiled beech plywood, overall height 855 mm, seat 395 x 425 mm), https://cdn.abicart.com/shop/32642/art96/156231896-c82da7-CAFE_VII.pdf, accessed 2026-11-20."},
]

NOTES = {
    "en": "Side elevation; near and far legs share one projection. Construction after café chairs of the bistro type (Thonet 214 proportions: seat height 46 cm, rear leg and back post one bent member, feet of the rear legs under the back top): Ø22 x 2 mm steel tube frame, 20 mm beech seat board raked 2 degrees on 18 x 45 mm beech rails fixed to the tubes with welded tabs, 18 mm curved beech backrest board and rail screwed to the post fronts, nylon glides. Masses from the modelled volumes (steel 7850, beech 720, polyamide 1150 kg/m3). Tipping is a rigid rotation about the rear edges of the rear glides, not a bending analysis. No occupant load in the empty poses.",
    "de": "Seitenansicht; nahe und ferne Beine liegen in einer Projektion. Konstruktion nach Caféstühlen vom Bistro-Typ (Proportionen des Thonet 214: Sitzhöhe 46 cm, Hinterbein und Lehnenpfosten ein gebogenes Glied, Füße der Hinterbeine unter der Lehnenoberkante): Stahlrohrgestell Ø22 x 2 mm, 20 mm Buchensitzbrett 2 Grad geneigt auf Buchenleisten 18 x 45 mm, mit angeschweißten Laschen an den Rohren befestigt, gebogenes 18-mm-Buchenrückenbrett und Leiste auf die Pfostenvorderseiten geschraubt, Nylongleiter. Massen aus den modellierten Volumen (Stahl 7850, Buche 720, Polyamid 1150 kg/m3). Das Kippen ist eine starre Drehung um die hinteren Kanten der hinteren Gleiter, keine Biegeanalyse. Keine Personenlast in den unbesetzten Posen."
}

NYLON = {
    "id": "polyamide-glide",
    "title": {"en": "Polyamide (nylon) glides and caps", "de": "Polyamid-Gleiter und -Kappen"},
    "emissivity": 0.9,
    "solarAbsorptivity": 0.6,
    "densityKgM3": 1150,
    "conductivityWmK": 0.25,
    "specificHeatJKgK": 1600,
    "source": "literature",
    "reference": "iso-10456-2007",
}

CONTACT_Z = {"contact-rear-left": K.REAR_FOOT_Z, "contact-rear-right": -K.REAR_FOOT_Z, "contact-front-left": K.FRONT_FOOT_Z, "contact-front-right": -K.FRONT_FOOT_Z}


def tilt_root():
    """↩ The root placement that turns the chair by TILT degrees about the rear bottom edge of the rear glides (x = -glide radius, y = 0)."""
    a = math.radians(TILT)
    px, py = -K.GLIDE_R, 0.0
    rx, ry = px * math.cos(a) - py * math.sin(a), px * math.sin(a) + py * math.cos(a)
    return {"x": round(px - rx, 6), "y": round(py - ry, 6), "angleDeg": TILT}


def bbox():
    """📦 The overall extents (x, y, z) of the 1:50 body in the rest pose, from the measures: rear glide edge to seat front edge, floor to cap top, front glide outer edges."""
    x0 = min(-K.GLIDE_R, K.POST_TOP[0] - K.R)
    x1 = max(K.seat_xy(K.SEAT_L, 0.0)[0], K.FRONT_FOOT[0] + K.GLIDE_R)
    y1 = K.CORNER[1] + (K.POST_L + K.CAP_H) * math.sin(math.radians(K.POST_DEG))
    z1 = max(K.FRONT_FOOT_Z + K.GLIDE_R, K.SEAT_FRONT_W, K.BACK_W)
    return x1 - x0, y1, 2 * z1


def apply(asset):
    """🌱 Writes every root correction except rig, volumes and 2D into the asset in memory."""
    asset["summary"] = SUMMARY
    prov = asset["provenance"]
    known = {r["id"] for r in prov["references"]}
    prov["references"] = prov["references"] + [r for r in REFERENCES if r["id"] not in known]
    prov["notes"] = NOTES
    w, h, d = bbox()
    m = K.masses()
    asset["dimensions"] = {"heightM": round(h, 2), "widthM": round(w, 3), "depthM": DEPTH_M, "massKg": round(sum(v[0] for k, v in m.items() if k != "parts"), 1)}
    if not any(x["id"] == NYLON["id"] for x in asset["materials"]):
        asset["materials"] = asset["materials"] + [NYLON]
    for anchor in asset["anchors"]:
        if anchor["id"] in CONTACT_Z:
            anchor.update(x=0, y=0, z=CONTACT_Z[anchor["id"]], footprintLengthM=0.025, footprintWidthM=0.025)
    for pose in asset["poses"]:
        if pose["id"] == "tilted-on-two-legs":
            pose["root"] = tilt_root()
            pose["description"] = {"en": "Rigid rotation of 12 degrees about the rear edges of the rear glides; the front feet are lifted.", "de": "Starre Drehung um 12 Grad um die hinteren Kanten der hinteren Gleiter; die vorderen Füße sind angehoben."}
        for load in pose.get("loads", []):
            if load["id"] == "occupant-back":
                load.update(x=round((K.BOARD_U0 + K.BOARD_U1) / 2, 3), y=round(K.back_vc(0.0) - K.BACK_T / 2, 3))
    th = asset["thermal"]
    for region in th["regions"]:
        if region["id"] == "seat-beech":
            region["title"] = {"en": "Beech seat board and apron frame", "de": "Buchensitzbrett und Zargenrahmen"}
            layer = region["layers"][0]
            cap = round(720 * 1600 * K.BOARD_T)
            layer.update(thicknessM=K.BOARD_T, resistanceM2KW=round(K.BOARD_T / 0.18, 6), heatCapacityJM2K=cap)
            region["heatCapacityJM2K"] = cap
        if region["id"] == "back-beech":
            region["title"] = {"en": "Beech backrest board and rail", "de": "Buchenrückenbrett und -leiste"}
        if region["id"] == "steel-tube":
            region["title"] = {"en": "Steel tube frame", "de": "Stahlrohrgestell"}
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    th["assumptions"] = [
        th["assumptions"][0],
        {"en": "Thin tubes (2 mm wall) respond within minutes (7 kJ/m2K), the 20 mm beech seat board within an hour (23 kJ/m2K): heat capacity is rho*cp*min(thickness, sqrt(alpha*3600 s)), values after ISO 10456.", "de": "Dünne Rohre (2 mm Wand) reagieren in Minuten (7 kJ/m2K), das 20-mm-Buchensitzbrett in einer Stunde (23 kJ/m2K): Wärmekapazität ist rho*cp*min(Dicke, sqrt(alpha*3600 s)), Werte nach ISO 10456."},
        th["assumptions"][2],
        {"en": f"Near and far tubes are one projected tube in the side drawing but all count in the {AREAS['steel-tube']} m2 steel area: two rear tubes running from the glides over the bend to the post caps, two front legs and two Ø16 stretchers.", "de": f"Nahe und ferne Rohre sind in der Seitenzeichnung ein projiziertes Rohr, zählen aber alle zur Stahlfläche {str(AREAS['steel-tube']).replace('.', ',')} m2: zwei Hinterrohre von den Gleitern über den Bogen bis zu den Pfostenkappen, zwei Vorderbeine und zwei Streben Ø16."},
    ]
    st = asset["structure"]
    st["supports"] = ["contact-rear-left", "contact-rear-right", "contact-front-left", "contact-front-right"]
    st["assumptions"] = [
        {"en": f"One contact per glide (rear feet at z = +-{K.REAR_FOOT_Z} m, front feet at z = +-{K.FRONT_FOOT_Z} m); tipping is a rigid rotation about the rear edges of the two rear glides.", "de": f"Ein Kontakt je Gleiter (hintere Füße bei z = +-{str(K.REAR_FOOT_Z).replace('.', ',')} m, vordere Füße bei z = +-{str(K.FRONT_FOOT_Z).replace('.', ',')} m); Kippen ist eine starre Drehung um die hinteren Kanten der beiden hinteren Gleiter."},
        {"en": "Near and far legs coincide in elevation and are drawn once; each rear leg and its back post are one bent tube (a rigid frame corner at the bend); the occupant is a 75 kg point load on the seat plus a 60 N lean on the backrest board (authored estimates).", "de": "Nahe und ferne Beine fallen im Aufriss zusammen und sind einmal gezeichnet; jedes Hinterbein ist mit seinem Lehnenpfosten ein gebogenes Rohr (eine biegesteife Rahmenecke am Bogen); die Person ist eine Punktlast von 75 kg auf dem Sitz plus 60 N Anlehnung am Rückenbrett (Schätzwerte)."},
    ]
    return asset


if __name__ == "__main__":
    print(tilt_root(), bbox())
