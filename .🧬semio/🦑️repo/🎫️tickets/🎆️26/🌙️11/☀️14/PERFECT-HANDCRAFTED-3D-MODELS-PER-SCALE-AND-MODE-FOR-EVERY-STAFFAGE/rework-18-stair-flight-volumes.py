"""🏗️ Rework 18 stair flight (balustrades on both open sides added): every solid of every scale and list, stated part by part from the kit geometry; writes volumes/components/stair-flight.json."""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "rework-18-stair-flight-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
SRC = os.path.join(T, "volumes", "components", "stair-flight.json")
text = K.text
STEEL, TIMBER = "steel", "timber"
TREAD_FACES = ["-v", "+u", "-u", "-w", "+w"]


def hea(tf, tw, foot_y, head_x, prefix="hea", sheet=False):
    """🔩 The HEA 160 as top flange, web and bottom flange, each cut horizontally at the foot and plumb at the head."""
    d = K.flange_drops(tf=tf)
    return [
        K.prism(prefix + "-top-flange", STEEL, 0.5, K.band(d[0], d[1], foot_y, head_x), -K.HEA_B / 2, K.HEA_B / 2),
        K.prism(prefix + "-web", STEEL, 0.45, K.band(d[1], d[2], foot_y, head_x), -tw / 2, tw / 2, **({"sheet": True} if sheet else {})),
        K.prism(prefix + "-bottom-flange", STEEL, 0.5, K.band(d[2], d[3], foot_y, head_x), -K.HEA_B / 2, K.HEA_B / 2),
    ]


def fillets():
    return [K.root_fillet("hea-root-%s-%s" % (a, b), STEEL, 0.47, a == "top", b == "l") for a in ("top", "bottom") for b in ("r", "l")]


def treads(rounded, t=K.T_TREAD, i="tread"):
    s = K.prism(i, TIMBER, 0.7, K.tread_outline(1, rounded=rounded, t=t), -K.HALF_W, K.HALF_W)
    s["repeat"] = K.step(K.N)
    return s


def guards():
    """🧒 The 16 guard battens under the nosings (30 x 30 mm timber over the full width)."""
    s = K.prism("riser-guard", TIMBER, 0.65, K.guard_outline(1), -K.HALF_W, K.HALF_W)
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
    """⚖️ The mass of stringer, brackets and treads: a sphere hung from their centre of mass (84 mm above the stringer axis, in the open zone over the top flange) into the stringer, so its top point is that centre."""
    u, v = K.masses()["flightComLocal"]
    return K.st(K.ellipsoid("mass-flight", STEEL, 0.1, (r, r, r), K.world((u, v - r))), part="mass")


def mass_all(r):
    """⚖️ 1:500: one mass for flight and balustrades at their common centre of mass, which lies in the saw-tooth of the steps over the stringer."""
    return K.st(K.ellipsoid("mass-flight", STEEL, 0.1, (r, r, r), K.masses()["com"]), part="mass")


def rail_mass(r):
    """⚖️ The mass of the two balustrades at their common centre of mass, which lies on the middle baluster of the middle bay in the side view."""
    return K.st(K.ellipsoid("mass-balustrade", STEEL, 0.1, (r, r, r), K.masses()["balustradeCom"]), part="mass")


def balustrade(flanges, infill, tube=K.RAIL_R, low=K.LOW_R, bal=K.BAL_R, every=1, bars=True):
    """🚧 Both balustrades: per side six posts CHS 42.4 standing on the treads 50 mm inside their ends (1:50 with a screwed 80 mm flange), the handrail CHS 42.4 parallel to the nosing line with its top 0.95 m above it, and as infill a bottom rail CHS 21.3 at 0.10 m and 35 balusters of 12 mm at 105 mm centres."""
    out = []
    x0, x1 = K.rail_span()
    a, b = K.post_x(K.POST_KS[0]), K.post_x(K.POST_KS[-1])
    for side, z in (("r", -K.RAIL_Z), ("l", K.RAIL_Z)):
        k = K.POST_KS[0]
        lo = K.FLANGE_T if flanges else 0.0
        post = K.rod("post-" + side, STEEL, 0.3, (a, K.tread_y(k) + lo, z), (a, K.rail_y(a), z), tube)
        post["repeat"] = K.step(len(K.POST_KS), K.POST_STEP * K.GOING, K.POST_STEP * K.RISE)
        out.append(post)
        if flanges:
            fl = K.rod("post-flange-" + side, STEEL, 0.25, (a, K.tread_y(k), z), (a, K.tread_y(k) + K.FLANGE_T, z), K.FLANGE_R)
            fl["repeat"] = K.step(len(K.POST_KS), K.POST_STEP * K.GOING, K.POST_STEP * K.RISE)
            out.append(fl)
        out.append(K.rod("handrail-" + side, STEEL, 0.3, (x0, K.rail_y(x0), z), (x1, K.rail_y(x1), z), tube))
        if infill:
            out.append(K.rod("bottom-rail-" + side, STEEL, 0.3, (a, K.rail_y(a, K.LOW_H), z), (b, K.rail_y(b, K.LOW_H), z), low))
            for bay in range(len(K.POST_KS) - 1 if bars else 0):
                x = K.baluster_x(bay, every)
                rod = K.rod("baluster-%s-%d" % (side, bay + 1), STEEL, 0.3, (x, K.rail_y(x, K.LOW_H), z), (x, K.rail_y(x), z), bal)
                rod["repeat"] = K.step(K.BAL_PER_BAY // every, every * K.BAL_STEP, every * K.BAL_STEP * K.M)
                out.append(rod)
    return out


RAIL_IDS = ("post-", "handrail-", "bottom-rail-", "baluster-")


def is_rail(i):
    return i.startswith(RAIL_IDS)


def rail_refs(body):
    return [K.th_of("th-" + s["id"], s["id"], "balustrade-steel") for s in body if is_rail(s["id"])]


def rail_members(body):
    return [K.st_of("st-" + s["id"], s["id"], "joint" if s["id"].startswith("post-flange-") else "member") for s in body if is_rail(s["id"])]


def steel_refs(body):
    out = []
    for s in body:
        if s["material"] != STEEL or is_rail(s["id"]):
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
    body = hea(K.HEA_TF, K.HEA_TW, K.BASE_T, K.RUN - K.HEAD_T, sheet=True) + fillets() + plates(K.BASE_T, K.HEAD_T) + anchors() + [carriers()] + fins(K.FIN_T, K.DROP, K.BASE_T) + [treads(True), guards()] + balustrade(True, True)
    thermal = steel_refs(body) + rail_refs(body) + [K.th_of("th-tread", "tread", "tread-top", TREAD_FACES)]
    structure = [K.st_of("st-" + i, i) for i in ("hea-top-flange", "hea-web", "hea-bottom-flange", "hea-root-top-r", "hea-root-top-l", "hea-root-bottom-r", "hea-root-bottom-l")]
    structure += [K.st_of("st-tread", "tread"), K.st_of("st-carrier", "carrier", "joint"), K.st_of("st-fin-01", "fin-01", "joint"), K.st_of("st-fin", "fin", "joint")]
    structure += [K.st_of("support-foot-pin", "base-plate", "support", "foot-pin"), K.st_of("support-landing-roller", "head-plate", "support", "landing-roller"), mass(0.035), rail_mass(0.012)] + rail_members(body)
    decisions = [
        text("1:50 is the flight as built: an HEA 160 centre stringer (h 152, b 160, flanges 9 mm, web 6 mm, root radii 15 mm) as top flange, web, bottom flange and the four root fillets, cut horizontally where it stands on a 300 x 260 x 15 base plate and plumb where it meets a 15 mm head plate on the landing face; four M12 anchors with hexagon nuts on each plate; under each of the 16 treads a welded bracket of a 10 mm fin and a 170 x 360 x 8 carrier plate; 16 C24 treads of 310 x 1200 x 40 mm with 8 mm rounded nosing edges, lapping 30 mm; the risers are open, and a 30 x 30 mm timber guard batten screwed under each tread 30 mm behind its nosing narrows the opening from 141 to 111 mm (at most 120 mm where children are expected).",
             "1:50 ist der Lauf wie gebaut: eine Mittelwange HEA 160 (h 152, b 160, Flansche 9 mm, Steg 6 mm, Ausrundungen 15 mm) aus Obergurt, Steg, Untergurt und den vier Ausrundungen, waagrecht geschnitten, wo sie auf einer Fußplatte 300 x 260 x 15 steht, und lotrecht, wo sie an eine 15-mm-Kopfplatte an der Podeststirn stößt; je vier Anker M12 mit Sechskantmuttern an beiden Platten; unter jeder der 16 Stufen eine angeschweißte Konsole aus 10-mm-Steg und Auflagerplatte 170 x 360 x 8; 16 Stufen aus C24, 310 x 1200 x 40 mm, mit 8 mm gerundeten Vorderkanten und 30 mm Unterschneidung; die Setzstufen sind offen, eine unter jede Stufe 30 mm hinter der Vorderkante geschraubte Holzleiste 30 x 30 mm verengt die Öffnung von 141 auf 111 mm (höchstens 120 mm, wo mit Kindern zu rechnen ist)."),
        text("Thermal: stringer-steel is every steel solid by reference; the carrier top lies against the tread, the base plate underside on the floor slab and the head plate back on the landing face, those faces are marked as section faces (C25 b). tread-top is the tread by reference with every face except the walking surface marked, as ruled for this region (area-decisions).",
             "Thermik: Wangenstahl ist jeder Stahlkörper per Verweis; die Oberseite der Auflagerplatte liegt an der Stufe, die Unterseite der Fußplatte auf der Bodenplatte und die Rückseite der Kopfplatte an der Podeststirn, diese Flächen sind als Schnittflächen markiert (C25 b). Stufenoberseite ist die Stufe per Verweis, alle Flächen außer der Trittfläche markiert, wie für diese Region entschieden (area-decisions)."),
        text("Structure: the HEA flanges and web are the member along the centroid axis, the treads carry the load into the brackets, which are the joints; the base plate is the pin at the foot, the head plate with horizontal slotted holes the roller at the landing; the mass of the whole flight is a 70 mm sphere whose top point is the centre of mass (84 mm above the stringer axis, just over the top flange).",
             "Tragwerk: Flansche und Steg der HEA sind das Bauteil entlang der Schwerachse, die Stufen tragen die Last in die Konsolen, die die Knoten sind; die Fußplatte ist das Festlager am Fuß, die Kopfplatte mit waagrechten Langlöchern das Rollenlager am Podest; die Masse des ganzen Laufs ist eine 70-mm-Kugel, deren oberster Punkt der Schwerpunkt ist (84 mm über der Wangenachse, knapp über dem Obergurt)."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    body = hea(0.015, 0.015, K.BASE_T, K.RUN - K.HEAD_T) + plates(K.BASE_T, K.HEAD_T) + fins(0.015, K.T_TREAD, K.BASE_T) + [treads(False), guards()] + balustrade(False, True, bal=0.0075)
    thermal = steel_refs(body) + rail_refs(body) + [K.th_of("th-tread", "tread", "tread-top", TREAD_FACES)]
    structure = [K.st_of("st-" + i, i) for i in ("hea-top-flange", "hea-web", "hea-bottom-flange")]
    structure += [K.st_of("st-tread", "tread"), K.st_of("st-fin-01", "fin-01", "joint"), K.st_of("st-fin", "fin", "joint")]
    structure += [K.st_of("support-foot-pin", "base-plate", "support", "foot-pin"), K.st_of("support-landing-roller", "head-plate", "support", "landing-roller"), mass(0.045), rail_mass(0.02)] + rail_members(body)
    decisions = [
        text("1:100 keeps the I of the HEA 160 (the 77 mm recess between the flanges is five features deep) with flanges and web at the 15 mm feature, the base plate and head plate at their true 15 mm, the 16 brackets as one 15 mm fin each that reaches the tread (the 8 mm carrier plate is below the feature and merged into the fin), and 16 plain 40 mm treads, each with its 30 x 30 mm guard batten (two features) under the nosing; the anchors and the nosing radius are left out.",
             "1:100 behält das I der HEA 160 (die 77 mm tiefe Kammer zwischen den Flanschen ist fünf Merkmale tief) mit Flanschen und Steg im Merkmal 15 mm, Fuß- und Kopfplatte in ihren wahren 15 mm, die 16 Konsolen als je einen 15-mm-Steg, der bis zur Stufe reicht (die 8-mm-Auflagerplatte liegt unter dem Merkmal und geht im Steg auf), und 16 glatte 40-mm-Stufen, jede mit ihrer Kinderschutzleiste 30 x 30 mm (zwei Merkmale) unter der Vorderkante; die Anker und die Kantenrundung entfallen."),
        text("Thermal: every steel solid by reference with the contact faces of plates marked (C25 b), the treads with only the walking surface counted.",
             "Thermik: jeder Stahlkörper per Verweis mit markierten Kontaktflächen der Platten (C25 b), die Stufen nur mit der Trittfläche gezählt."),
        text("Structure: flanges and web as the member, treads as members, the fins as joints, the base plate as pin and the head plate as roller, one mass at the centre of mass.",
             "Tragwerk: Flansche und Steg als Bauteil, Stufen als Bauteile, die Stege als Knoten, Fußplatte als Festlager und Kopfplatte als Rollenlager, eine Masse im Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    d = K.flange_drops()
    beam = K.prism("stringer", STEEL, 0.45, K.band(d[0], d[3], 0.03, K.RUN - 0.03), -K.HEA_B / 2, K.HEA_B / 2)
    body = [beam] + plates(0.03, 0.03) + fins(0.03, K.T_TREAD, 0.03) + [treads(False), guards()] + balustrade(False, True, low=0.015, bal=0.015)
    thermal = steel_refs(body) + rail_refs(body) + [K.th_of("th-tread", "tread", "tread-top", TREAD_FACES)]
    structure = [K.st_of("st-stringer", "stringer"), K.st_of("st-tread", "tread"), K.st_of("st-fin-01", "fin-01", "joint"), K.st_of("st-fin", "fin", "joint")]
    structure += [K.st_of("support-foot-pin", "base-plate", "support", "foot-pin"), K.st_of("support-landing-roller", "head-plate", "support", "landing-roller"), mass(0.06), rail_mass(0.03)] + rail_members(body)
    decisions = [
        text("1:200 merges flanges and web into one 152 x 160 stringer bar (the I recess reads only in the end view, which the plates cover), keeps the base plate and head plate generalised to the 30 mm feature, one 30 mm fin per tread up to the tread and 16 plain 40 mm treads with their 30 mm lap and the open risers, narrowed to 111 mm by a guard batten of the 30 mm feature under each nosing.",
             "1:200 fasst Flansche und Steg zu einem Wangenbalken 152 x 160 zusammen (die I-Kammer ist nur in der Stirnansicht lesbar, die die Platten verdecken), behält Fuß- und Kopfplatte, auf das Merkmal 30 mm verallgemeinert, je Stufe einen 30-mm-Steg bis zur Stufe und 16 glatte 40-mm-Stufen mit 30 mm Unterschneidung und offenen Setzstufen, die eine Leiste im Merkmal 30 mm unter jeder Vorderkante auf 111 mm verengt."),
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
    body = [beam, flight] + balustrade(False, True, tube=0.0375, low=0.0375, bars=False)
    thermal = [K.th_of("th-stringer", "stringer", "stringer-steel"), K.th_of("th-flight", "flight", "tread-top", TREAD_FACES)] + rail_refs(body)
    foot = K.st(K.prism("support-foot-pin", STEEL, 0.15, K.band(d[0], d[3], 0.0, K.RUN)[:2] + [(K.line_x(0.075, d[3]), 0.075), (K.line_x(0.075, d[0]), 0.075)], -0.0875, 0.0875, role="detail"), part="support", anchor="foot-pin")
    head = K.st(K.prism("support-landing-roller", STEEL, 0.15, [(K.RUN - 0.075, K.line_y(K.RUN - 0.075, d[3])), (K.RUN, K.line_y(K.RUN, d[3])), (K.RUN, K.line_y(K.RUN, d[0])), (K.RUN - 0.075, K.line_y(K.RUN - 0.075, d[0]))], -0.0875, 0.0875, role="detail"), part="support", anchor="landing-roller")
    structure = [K.st_of("st-stringer", "stringer"), K.st_of("st-flight", "flight"), foot, head, mass_all(0.075)] + rail_members(body)
    decisions = [
        text("1:500 is the stringer as one 152 x 160 bar standing on the floor and ending at the landing face (base plate, head plate and brackets merged into it) and the 16 steps as one folded band of the 75 mm feature, 1.2 m wide, resting on it: the open risers (111 mm clear under the guard battens, drawn 106 mm between 75 mm bars) would leave 0.08 mm of white between the pens, which does not print, so they are closed in the drawing and in the model alike; the 16 steps of 0.28 x 0.181 m still read as a saw-tooth.",
             "1:500 ist die Wange als ein Balken 152 x 160, der auf dem Boden steht und an der Podeststirn endet (Fußplatte, Kopfplatte und Konsolen gehen in ihm auf), und die 16 Stufen als ein gefaltetes Band im Merkmal 75 mm, 1,2 m breit, auf ihr aufliegend: die offenen Setzstufen (111 mm licht unter den Kinderschutzleisten, zwischen 75-mm-Balken 106 mm) ließen 0,08 mm Weiß zwischen den Stiften, das nicht druckt, sie sind daher in Zeichnung und Modell geschlossen; die 16 Stufen von 0,28 x 0,181 m lesen sich weiter als Sägezahn."),
        text("Thermal: the bar and the folded band by reference, the band with only its walking surfaces counted; only the total is compared at this scale (C15).",
             "Thermik: Balken und gefaltetes Band per Verweis, das Band nur mit seinen Trittflächen gezählt; in diesem Maßstab wird nur die Summe verglichen (C15)."),
        text("Structure: the bar as the member and the band as the treads that load it; the supports are the 75 mm end zones of the bar where it bears on the floor (pin) and on the landing face (roller); posts and rails are members, and flight and balustrades share one mass at their common centre of mass (%d mm above the stringer axis, inside the saw-tooth of the steps)." % round(K.masses()["comLocal"][1] * 1000),
             "Tragwerk: der Balken als Bauteil und das Band als die Stufen, die ihn belasten; die Lager sind die 75-mm-Endzonen des Balkens, wo er auf dem Boden (Festlager) und an der Podeststirn (Rollenlager) aufliegt; Pfosten und Gurte sind Bauteile, Lauf und Geländer teilen sich eine Masse in ihrem gemeinsamen Schwerpunkt (%d mm über der Wangenachse, im Sägezahn der Stufen)." % round(K.masses()["comLocal"][1] * 1000)),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


RAILS = {
    "50": ("Both open sides carry a balustrade (MBO section 38, DIN 18065): six posts CHS 42.4 x 2.6 standing on treads 1, 4, 7, 10, 13 and 16, 50 mm inside the tread ends, each on a screwed 80 x 8 mm flange; a CHS 42.4 handrail parallel to the nosing line with its top 0.95 m above it, running 60 mm past the end posts; a CHS 21.3 bottom rail 0.10 m above the nosing line and 35 balusters of 12 mm at 105 mm centres (93 mm clear) between the two rails. Thermal: these parts are the region balustrade-steel. Structure: posts, rails and balusters are members with the post flanges as joints, and their mass is its own point at the centre of mass of both balustrades, which lies on the middle baluster in the side view.",
           "Beide offenen Seiten tragen ein Geländer (MBO § 38, DIN 18065): sechs Pfosten CHS 42,4 x 2,6 auf den Stufen 1, 4, 7, 10, 13 und 16, 50 mm innerhalb der Stufenenden, je auf einem geschraubten Flansch 80 x 8 mm; ein Handlauf CHS 42,4 parallel zur Stufenvorderkantenlinie mit Oberkante 0,95 m darüber, 60 mm über die Endpfosten hinaus; ein Untergurt CHS 21,3 0,10 m über der Vorderkantenlinie und 35 Stäbe von 12 mm im Abstand von 105 mm (93 mm licht) zwischen den beiden Gurten. Thermik: diese Teile sind die Region balustrade-steel. Tragwerk: Pfosten, Gurte und Stäbe sind Bauteile mit den Pfostenflanschen als Knoten, ihre Masse ist ein eigener Punkt im Schwerpunkt beider Geländer, der in der Seitenansicht auf dem mittleren Stab liegt."),
    "100": ("The two balustrades keep posts, handrail and bottom rail at their true diameters and all balusters as rods of the 15 mm feature (real 12 mm); the 8 mm post flanges are below the feature and left out.",
            "Die zwei Geländer behalten Pfosten, Handlauf und Untergurt in wahrem Durchmesser und alle Stäbe als Stäbe im Merkmal 15 mm (wahr 12 mm); die 8-mm-Pfostenflansche liegen unter dem Merkmal und entfallen."),
    "200": ("The two balustrades keep posts and handrail at their true 42.4 mm; the bottom rail and all balusters are bars of the 30 mm feature (real 21.3 and 12 mm; 75 mm clear between balusters, 0.38 mm on paper); thinning them to every second bar was tried and rejected, because the side view then departs from 1:100 by 90 mm and from 1:500 by 188 mm (volumes-cross-scale).",
            "Die zwei Geländer behalten Pfosten und Handlauf in ihren wahren 42,4 mm; Untergurt und alle Stäbe sind Stäbe im Merkmal 30 mm (wahr 21,3 und 12 mm; 75 mm licht zwischen den Stäben, 0,38 mm auf dem Papier); jeden zweiten Stab wegzulassen wurde versucht und verworfen, weil die Seitenansicht dann um 90 mm von 1:100 und um 188 mm von 1:500 abweicht (volumes-cross-scale)."),
    "500": ("The two balustrades are modelled at the 75 mm feature: the six posts, the handrail (real 42.4 mm) and the bottom rail; the 35 balusters of 12 mm (0.024 mm on paper, 93 mm clear = 0.19 mm) are left out, at the feature they would close into a wall that the real balustrade is not.",
            "Die zwei Geländer sind im Merkmal 75 mm modelliert: die sechs Pfosten, der Handlauf (wahr 42,4 mm) und der Untergurt; die 35 Stäbe von 12 mm (0,024 mm auf dem Papier, 93 mm licht = 0,19 mm) entfallen, im Merkmal würden sie sich zu einer Wand schließen, die das wirkliche Geländer nicht ist."),
}


def main():
    vols = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
    for sc, v in vols.items():
        v["decisions"][0] = text(v["decisions"][0]["en"] + " " + RAILS[sc][0], v["decisions"][0]["de"] + " " + RAILS[sc][1])
    with open(SRC, "w", encoding="utf-8") as f:
        json.dump({"volumes": vols}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("[DEBUG] wrote", SRC, {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in vols.items()})


if __name__ == "__main__":
    main()
