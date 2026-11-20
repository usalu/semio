"""🏗️ Excavator at 1:500 (smallest feature 75 mm): the silhouette masses of the machine. Two solid track blocks on a carbody box, one house prism that takes in the deck, tool box, counterweight, exhaust stack and a cab block with three panes; boom with its fork, stick and bucket as closed side profiles, links as bars, cylinders as barrel and rod. Structure: track beams, cross beam and post, two frame rails, straight bars along boom, stick and bucket, the cylinder bars and the three main pins."""
import sys

F = sys.modules["r3_excavator_form"]
L, K = F.L, F.K
C = F.load("r3-excavator-coarse")
PS, TS, WS = C.PS, C.TS, C.WS
CH = 0.02


def arm(S, R):
    bw, sw, kw = F.BOOM_W, F.STICK_W, F.BUCKET_W
    att = dict(material=PS, region="attachment")
    box = L.thin(L.poly(F.BOOM_BOX).pts(CH), 0.015)
    S.ext("boom-body", "boom", box, "w", bw[0], bw[1], tone=0.58, **att)
    S.draw("boom-body", "boom", L.Path_of(L.thin(L.poly(F.BOOM).pts(CH), 0.015)), "w", bw[0], bw[1], tone=0.58, views="e", **att)
    S.draw("boom-body", "boom", L.Path_of(box), "w", bw[0], bw[1], tone=0.58, views="fp", **att)
    for name, a, b in (("right", bw[0], bw[0] + 0.08), ("left", bw[1] - 0.08, bw[1])):
        S.ext("boom-cheek-" + name, "boom", F.BOOM_CHEEK, "w", a, b, tone=0.58, **att)
        S.draw("boom-cheek-" + name, "boom", L.poly(F.BOOM_CHEEK), "w", a, b, tone=0.58, views="fp", **att)
    stick = L.thin(L.poly(F.STICK).pts(CH), 0.015)
    S.ext("stick-body", "stick", stick, "w", sw[0], sw[1], tone=0.58, **att)
    S.draw("stick-body", "stick", L.Path_of(stick), "w", sw[0], sw[1], tone=0.58, **att)
    side = L.thin(L.poly(F.BUCKET).pts(CH), 0.015)
    S.ext("bucket-body", "bucket", side, "w", kw[0], kw[1], material=WS, tone=0.4, region="attachment")
    S.ext("bucket-teeth", "bucket", [(1.333, 0.147), (1.47, 0.0), (1.234, 0.155), (1.107, 0.3445)], "w", kw[0], kw[1], material=WS, tone=0.25, region="attachment")
    S.draw("bucket-body", "bucket", L.Path_of(side), "w", kw[0], kw[1], tone=0.4, region="attachment", material=WS)
    S.draw("bucket-teeth", "bucket", L.poly(F.TOOTH), "w", kw[0], kw[1], tone=0.25, region="attachment", material=WS, parent="bucket-body", sub=1)
    il = C.stadium(K.IDLER, 0.055).pts(CH)
    bl = C.stadium(K.LINK, 0.06).pts(CH)
    S.ext("idler-link", "idler-link", il, "w", F.ZC - 0.21, F.ZC + 0.21, tone=0.45, member="idler-link", **att)
    S.draw("idler-link", "idler-link", L.Path_of(il), "w", F.ZC - 0.21, F.ZC + 0.21, tone=0.45, region="attachment", material=PS)
    S.ext("bucket-link", "bucket-link", bl, "w", F.ZC - 0.12, F.ZC + 0.12, tone=0.45, member="bucket-link", **att)
    S.draw("bucket-link", "bucket-link", L.Path_of(bl), "w", F.ZC - 0.12, F.ZC + 0.12, tone=0.45, region="attachment", material=PS)
    C.cylinder(S, "boom-cyl", F.CYL_Z, K.CYL["boom"], R["boom-rod"]["lengthM"], role="body", bore=False)
    C.cylinder(S, "stick-cyl", (F.ZC,), K.CYL["stick"], R["stick-rod"]["lengthM"], role="body", bore=False)
    C.cylinder(S, "bucket-cyl", (F.ZC,), K.CYL["bucket"], R["bucket-rod"]["lengthM"], role="body", bore=False)
    C.bars(S, "boom-bar", "boom", [(0, 0), (2.58, 0.52), (5.7, 0)], (0.36, 0.40), "boom")
    C.bars(S, "stick-bar", "stick", [(-0.78, 0.44), (0.10, 0.10), (2.9, 0)], (0.22, 0.26), "stick")
    C.bars(S, "bucket-bar", "bucket", [(0, 0.05), (0.40, 0.58), (1.25, 0.20)], (0.12, 0.90), "bucket")
    P = K.PIN
    for id, bone, at, half, member in (("boom-pin", "upper", P["A"], 0.37, "boom"), ("stick-pin", "boom", (5.7, 0), 0.26, "stick"), ("bucket-pin", "stick", (2.9, 0), 0.25, "bucket")):
        C.pin(S, id, bone, at, half, member, r=0.08)


def build(S, R):
    C.tracks(S, CH, band=0, links=0, wheels=0, rollers=0, carriers=0, frame_low=0, solid_loop=True)
    C.carbody(S, cross=True, turret=0.68)
    for k, zc in enumerate((C.G, -C.G)):
        S.box("track-beam-%d" % (k + 1), C.U, (-0.10, 0.38, zc), (3.20, 0.34, 0.30), material=PS, tone=0.4, to="structure", member=C.U)
    S.box("carbody-beam", C.U, (0, 0.66, 0), (0.60, 0.34, 2.08), material=PS, tone=0.4, to="structure", member=C.U)
    S.cyl("carbody-post", C.U, (0, 0.83, 0), (0, 1.06, 0), 0.40, material=PS, tone=0.35, to="structure", member=C.U)
    C.supports(S)
    C.house(S, CH, toolbox=True, cover=0, grille=0, rail=False, foot=0.08, merged_deck=True, regions=["house", "engine-compartment", "exhaust-stack"])
    C.cab(S, CH, low=0, panes=("front", "right", "left"), wall=0.08)
    C.counterweight(S, CH)
    C.engine(S, bed=1.2)
    C.frame(S, cross=(), sides=False)
    arm(S, R)
    S.decide("Drawn anew for 1:500 as the silhouette masses. Two solid track blocks with the true loop outline (band, frame and wheels merged) on a carbody box with the turret drum; the upper structure as one house prism that takes in the deck, the tool box, the counterweight prism, the exhaust stack (0.12 m, 0.24 mm on paper) and the cab as one closed 80 mm shell from the frame to the roof with three panes; boom box with its two fork cheeks, stick and bucket as closed side profiles with one tooth bar, the two links as bars, the four cylinders as barrel and rod. Merged or left out against 1:200: track band, frame beams and end discs, the X of the carbody, the deck, grille plates, lug, stick fork, bucket shell, cheeks and ears.", "Für 1:500 neu als Silhouettenmassen gezeichnet. Zwei massive Kettenblöcke mit dem wahren Schleifenumriss (Band, Rahmen und Räder zusammengefasst) auf einem Mittelrahmenkasten mit der Turmtrommel; der Oberwagen als ein Hausprisma, das die Plattform aufnimmt, der Werkzeugkasten, das Gegengewichtsprisma, das Auspuffrohr (0,12 m, 0,24 mm auf dem Papier) und die Kabine als eine geschlossene 80-mm-Schale vom Rahmen bis zum Dach mit drei Scheiben; Auslegerkasten mit seinen zwei Gabelwangen, Stiel und Löffel als geschlossene Seitenprofile mit einer Zahnleiste, die zwei Laschen als Stäbe, die vier Zylinder als Rohr und Stange. Gegenüber 1:200 zusammengefasst oder weggelassen: Kettenband, Rahmenträger und Endscheiben, das X des Mittelrahmens, die Plattform, Gitterbleche, Lasche, Stielgabel, Löffelschale, Wangen und Laschen.")
    S.decide("Thermal: every region references its body mass; the engine compartment is the engine block inside the house and two patches let into the side walls where the grilles are, the exhaust emitter is the outlet patch on the stack mouth; pane faces on the cab walls are not outer (C25 b).", "Thermik: Jede Region verweist auf ihre Körpermasse; der Motorraum ist der Motorblock im Haus mit zwei in die Seitenwände eingelassenen Flächen an der Stelle der Gitter, der Abgasemitter ist die Austrittsfläche auf der Rohrmündung; Scheibenflächen an den Kabinenwänden sind nicht außenliegend (C25 b).")
    S.decide("Structure: its own load path, never the body masses: two track beams, the carbody cross beam and post, eight ground pads; two frame rails with the slew flange, the boom foot plates, the ROPS frame, engine bed and counterweight carriers; boom, stick and bucket as straight box bars from pin to pin through the knee, both links, the cylinders as bars, the three main pins and four masses.", "Tragwerk: ein eigener Lastpfad, nie die Körpermassen: zwei Laufwerksträger, der Mittelrahmen-Querträger mit Stütze, acht Standflächen; zwei Rahmenholme mit dem Drehkranzflansch, die Auslegerfußbleche, der ROPS-Rahmen, Motorlager und Gegengewichtsträger; Ausleger, Stiel und Löffel als gerade Kastenstäbe von Bolzen zu Bolzen über das Knie, beide Laschen, die Zylinder als Stäbe, die drei Hauptbolzen und vier Massen.")
