"""🏗️ Excavator at 1:100 (smallest feature 15 mm): the machine drawn anew for the scale. The shoe band and its grousers are one 45 mm band on one link chain, wheels are plain discs, the cab is a closed block glazed by four panes, boom and stick keep their forks but lose bosses, pipes, lamps and pins, the bucket is shell, cheeks, lip, teeth and ears, every cylinder is a barrel and a rod. Structure: track frames, carbody ring with two diagonal beams, the revolving frame, ROPS, box girders as flange band and webs, cylinder tubes and rods, a pin at every joint."""
import math
import sys

F = sys.modules["r3_excavator_form"]
L, K = F.L, F.K
C = F.load("r3-excavator-coarse")
PS, TS, WS = C.PS, C.TS, C.WS
CH = 0.004


def arm(S, R):
    bw, sw, kw = F.BOOM_W, F.STICK_W, F.BUCKET_W
    att = dict(material=PS, region="attachment")
    box = L.poly(F.BOOM_BOX).pts(CH)
    S.ext("boom-body", "boom", box, "w", bw[0], bw[1], tone=0.58, **att)
    S.draw("boom-body", "boom", L.Path_of(L.poly(F.BOOM).pts(CH)), "w", bw[0], bw[1], tone=0.58, views="e", **att)
    S.draw("boom-body", "boom", L.Path_of(box), "w", bw[0], bw[1], tone=0.58, views="fp", **att)
    for name, a, b in (("right", bw[0], bw[0] + 0.07), ("left", bw[1] - 0.07, bw[1])):
        S.ext("boom-cheek-" + name, "boom", F.BOOM_CHEEK, "w", a, b, tone=0.58, **att)
        S.draw("boom-cheek-" + name, "boom", L.poly(F.BOOM_CHEEK), "w", a, b, tone=0.58, views="fp", **att)
    S.cyl("boom-root-boss", "boom", (0, 0, bw[0] - 0.01), (0, 0, bw[1] + 0.01), 0.20, tone=0.5, **att)
    S.draw("boom-root-boss", "boom", L.circle(0, 0, 0.20), "w", bw[0] - 0.01, bw[1] + 0.01, role="detail", tone=0.5, material=PS, parent="boom-body", sub=1)
    S.draw("boom-tip-boss", "boom", L.circle(5.7, 0, 0.20), "w", bw[0], bw[1], role="detail", tone=0.5, material=PS, views="e", parent="boom-body", sub=1)
    for name, a, b in (("right", bw[0], bw[0] + 0.07), ("left", bw[1] - 0.07, bw[1])):
        S.cyl("boom-tip-boss-" + name, "boom", (5.7, 0, a), (5.7, 0, b), 0.20, tone=0.5, **att)
    top = [(u, v + 0.02) for u, v in L.thin(L.chain(L.poly(F.BOOM).pts(CH), 0.55, 4.9), 0.006)]
    for name, z in (("right", bw[0] + 0.07), ("left", bw[1] - 0.07)):
        S.tube("boom-pipe-" + name, "boom", [(u, v, z) for u, v in top], 0.017, material=PS, role="detail", tone=0.25, region="attachment")
        S.draw("boom-pipe-" + name, "boom", L.band(top, 0.034), "w", z - 0.017, z + 0.017, role="detail", tone=0.25, material=PS, views="efp" if name == "right" else "fp", parent="boom-body", sub=3)
    D = K.PIN["D"]
    S.cyl("boom-rod-boss", "boom", (D[0], D[1], F.CYL_Z[1] - 0.06), (D[0], D[1], F.CYL_Z[0] + 0.06), 0.07, role="detail", tone=0.4, **att)
    S.draw("boom-rod-boss", "boom", L.circle(D[0], D[1], 0.07), "w", F.CYL_Z[1] - 0.06, F.CYL_Z[0] + 0.06, role="detail", tone=0.4, material=PS, parent="boom-body", sub=2)
    elug = [(2.74, 0.80), (3.26, 0.72), (3.07, 0.99), (2.93, 0.99)]
    S.ext("stick-cyl-lug", "boom", elug, "w", F.ZC - 0.10, F.ZC + 0.10, role="detail", tone=0.45, **att)
    S.draw("stick-cyl-foot", "boom", L.Path_of(elug), "w", F.ZC - 0.10, F.ZC + 0.10, role="detail", tone=0.45, material=PS, parent="boom-body", sub=1)
    sbox = L.poly(F.STICK_BOX).pts(CH)
    S.ext("stick-body", "stick", sbox, "w", sw[0], sw[1], tone=0.58, **att)
    S.draw("stick-body", "stick", L.Path_of(L.poly(F.STICK).pts(CH)), "w", sw[0], sw[1], tone=0.58, views="e", **att)
    S.draw("stick-body", "stick", L.Path_of(sbox), "w", sw[0], sw[1], tone=0.58, views="fp", **att)
    for name, a, b in (("right", sw[0], sw[0] + 0.05), ("left", sw[1] - 0.05, sw[1])):
        S.ext("stick-tail-" + name, "stick", F.STICK_TAIL, "w", a, b, tone=0.58, **att)
        S.draw("stick-tail-" + name, "stick", L.poly(F.STICK_TAIL), "w", a, b, tone=0.58, views="fp", **att)
    S.cyl("stick-root-boss", "stick", (0, 0, bw[0] + 0.07), (0, 0, bw[1] - 0.07), 0.19, tone=0.5, **att)
    S.draw("stick-root-boss", "stick", L.circle(0, 0, 0.19), "w", sw[0], sw[1], role="detail", tone=0.5, material=PS, views="e", parent="stick-body", sub=1)
    hlug = [(0.02, 0.46), (0.50, 0.40), (0.32, 0.63), (0.18, 0.63)]
    S.ext("bucket-cyl-lug", "stick", hlug, "w", F.ZC - 0.10, F.ZC + 0.10, role="detail", tone=0.45, **att)
    S.draw("bucket-cyl-foot", "stick", L.Path_of(hlug), "w", F.ZC - 0.10, F.ZC + 0.10, role="detail", tone=0.45, material=PS, parent="stick-body", sub=1)
    wear = dict(material=WS, region="attachment")
    shell = L.poly(F.BUCKET_SHELL).pts(CH)
    S.ext("bucket-shell", "bucket", L.strip(shell, 0.02), "w", kw[0] + 0.02, kw[1] - 0.02, tone=0.4, member="bucket", **wear)
    side = L.poly(F.BUCKET).pts(CH)
    for name, a, b in (("right", kw[0], kw[0] + 0.02), ("left", kw[1] - 0.02, kw[1])):
        S.ext("bucket-cheek-" + name, "bucket", side, "w", a, b, tone=0.42, member="bucket", **wear)
    S.ext("bucket-lip", "bucket", F.LIP, "w", kw[0], kw[1], tone=0.3, **wear)
    step = (kw[1] - kw[0] - 0.15) / 4
    S.ext("bucket-tooth", "bucket", F.TOOTH, "w", kw[0] + 0.03, kw[0] + 0.12, role="detail", tone=0.2, repeat={"count": 5, "translation": [0, 0, L.r4(step)]}, **wear)
    ear = [(-0.17, 0.62), (-0.22, 0.20), (-0.12, -0.10), (0.12, -0.10), (0.12, 0.30), (0.04, 0.62)]
    for name, z in (("right", F.ZC - 0.215), ("left", F.ZC + 0.17)):
        S.ext("bucket-ear-" + name, "bucket", ear, "w", z, z + 0.045, tone=0.38, member="bucket", **wear)
    S.draw("bucket-body", "bucket", L.Path_of(side), "w", kw[0], kw[1], tone=0.4, region="attachment", material=WS)
    S.draw("bucket-teeth", "bucket", L.poly(F.TOOTH), "w", kw[0] + 0.03, kw[1] - 0.03, role="detail", tone=0.2, region="attachment", material=WS, views="e", parent="bucket-body", sub=1)
    for k in range(5):
        S.draw("bucket-tooth-%d" % k, "bucket", L.poly(F.TOOTH), "w", kw[0] + 0.03 + k * step, kw[0] + 0.12 + k * step, role="detail", tone=0.2, region="attachment", material=WS, views="fp", parent="bucket-body", sub=1)
    S.draw("bucket-ear", "bucket", L.Path_of(ear), "w", F.ZC - 0.215, F.ZC + 0.215, role="detail", tone=0.38, material=WS, parent="bucket-body", sub=2)
    il = C.stadium(K.IDLER, 0.055).pts(CH)
    bl = C.stadium(K.LINK, 0.06).pts(CH)
    for name, z in (("right", F.ZC - 0.21), ("left", F.ZC + 0.175)):
        S.ext("idler-link-" + name, "idler-link", il, "w", z, z + 0.035, role="detail", tone=0.45, member="idler-link", **att)
        S.draw("idler-link-" + name, "idler-link", L.Path_of(il), "w", z, z + 0.035, role="detail", tone=0.45, region="attachment", material=PS, views="fp")
    S.draw("idler-link", "idler-link", L.Path_of(il), "w", F.ZC - 0.21, F.ZC + 0.21, role="detail", tone=0.45, region="attachment", material=PS, views="e")
    for name, z in (("right", F.ZC - 0.12), ("left", F.ZC + 0.075)):
        S.ext("bucket-link-" + name, "bucket-link", bl, "w", z, z + 0.045, role="detail", tone=0.45, member="bucket-link", **att)
        S.draw("bucket-link-" + name, "bucket-link", L.Path_of(bl), "w", z, z + 0.045, role="detail", tone=0.45, region="attachment", material=PS, views="fp")
    S.draw("bucket-link", "bucket-link", L.Path_of(bl), "w", F.ZC - 0.12, F.ZC + 0.12, role="detail", tone=0.45, region="attachment", material=PS, views="e")
    C.cylinder(S, "boom-cyl", F.CYL_Z, K.CYL["boom"], R["boom-rod"]["lengthM"])
    C.cylinder(S, "stick-cyl", (F.ZC,), K.CYL["stick"], R["stick-rod"]["lengthM"])
    C.cylinder(S, "bucket-cyl", (F.ZC,), K.CYL["bucket"], R["bucket-rod"]["lengthM"])
    C.girder(S, "boom", "boom", L.poly(F.BOOM_BOX).pts(0.01), bw, webs=True, wall=0.015)
    C.girder(S, "stick", "stick", L.poly(F.STICK_BOX).pts(0.01), sw, webs=True, wall=0.015)
    for name in ("right", "left"):
        S.st("boom-cheek-" + name, "boom")
        S.st("stick-tail-" + name, "stick")
    P = K.PIN
    for id, bone, at, half, member in (("boom-pin", "upper", P["A"], 0.37, "boom"), ("boom-cyl-pin", "upper", P["C"], 0.45, "boom-cyl"), ("boom-rod-pin", "boom", P["D"], 0.43, "boom-rod"), ("stick-cyl-pin", "boom", P["E"], 0.12, "stick-cyl"), ("stick-pin", "boom", (5.7, 0), 0.26, "stick"), ("stick-rod-pin", "stick", P["F"], 0.165, "stick-rod"), ("bucket-cyl-pin", "stick", P["H"], 0.12, "bucket-cyl"), ("idler-pin", "stick", P["I"], 0.23, "idler-link"), ("bucket-pin", "stick", (2.9, 0), 0.25, "bucket"), ("link-pin", "idler-link", (K.IDLER, 0), 0.23, "bucket-rod"), ("bucket-link-pin", "bucket", P["J"], 0.25, "bucket-link")):
        C.pin(S, id, bone, at, half, member)


def build(S, R):
    C.tracks(S, CH, band=0.045, links=0.08, wheels=0.09, rollers=8, carriers=2, frame_low=0.215, skirt=0.04)
    S.box("idler-yoke", C.U, (1.62, 0.45, C.G), (0.56, 0.20, 0.28), material=PS, role="detail", tone=0.4, region="tracks", repeat=C.TWO)
    C.carbody(S, cross=False, turret=0.68)
    S.cyl("carbody-ring", C.U, (0, 0.47, 0), (0, 1.06, 0), 0.68, material=PS, bore=0.58, tone=0.35, to="structure", member=C.U)
    for k, sz in enumerate((1, -1)):
        S.box("carbody-beam-%d" % (k + 1), C.U, (0, 0.66, 0), (2.50, 0.34, 0.26), material=PS, tone=0.4, rot=L.quat_y(math.degrees(math.atan2(-sz * 0.98, 0.80))), to="structure", member=C.U)
    C.supports(S)
    C.house(S, CH, toolbox=True, cover=0.06, grille=0.02, rail=True, foot=0.07)
    C.cab(S, CH, low=F.CAB["floor"], panes=("front", "right", "left", "rear"), wall=0.04)
    C.counterweight(S, CH)
    C.engine(S, bed=0.9)
    C.frame(S, cross=(-2.30, -1.20, 0.38), sides=True)
    arm(S, R)
    S.decide("Drawn anew for 1:100 on the corrected rig. Undercarriage: one 45 mm shoe band per track (shoe and grouser merged) on one link chain, plain sprocket and idler discs with the final drive hub, eight track rollers and two carrier rollers as discs, the track frame as a ridge-roofed channel over the rollers with the idler yoke as one block, the X carbody and one drum for turret and slew bearing. Upper structure: deck, the L-shaped engine house as one prism, tool box, hood cover, both grilles as 20 mm plates, exhaust stack as a plain tube, handrail, two boom foot plates, the counterweight with its tail swing arc as one prism, and the cab as a closed 40 mm shell of one ring and two side plates glazed by four 8 mm panes. Attachment: boom and stick boxes with their fork cheeks and root bosses, the cylinder feet as one lug block each, the two steel lines on the boom, bucket of 20 mm shell and cheeks with lip, five teeth and two ears, twin idler links and the two plates of the H link, and four cylinders as barrel and rod. Merged or left out against 1:50: grouser ribs, link rails, roller flanges and shafts, sprocket teeth, pillars and interior of the cab, lamps, mirrors, wear strips, glands, eyes and pins.", "Für 1:100 neu gezeichnet auf dem berichtigten Skelett. Unterwagen: je Kette ein 45-mm-Plattenband (Platte und Stollen zusammengefasst) auf einem Kettenstrang, glatte Scheiben für Antriebs- und Leitrad mit der Nabe des Fahrgetriebes, acht Laufrollen und zwei Stützrollen als Scheiben, der Laufwerksrahmen als U-Träger mit Firstdach über den Rollen und der Leitradgabel als Block, X-Mittelrahmen und eine Trommel für Turm und Drehkranzlager. Oberwagen: Plattform, das L-förmige Maschinenhaus als ein Prisma, Werkzeugkasten, Haubendeckel, beide Gitter als 20-mm-Bleche, Auspuff als glattes Rohr, Handlauf, zwei Auslegerfußbleche, das Gegengewicht mit seinem Heckschwenkbogen als ein Prisma und die Kabine als geschlossene 40-mm-Schale aus einem Ring und zwei Seitenblechen mit vier 8-mm-Scheiben. Ausrüstung: Ausleger- und Stielkasten mit Gabelwangen und Fußnaben, die Zylinderfüße als je ein Laschenblock, die zwei Stahlleitungen auf dem Ausleger, Löffel aus 20-mm-Schale und Wangen mit Schneide, fünf Zähnen und zwei Laschen, zwei Umlenkhebel und die zwei Bleche der H-Lasche sowie vier Zylinder als Rohr und Stange. Gegenüber 1:50 zusammengefasst oder weggelassen: Stollenrippen, Kettenschienen, Rollenbunde und Achsen, Zähne des Antriebsrads, Säulen und Innenleben der Kabine, Leuchten, Spiegel, Schleißleisten, Führungsbuchsen, Augen und Bolzen.")
    S.decide("Thermal: every region references its body surfaces; the faces of the panes that lie on the cab walls and the backs of the two grille plates are marked as not outer (C25 b). The engine compartment is one block for engine and cooler pack inside the house plus the two grille plates; the exhaust emitter is the outlet patch on the mouth of the stack.", "Thermik: Jede Region verweist auf ihre Körperoberflächen; die Flächen der Scheiben, die auf den Kabinenwänden liegen, und die Rückseiten der beiden Gitterbleche sind als nicht außenliegend markiert (C25 b). Der Motorraum ist ein Block für Motor und Kühlerpaket im Maschinenhaus sowie die beiden Gitterbleche; der Abgasemitter ist die Austrittsfläche auf der Mündung des Auspuffrohrs.")
    S.decide("Structure: track frames, the carbody as a hollow turret ring crossed by two diagonal beams, eight ground pads; the revolving frame of two centre rails, three cross members and two side beams with the slew flange, the boom foot plates, the ROPS frame of the cab, one engine bed, two counterweight carriers; boom and stick as box girders of a 15 mm flange band and two webs with their forks, bucket shell, cheeks and ears, the link plates, the cylinders as tubes and rods, eleven pins and the four concentrated masses.", "Tragwerk: Laufwerksrahmen, der Mittelrahmen als hohler Turmring mit zwei Diagonalträgern, acht Standflächen; der Oberwagenrahmen aus zwei Mittelholmen, drei Querträgern und zwei Seitenträgern mit dem Drehkranzflansch, die Auslegerfußbleche, der ROPS-Rahmen der Kabine, ein Motorlager, zwei Gegengewichtsträger; Ausleger und Stiel als Kastenträger aus einem 15-mm-Gurtband und zwei Stegen mit ihren Gabeln, Löffelschale, Wangen und Laschen, die Laschenbleche, die Zylinder als Rohre und Stangen, elf Bolzen und die vier Einzelmassen.")
