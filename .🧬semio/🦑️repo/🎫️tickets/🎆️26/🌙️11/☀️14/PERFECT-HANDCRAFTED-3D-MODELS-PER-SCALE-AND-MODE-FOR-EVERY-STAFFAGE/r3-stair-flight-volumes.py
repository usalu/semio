"""🏗️ Round 3 stair flight: every solid of every scale and list, stated part by part from the kit geometry; writes volumes/components/stair-flight.json."""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-stair-flight-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
SRC = os.path.join(T, "volumes", "components", "stair-flight.json")
text = K.text
STEEL, TIMBER = "steel", "timber"
TREAD_FACES = ["-v", "+u", "-u", "-w", "+w"]


def hea(tf, tw, foot_y, head_x, prefix="hea"):
    """🔩 The HEA 160 as top flange, web and bottom flange, each cut horizontally at the foot and plumb at the head."""
    d = K.flange_drops(tf=tf)
    return [
        K.prism(prefix + "-top-flange", STEEL, 0.5, K.band(d[0], d[1], foot_y, head_x), -K.HEA_B / 2, K.HEA_B / 2),
        K.prism(prefix + "-web", STEEL, 0.45, K.band(d[1], d[2], foot_y, head_x), -tw / 2, tw / 2),
        K.prism(prefix + "-bottom-flange", STEEL, 0.5, K.band(d[2], d[3], foot_y, head_x), -K.HEA_B / 2, K.HEA_B / 2),
    ]


def treads(rounded, t=K.T_TREAD, i="tread"):
    s = K.prism(i, TIMBER, 0.7, K.tread_outline(1, rounded=rounded, t=t), -K.HALF_W, K.HALF_W)
    s["repeat"] = K.step(K.N)
    return s


def fins(t, top_drop, foot_y):
    first = K.prism("fin-01", STEEL, 0.4, K.fin_outline(1, top_drop=top_drop, foot_y=foot_y), -t / 2, t / 2)
    rest = K.prism("fin", STEEL, 0.4, K.fin_outline(2, top_drop=top_drop, foot_y=foot_y), -t / 2, t / 2)
    rest["repeat"] = K.step(K.N - 1)
    return [first, rest]


def plates(base_t, head_t):
    base = K.prism("base-plate", STEEL, 0.35, K.rect(K.BASE_X[0], 0.0, K.BASE_X[1], base_t), -K.BASE_HALF, K.BASE_HALF)
    head = K.prism("head-plate", STEEL, 0.35, K.rect(K.RUN - head_t, K.HEAD_Y[0], K.RUN, K.HEAD_Y[1]), -K.HEAD_HALF, K.HEAD_HALF)
    return [base, head]


def anchors():
    """🔩 Four M12 anchors of the base plate (nut and bolt end) and four of the head plate, as pairs repeated along x and along y."""
    out = []
    dx = K.ANCHOR_X[1] - K.ANCHOR_X[0]
    dy = K.HEAD_BOLT_Y[1] - K.HEAD_BOLT_Y[0]
    for side, z in (("r", -K.ANCHOR_Z), ("l", K.ANCHOR_Z)):
        nut = K.hex_up("base-nut-" + side, STEEL, 0.25, (K.ANCHOR_X[0], z), K.NUT_AF, K.BASE_T, K.BASE_T + K.NUT_H)
        nut["repeat"] = K.step(2, dx, 0.0)
        bolt = K.rod("base-bolt-" + side, STEEL, 0.2, (K.ANCHOR_X[0], K.BASE_T + K.NUT_H - 0.002, z), (K.ANCHOR_X[0], K.BASE_T + K.NUT_H + K.BOLT_OUT, z), K.BOLT_R)
        bolt["repeat"] = K.step(2, dx, 0.0)
        x_face = K.RUN - K.HEAD_T
        hnut = K.hex_ahead("head-nut-" + side, STEEL, 0.25, (K.HEAD_BOLT_Y[0], z), K.NUT_AF, x_face - K.NUT_H, x_face)
        hnut["repeat"] = K.step(2, 0.0, dy)
        hbolt = K.rod("head-bolt-" + side, STEEL, 0.2, (x_face - K.NUT_H + 0.002, K.HEAD_BOLT_Y[0], z), (x_face - K.NUT_H - K.HEAD_BOLT_OUT, K.HEAD_BOLT_Y[0], z), K.BOLT_R)
        hbolt["repeat"] = K.step(2, 0.0, dy)
        out += [nut, bolt, hnut, hbolt]
    return out


def carriers():
    s = K.prism("carrier", STEEL, 0.45, K.carrier_outline(1), -K.CARRIER_HALF, K.CARRIER_HALF)
    s["repeat"] = K.step(K.N)
    return s


def mass(r):
    """⚖️ The mass of the flight: a sphere hung from the true centre of mass (81 mm above the stringer axis, in the open zone over the top flange) into the stringer, so its top point is the centre of mass."""
    u, v = K.masses()["comLocal"]
    return K.st(K.ellipsoid("mass-flight", STEEL, 0.1, (r, r, r), K.world((u, v - r))), part="mass")


def steel_refs(body):
    out = []
    for s in body:
        if s["material"] != STEEL:
            continue
        faces = None
        if s["id"] == "carrier":
            faces = ["+v"]
        if s["id"] == "base-plate":
            faces = ["-v"]
        if s["id"] == "head-plate":
            faces = ["+u"]
        out.append(K.th_of("th-" + s["id"], s["id"], "stringer-steel", faces))
    return out


def scale50():
    d = K.flange_drops()
    body = hea(K.HEA_TF, 0.0075, K.BASE_T, K.RUN - K.HEAD_T) + plates(K.BASE_T, K.HEAD_T) + anchors() + [carriers()] + fins(K.FIN_T, K.DROP, K.BASE_T) + [treads(True)]
    thermal = steel_refs(body) + [K.th_of("th-tread", "tread", "tread-top", TREAD_FACES)]
    structure = [K.st_of("st-" + i, i) for i in ("hea-top-flange", "hea-web", "hea-bottom-flange")]
    structure += [K.st_of("st-tread", "tread"), K.st_of("st-carrier", "carrier", "joint"), K.st_of("st-fin-01", "fin-01", "joint"), K.st_of("st-fin", "fin", "joint")]
    structure += [K.st_of("support-foot-pin", "base-plate", "support", "foot-pin"), K.st_of("support-landing-roller", "head-plate", "support", "landing-roller"), mass(0.035)]
    decisions = [
        text("1:50 is the flight as built: an HEA 160 centre stringer (h 152, b 160, flanges 9 mm, web 7.5 mm for the real 6 mm, the 15 mm root radii left out) as top flange, web and bottom flange, cut horizontally where it stands on a 300 x 260 x 15 base plate and plumb where it meets a 15 mm head plate on the landing face; four M12 anchors with hexagon nuts on each plate; under each of the 16 treads a welded bracket of a 10 mm fin and a 170 x 360 x 8 carrier plate; 16 C24 treads of 310 x 1200 x 40 mm with 8 mm rounded nosing edges, lapping 30 mm, risers open.",
             "1:50 ist der Lauf wie gebaut: eine Mittelwange HEA 160 (h 152, b 160, Flansche 9 mm, Steg 7,5 mm statt der wirklichen 6 mm, die Ausrundungen 15 mm weggelassen) aus Obergurt, Steg und Untergurt, waagrecht geschnitten, wo sie auf einer Fußplatte 300 x 260 x 15 steht, und lotrecht, wo sie an eine 15-mm-Kopfplatte an der Podeststirn stößt; je vier Anker M12 mit Sechskantmuttern an beiden Platten; unter jeder der 16 Stufen eine angeschweißte Konsole aus 10-mm-Steg und Auflagerplatte 170 x 360 x 8; 16 Stufen aus C24, 310 x 1200 x 40 mm, mit 8 mm gerundeten Vorderkanten und 30 mm Unterschneidung, Setzstufen offen."),
        text("Thermal: stringer-steel is every steel solid by reference; the carrier top lies against the tread, the base plate underside on the floor slab and the head plate back on the landing face, those faces are marked as section faces (C25 b). tread-top is the tread by reference with every face except the walking surface marked, as ruled for this region (area-decisions).",
             "Thermik: Wangenstahl ist jeder Stahlkörper per Verweis; die Oberseite der Auflagerplatte liegt an der Stufe, die Unterseite der Fußplatte auf der Bodenplatte und die Rückseite der Kopfplatte an der Podeststirn, diese Flächen sind als Schnittflächen markiert (C25 b). Stufenoberseite ist die Stufe per Verweis, alle Flächen außer der Trittfläche markiert, wie für diese Region entschieden (area-decisions)."),
        text("Structure: the HEA flanges and web are the member along the centroid axis, the treads carry the load into the brackets, which are the joints; the base plate is the pin at the foot, the head plate with horizontal slotted holes the roller at the landing; the mass of the whole flight is a 70 mm sphere whose top point is the centre of mass (81 mm above the stringer axis, just over the top flange).",
             "Tragwerk: Flansche und Steg der HEA sind das Bauteil entlang der Schwerachse, die Stufen tragen die Last in die Konsolen, die die Knoten sind; die Fußplatte ist das Festlager am Fuß, die Kopfplatte mit waagrechten Langlöchern das Rollenlager am Podest; die Masse des ganzen Laufs ist eine 70-mm-Kugel, deren oberster Punkt der Schwerpunkt ist (81 mm über der Wangenachse, knapp über dem Obergurt)."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    body = hea(0.015, 0.015, K.BASE_T, K.RUN - K.HEAD_T) + plates(K.BASE_T, K.HEAD_T) + fins(0.015, K.T_TREAD, K.BASE_T) + [treads(False)]
    thermal = steel_refs(body) + [K.th_of("th-tread", "tread", "tread-top", TREAD_FACES)]
    structure = [K.st_of("st-" + i, i) for i in ("hea-top-flange", "hea-web", "hea-bottom-flange")]
    structure += [K.st_of("st-tread", "tread"), K.st_of("st-fin-01", "fin-01", "joint"), K.st_of("st-fin", "fin", "joint")]
    structure += [K.st_of("support-foot-pin", "base-plate", "support", "foot-pin"), K.st_of("support-landing-roller", "head-plate", "support", "landing-roller"), mass(0.045)]
    decisions = [
        text("1:100 keeps the I of the HEA 160 (the 77 mm recess between the flanges is five features deep) with flanges and web at the 15 mm feature, the base plate and head plate at their true 15 mm, the 16 brackets as one 15 mm fin each that reaches the tread (the 8 mm carrier plate is below the feature and merged into the fin), and 16 plain 40 mm treads; the anchors and the nosing radius are left out.",
             "1:100 behält das I der HEA 160 (die 77 mm tiefe Kammer zwischen den Flanschen ist fünf Merkmale tief) mit Flanschen und Steg im Merkmal 15 mm, Fuß- und Kopfplatte in ihren wahren 15 mm, die 16 Konsolen als je einen 15-mm-Steg, der bis zur Stufe reicht (die 8-mm-Auflagerplatte liegt unter dem Merkmal und geht im Steg auf), und 16 glatte 40-mm-Stufen; die Anker und die Kantenrundung entfallen."),
        text("Thermal: every steel solid by reference with the contact faces of plates marked (C25 b), the treads with only the walking surface counted.",
             "Thermik: jeder Stahlkörper per Verweis mit markierten Kontaktflächen der Platten (C25 b), die Stufen nur mit der Trittfläche gezählt."),
        text("Structure: flanges and web as the member, treads as members, the fins as joints, the base plate as pin and the head plate as roller, one mass at the centre of mass.",
             "Tragwerk: Flansche und Steg als Bauteil, Stufen als Bauteile, die Stege als Knoten, Fußplatte als Festlager und Kopfplatte als Rollenlager, eine Masse im Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    d = K.flange_drops()
    beam = K.prism("stringer", STEEL, 0.45, K.band(d[0], d[3], 0.03, K.RUN - 0.03), -K.HEA_B / 2, K.HEA_B / 2)
    body = [beam] + plates(0.03, 0.03) + fins(0.03, K.T_TREAD, 0.03) + [treads(False)]
    thermal = steel_refs(body) + [K.th_of("th-tread", "tread", "tread-top", TREAD_FACES)]
    structure = [K.st_of("st-stringer", "stringer"), K.st_of("st-tread", "tread"), K.st_of("st-fin-01", "fin-01", "joint"), K.st_of("st-fin", "fin", "joint")]
    structure += [K.st_of("support-foot-pin", "base-plate", "support", "foot-pin"), K.st_of("support-landing-roller", "head-plate", "support", "landing-roller"), mass(0.06)]
    decisions = [
        text("1:200 merges flanges and web into one 152 x 160 stringer bar (the I recess reads only in the end view, which the plates cover), keeps the base plate and head plate generalised to the 30 mm feature, one 30 mm fin per tread up to the tread and 16 plain 40 mm treads with their 30 mm lap and the open risers (141 mm, almost five features).",
             "1:200 fasst Flansche und Steg zu einem Wangenbalken 152 x 160 zusammen (die I-Kammer ist nur in der Stirnansicht lesbar, die die Platten verdecken), behält Fuß- und Kopfplatte, auf das Merkmal 30 mm verallgemeinert, je Stufe einen 30-mm-Steg bis zur Stufe und 16 glatte 40-mm-Stufen mit 30 mm Unterschneidung und offenen Setzstufen (141 mm, fast fünf Merkmale)."),
        text("Thermal: the bar, plates and fins by reference with the plate contact faces marked, the treads with the walking surface; the bar has no inner flange faces, so the steel area is smaller than at 1:50 (C15 compares the total).",
             "Thermik: Balken, Platten und Stege per Verweis mit markierten Plattenkontaktflächen, die Stufen mit der Trittfläche; der Balken hat keine inneren Flanschflächen, die Stahlfläche ist kleiner als bei 1:50 (C15 vergleicht die Summe)."),
        text("Structure: the bar as the member, treads as members, fins as joints, base plate pin and head plate roller, one mass.",
             "Tragwerk: der Balken als Bauteil, Stufen als Bauteile, Stege als Knoten, Fußplatte als Festlager und Kopfplatte als Rollenlager, eine Masse."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale500():
    d = K.flange_drops()
    beam = K.prism("stringer", STEEL, 0.45, K.band(d[0], d[3], 0.0, K.RUN), -K.HEA_B / 2, K.HEA_B / 2)
    flight = K.prism("flight", TIMBER, 0.7, K.folded_band(0.075), -K.HALF_W, K.HALF_W)
    body = [beam, flight]
    thermal = [K.th_of("th-stringer", "stringer", "stringer-steel"), K.th_of("th-flight", "flight", "tread-top", TREAD_FACES)]
    foot = K.st(K.prism("support-foot-pin", STEEL, 0.15, K.band(d[0], d[3], 0.0, K.RUN)[:2] + [(K.line_x(0.075, d[3]), 0.075), (K.line_x(0.075, d[0]), 0.075)], -0.0875, 0.0875, role="detail"), part="support", anchor="foot-pin")
    head = K.st(K.prism("support-landing-roller", STEEL, 0.15, [(K.RUN - 0.075, K.line_y(K.RUN - 0.075, d[3])), (K.RUN, K.line_y(K.RUN, d[3])), (K.RUN, K.line_y(K.RUN, d[0])), (K.RUN - 0.075, K.line_y(K.RUN - 0.075, d[0]))], -0.0875, 0.0875, role="detail"), part="support", anchor="landing-roller")
    structure = [K.st_of("st-stringer", "stringer"), K.st_of("st-flight", "flight"), foot, head, mass(0.075)]
    decisions = [
        text("1:500 is the stringer as one 152 x 160 bar standing on the floor and ending at the landing face (base plate, head plate and brackets merged into it) and the 16 steps as one folded band of the 75 mm feature, 1.2 m wide, resting on it: the open risers (141 mm, drawn 106 mm between 75 mm bars) would leave 0.08 mm of white between the pens, which does not print, so they are closed in the drawing and in the model alike; the 16 steps of 0.28 x 0.181 m still read as a saw-tooth.",
             "1:500 ist die Wange als ein Balken 152 x 160, der auf dem Boden steht und an der Podeststirn endet (Fußplatte, Kopfplatte und Konsolen gehen in ihm auf), und die 16 Stufen als ein gefaltetes Band im Merkmal 75 mm, 1,2 m breit, auf ihr aufliegend: die offenen Setzstufen (141 mm, zwischen 75-mm-Balken 106 mm) ließen 0,08 mm Weiß zwischen den Stiften, das nicht druckt, sie sind daher in Zeichnung und Modell geschlossen; die 16 Stufen von 0,28 x 0,181 m lesen sich weiter als Sägezahn."),
        text("Thermal: the bar and the folded band by reference, the band with only its walking surfaces counted; only the total is compared at this scale (C15).",
             "Thermik: Balken und gefaltetes Band per Verweis, das Band nur mit seinen Trittflächen gezählt; in diesem Maßstab wird nur die Summe verglichen (C15)."),
        text("Structure: the bar as the member and the band as the treads that load it; the supports are the 75 mm end zones of the bar where it bears on the floor (pin) and on the landing face (roller), one mass at the centre of mass.",
             "Tragwerk: der Balken als Bauteil und das Band als die Stufen, die ihn belasten; die Lager sind die 75-mm-Endzonen des Balkens, wo er auf dem Boden (Festlager) und an der Podeststirn (Rollenlager) aufliegt, eine Masse im Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    vols = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
    with open(SRC, "w", encoding="utf-8") as f:
        json.dump({"volumes": vols}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("[DEBUG] wrote", SRC, {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in vols.items()})


if __name__ == "__main__":
    main()
