"""🪛 Round-3 root fixes of appliances/ceiling-diffuser outside `volumes`: 2D shapes and decisions of 1:50, 1:100, 1:200 (cones flare down and out, flange heights, plan seam, screws, spigot), outlet anchors, rig length and centre of mass, provenance.

Usage: python3 r3-ceiling-diffuser-roots.py [--candidate] — writes the candidate to 🗑️generated/r3-ceiling-diffuser/🔣️.json, or (default) re-reads and rewrites the catalog file,
replacing only the changed members with the formatter of volumes-insert.py so member order and style stay.
"""
import json
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
R = T.parents[6]
CATALOG = R / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/ceiling-diffuser/🔣️.json"
CANDIDATE = T / "🗑️generated/r3-ceiling-diffuser/🔣️.json"
source = (T / "volumes-insert.py").read_text(encoding="utf-8").replace("\nmain()\n", "\n")
vi = {}
exec(compile(source, "volumes-insert.py", "exec"), vi)


def text(en, de):
    return {"en": en, "de": de}


def mirror(half):
    right = [[x, y] for x, y in half]
    left = [[-x, y] for x, y in reversed(half)]
    return left + right


def poly(shape, half):
    shape = dict(shape)
    shape["points"] = mirror(half)
    return shape


def section_shapes(shapes, flange_h, cones, band, slots):
    out = []
    for s in shapes:
        s = dict(s)
        if s["id"] == "collar":
            s.update({"x": -0.1245, "w": 0.249})
        elif s["id"] == "flange":
            s.update({"y": -flange_h, "h": flange_h})
        elif s["id"] in cones:
            s = poly(s, cones[s["id"]])
        elif s["id"] == "slot-left":
            s["points"] = [[-0.25, -0.0246], [-0.17, -0.0179]]
        elif s["id"] == "slot-right":
            s["points"] = [[0.25, -0.0246], [0.17, -0.0179]]
        out.append(s)
    if band and not any(s["id"] == "collar-band" for s in out):
        at = next(i for i, s in enumerate(out) if s["id"] == "collar") + 1
        out.insert(at, {"id": "collar-band", "bone": "body", "role": "detail", "z": 2.95, "filled": False, "tone": 0.3, "kind": "poly",
                        "points": [[-0.1245, 0.026], [0.1245, 0.026]], "closed": False, "region": "collar", "material": "galvanised-steel"})
    if band and not any(s["id"] == "backplate" for s in out):
        at = next(i for i, s in enumerate(out) if s["id"] == "collar")
        out.insert(at, {"id": "backplate", "bone": "body", "role": "body", "z": 2.8, "filled": True, "tone": 0.84, "kind": "rect",
                        "x": -0.2625, "y": 0, "w": 0.525, "h": 0.0075, "region": "plate", "material": "powder-coated-steel"})
    for s in out:
        if s["id"] == "collar-band":
            s["points"] = [[-0.1245, 0.026], [0.1245, 0.026]]
    return out


def plan_shapes(shapes, collar_r, opening_r, edge=None, screws=None, bead=None):
    out = []
    for s in shapes:
        s = dict(s)
        if s["id"] == "collar":
            s.update({"rx": collar_r, "ry": collar_r})
        elif s["id"] == "opening":
            s.update({"rx": opening_r, "ry": opening_r})
        elif s["id"] == "collar-bead" and bead:
            s.update({"rx": bead, "ry": bead})
        elif s["id"] == "flange-edge" and edge:
            s["points"] = [[-edge, -edge], [edge, -edge], [edge, edge], [-edge, edge]]
        elif s["id"].startswith("screw-") and screws:
            s.update({"cx": screws if s["cx"] > 0 else -screws, "cy": screws if s["cy"] > 0 else -screws})
        out.append(s)
    return out


CONES = {
    "50": {"cone-1": [[0.135, -0.012], [0.255, -0.022], [0.265, -0.032]],
           "cone-2": [[0.06, -0.032], [0.19, -0.04], [0.2, -0.05]],
           "cone-3": [[0.071, -0.05], [0.123, -0.058], [0.135, -0.07]]},
    "100": {"cone-1": [[0.19, -0.015], [0.247, -0.03], [0.262, -0.045]],
            "cone-2": [[0.11, -0.045], [0.16, -0.06], [0.17, -0.07]]},
    "200": {"cone": [[0.262, -0.03], [0.135, -0.07]]},
}


DECISIONS = {
    "50": {
        "silhouette": text("A 0.595 m square four-way multi-cone ceiling diffuser in side view, 11.9 mm wide and 2.4 mm high on paper: a 12 mm border frame flush with the ceiling with the back plate of the plenum in the ceiling cutout behind it, below it three nested square cones, each flaring down and out to its lip and each lower one narrower, with a dark slot under the frame and under every lip where the cone sheet rises inwards, and the round spigot with its clamp bead above the ceiling. Front: the same view, because the diffuser is square. Plan, 11.9 mm square: the back of the diffuser with the seam between border frame and back plate, the four ceiling screws, the spigot with its bead and dark bore; the cones lie hidden below.",
                           "Ein quadratischer Vierwege-Mehrkegel-Deckenauslass von 0,595 m in der Seitenansicht, auf Papier 11,9 mm breit und 2,4 mm hoch: ein 12 mm hoher Randrahmen bündig mit der Decke mit der Rückplatte des Plenums im Deckenausschnitt dahinter, darunter drei ineinanderliegende quadratische Kegel, die sich jeweils nach unten und außen zur Lippe aufweiten, jeder tiefere schmaler, mit einem dunklen Schlitz unter dem Rahmen und unter jeder Lippe, wo das Kegelblech nach innen ansteigt, und über der Decke der runde Stutzen mit seiner Spannsicke. Vorderansicht: dieselbe Ansicht, weil der Auslass quadratisch ist. Grundriss, 11,9 mm im Quadrat: die Rückseite des Auslasses mit der Fuge zwischen Randrahmen und Rückplatte, die vier Deckenschrauben, der Stutzen mit Sicke und dunkler Öffnung; die Kegel liegen verdeckt darunter."),
        "retained": text("Border frame of 12 mm and 35.5 mm width, three cones with 10, 10 and 12 mm rims at 45 degrees (lips 0.53, 0.40 and 0.27 m wide) and the slots of 10, 8 and 8 mm between them, the 249 mm spigot of 50 mm height with its bead, four screws, and one slot line on each side: the shadow of the outer slot on the first cone, where the air leaves.",
                         "Randrahmen von 12 mm Höhe und 35,5 mm Breite, drei Kegel mit 45-Grad-Rändern von 10, 10 und 12 mm (Lippen 0,53, 0,40 und 0,27 m breit) und dazwischen die Schlitze von 10, 8 und 8 mm, der 249-mm-Stutzen von 50 mm Höhe mit seiner Sicke, vier Schrauben und je eine Schlitzlinie auf jeder Seite: der Schatten des äußeren Schlitzes auf dem ersten Kegel, wo die Luft austritt."),
        "merged": text("The 0.8 mm back plate of the plenum is drawn at the 7.5 mm smallest feature as a band above the ceiling line, in the ceiling cutout; the 8 mm corner spacers sit hidden in the slots; the 0.4 mm gap between the inner edge of the second cone and the first cone is closed in the drawing (under the pen).",
                       "Die 0,8-mm-Rückplatte des Plenums ist mit dem kleinsten Merkmal von 7,5 mm als Band über der Deckenlinie gezeichnet, im Deckenausschnitt; die 8-mm-Eckabstandshalter sitzen verdeckt in den Schlitzen; der Spalt von 0,4 mm zwischen der Innenkante des zweiten Kegels und dem ersten Kegel ist in der Zeichnung geschlossen (unter der Stiftbreite)."),
        "omitted": text("The neck damper and the flexible duct over the spigot. In the plan the cones, the slots and the spacers are hidden under the back plate.",
                        "Die Drosselklappe im Hals und der flexible Kanal über dem Stutzen. Im Grundriss sind Kegel, Schlitze und Abstandshalter von der Rückplatte verdeckt."),
        "thinElements": text("The frame is 12 mm (0.24 mm), the cone rims 10 to 12 mm (0.2 to 0.24 mm); the slot and bead lines are 0.13 mm detail pens.",
                             "Der Rahmen ist 12 mm (0,24 mm) hoch, die Kegelränder 10 bis 12 mm (0,2 bis 0,24 mm); die Schlitz- und Sickenlinien sind Linien mit dem 0,13-mm-Detailstift."),
    },
    "100": {
        "silhouette": text("A 6 mm wide and 1.2 mm high diffuser: a 15 mm flange, two stepped cones flaring down and out to their lips with a slot notch above each, and the short spigot. Front: the same view, because the diffuser is square. Plan, 6 mm square: the flange as one square with the spigot ring and its dark bore.",
                           "Ein Auslass von 6 mm Breite und 1,2 mm Höhe: ein Flansch von 15 mm, zwei Kegelstufen, die sich nach unten und außen zur Lippe aufweiten, mit einer Schlitzkerbe über jeder, und der kurze Stutzen. Vorderansicht: dieselbe Ansicht, weil der Auslass quadratisch ist. Grundriss, 6 mm im Quadrat: der Flansch als ein Quadrat mit dem Stutzenring und seiner dunklen Öffnung."),
        "retained": text("Flange, two cones (three merged into two) with their slot notches, spigot.", "Flansch, zwei Kegelstufen (aus drei zusammengeführt) mit ihren Schlitzkerben, Stutzen."),
        "merged": text("The three cones of 1:50 become two; frame and back plate become one flange.", "Die drei Kegel von 1:50 werden zu zwei; Rahmen und Rückplatte werden zu einem Flansch."),
        "thinElements": text("The 12 mm frame is drawn at 15 mm (0.15 mm) so that it prints as a filled shape.", "Der 12-mm-Rahmen ist mit 15 mm (0,15 mm) gezeichnet, damit er als Fläche druckt."),
    },
    "200": {
        "silhouette": text("A 3 mm wide and 0.6 mm high diffuser: a flange with one tapering cone mass below and the spigot above. Front: flange, cone and spigot. Plan, 3 mm square: a flange square with one spigot disc.",
                           "Ein Auslass von 3 mm Breite und 0,6 mm Höhe: ein Flansch mit einer sich verjüngenden Kegelmasse darunter und dem Stutzen darüber. Vorderansicht: Flansch, Kegel und Stutzen. Grundriss, 3 mm im Quadrat: ein Flanschquadrat mit einer Stutzenscheibe."),
        "merged": text("The three cones become the trapezoid of their envelope, from the frame's inner edge to the centre cone.", "Die drei Kegel werden zum Trapez ihrer Hülle, von der Innenkante des Rahmens bis zum Mittelkegel."),
        "thinElements": text("The 12 mm frame is drawn at 30 mm (0.15 mm), the smallest feature of this scale.", "Der 12-mm-Rahmen ist mit 30 mm (0,15 mm) gezeichnet, dem kleinsten Merkmal dieses Maßstabs."),
    },
}


def build(asset):
    reps = asset["representations"]
    cones = CONES
    for scale, flange_h in (("50", 0.012), ("100", 0.015), ("200", 0.03)):
        g = reps[scale]["geometry"]
        for view in ("elevation", "front"):
            g[view] = section_shapes(g[view], flange_h, cones[scale], scale == "50", scale == "50")
        if scale == "50":
            g["plan"] = plan_shapes(g["plan"], 0.1272, 0.1238, edge=0.262, screws=0.28, bead=0.1245)
        elif scale == "100":
            g["plan"] = plan_shapes(g["plan"], 0.1245, 0.1238)
        else:
            g["plan"] = plan_shapes(g["plan"], 0.1245, 0.1238)
        reps[scale]["decisions"].update(DECISIONS[scale])
    for a in asset["anchors"]:
        if a["id"] == "outlet-left":
            a.update({"x": -0.2585, "y": -0.017})
        elif a["id"] == "outlet-right":
            a.update({"x": 0.2585, "y": -0.017})
    bone = asset["rig"]["bones"][0]
    bone["lengthM"] = 0.2975
    bone["mass"]["comY"] = -0.02
    prov = asset["provenance"]
    known = {r["id"] for r in prov["references"]}
    for ref in (
        {"id": "titus-tms-submittal-2012", "citation": "Titus, TMS Round Neck Square Ceiling Diffusers, Steel, submittal D-TMS-1.0 (1-4-12): 24 x 24 module, neck 10 to 15 in., neck height 1 3/8 in., overall depth 5 1/8 in., cone drop 1 1/4 in., nested cones flaring to their lips.", "url": "https://www.titus-hvac.com/file/287/TMS.pdf"},
        {"id": "koolair-50-fr-4-deco", "citation": "Koolair, 50-FR-4-DECO square four-way supply diffuser for 600 x 600 ceilings: face 595 x 595 mm, 35 mm frame profiles, selection table 600 to 1100 m3/h.", "url": "https://koolair.com/wp-content/pdf/guia/50-FR-4-DECO_en.pdf"},
        {"id": "cas-aqn-4-way", "citation": "Complete Air Supplies, AQN four-way diffuser with removable core, 600 x 600: frame flange, spigot plenum PDZ-1 with 80 mm neck, fixing by screws.", "url": "https://cdn.shopify.com/s/files/1/0321/3613/4796/files/CAS_AQN.pdf"},
    ):
        if ref["id"] not in known:
            prov["references"].append(ref)
    prov["notes"] = text(
        "A surface-mounted square four-way multi-cone diffuser of nominal size 600 (Titus TMS class, steel): a border frame 0.595 m square, 35.5 mm wide and 12 mm deep, screwed to the ceiling at its four corners; a back plate closing the plenum, set into the 0.525 m ceiling cutout; the round DN 250 spigot (249 mm, 0.7 mm galvanised, 50 mm high, swaged bead for the clamp band of the flexible duct) through the ceiling; and three nested square cones of 0.8 mm powder-coated sheet below, each with a 45 degree rim flaring down and out to its lip (0.53, 0.40 and 0.27 m wide, 32, 50 and 70 mm below the ceiling) and a shallow cone rising inwards, carried by corner spacers. The air leaves through the slots between frame and cones, is turned outwards under each lip and clings to the ceiling. The origin is the centre of the back plate at the ceiling plane. At level 1 it supplies 0.083 m3/s (300 m3/h) of air 8 K cooler than a 26 degC room, that is 18 degC, in four directions; the side view shows two of them (left and right) as 0.021 m3/s each, leaving at the mouth of the outer slot. The mass of 3.2 kg is rounded from this product class (authored estimate); the centre of mass lies 20 mm below the ceiling (sheet areas of the 1:50 model).",
        "Ein aufgesetzter quadratischer Vierwege-Mehrkegelauslass der Nennweite 600 (Klasse Titus TMS, Stahl): ein Randrahmen von 0,595 m im Quadrat, 35,5 mm breit und 12 mm tief, an seinen vier Ecken an die Decke geschraubt; eine Rückplatte, die den Plenumraum schließt und im Deckenausschnitt von 0,525 m sitzt; der runde DN-250-Stutzen (249 mm, 0,7 mm verzinkt, 50 mm hoch, eingedrückte Sicke für das Spannband des flexiblen Kanals) durch die Decke; und darunter drei ineinanderliegende quadratische Kegel aus 0,8 mm pulverbeschichtetem Blech, jeder mit einem 45-Grad-Rand, der sich nach unten und außen zur Lippe aufweitet (0,53, 0,40 und 0,27 m breit, 32, 50 und 70 mm unter der Decke), und einem flachen, nach innen ansteigenden Kegel, getragen von Eckabstandshaltern. Die Luft tritt durch die Schlitze zwischen Rahmen und Kegeln aus, wird unter jeder Lippe nach außen gelenkt und legt sich an die Decke an. Der Ursprung ist die Mitte der Rückplatte in der Deckenebene. Auf Stufe 1 bläst er 0,083 m3/s (300 m3/h) Luft ein, die 8 K kühler ist als ein Raum von 26 °C, also 18 °C, in vier Richtungen; die Seitenansicht zeigt zwei davon (links und rechts) mit je 0,021 m3/s, die an der Mündung des äußeren Schlitzes austreten. Die Masse von 3,2 kg ist aus dieser Produktklasse gerundet (Schätzwert); der Schwerpunkt liegt 20 mm unter der Decke (Blechflächen des 1:50-Modells).")
    return asset


def main():
    text_before = CATALOG.read_text(encoding="utf-8")
    document = json.loads(text_before)
    asset = build(json.loads(json.dumps(document["asset"])))
    if "--candidate" in sys.argv:
        out = json.loads(text_before)
        out["asset"] = asset
        src = json.loads((T / "volumes/appliances/ceiling-diffuser.json").read_text(encoding="utf-8"))["volumes"]
        out["asset"]["volumes"] = src
        CANDIDATE.parent.mkdir(parents=True, exist_ok=True)
        CANDIDATE.write_text(json.dumps(out, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        print("[DEBUG] candidate", CANDIDATE)
        return
    indent = "    "
    result = text_before
    for key in ("provenance", "rig", "anchors", "representations"):
        if asset[key] == document["asset"][key]:
            continue
        start, end = vi["member_range"](result, indent, key)
        result = result[:start] + indent + '"%s": ' % key + vi["dump"](asset[key], 2) + result[end:]
    check = json.loads(result)
    assert check["asset"] == asset, "round trip"
    assert list(check["asset"]) == list(document["asset"]), "member order"
    if CATALOG.read_text(encoding="utf-8") != text_before:
        raise SystemExit("catalog changed while editing; retry")
    CATALOG.write_text(result, encoding="utf-8")
    print("[DEBUG] catalog written")


main()
