"""🏗️ Excavator at 1:200 (smallest feature 30 mm): masses, not parts. Each track is one 125 mm band around a frame beam that takes in the rollers, with two end discs; the upper structure is deck, house prism, tool box, counterweight and a closed cab block with three panes; boom and stick are forked boxes without bosses, the bucket a 30 mm shell with cheeks, tooth bar and ears, the links single bars and every cylinder a barrel and a rod. Structure: frame beams, cross beam and turret ring, a reduced revolving frame, box girders as one flange band each, solid cylinder bars and the pins of the main joints."""
import sys

F = sys.modules["r3_excavator_form"]
L, K = F.L, F.K
C = F.load("r3-excavator-coarse")
PS, TS, WS = C.PS, C.TS, C.WS
CH = 0.008


def arm(S, R):
    bw, sw, kw = F.BOOM_W, F.STICK_W, F.BUCKET_W
    att = dict(material=PS, region="attachment")
    box = L.thin(L.poly(F.BOOM_BOX).pts(CH), 0.006)
    S.ext("boom-body", "boom", box, "w", bw[0], bw[1], tone=0.58, **att)
    S.draw("boom-body", "boom", L.Path_of(L.thin(L.poly(F.BOOM).pts(CH), 0.006)), "w", bw[0], bw[1], tone=0.58, views="e", **att)
    S.draw("boom-body", "boom", L.Path_of(box), "w", bw[0], bw[1], tone=0.58, views="fp", **att)
    for name, a, b in (("right", bw[0], bw[0] + 0.07), ("left", bw[1] - 0.07, bw[1])):
        S.ext("boom-cheek-" + name, "boom", F.BOOM_CHEEK, "w", a, b, tone=0.58, **att)
        S.draw("boom-cheek-" + name, "boom", L.poly(F.BOOM_CHEEK), "w", a, b, tone=0.58, views="fp", **att)
    D = K.PIN["D"]
    S.cyl("boom-rod-boss", "boom", (D[0], D[1], F.CYL_Z[1] - 0.06), (D[0], D[1], F.CYL_Z[0] + 0.06), 0.07, role="body", tone=0.4, **att)
    S.draw("boom-rod-boss", "boom", L.circle(D[0], D[1], 0.07), "w", F.CYL_Z[1] - 0.06, F.CYL_Z[0] + 0.06, role="body", tone=0.4, material=PS, parent="boom-body", sub=2)
    elug = [(2.74, 0.80), (3.26, 0.72), (3.07, 0.99), (2.93, 0.99)]
    S.ext("stick-cyl-lug", "boom", elug, "w", F.ZC - 0.10, F.ZC + 0.10, tone=0.5, **att)
    S.draw("stick-cyl-foot", "boom", L.Path_of(elug), "w", F.ZC - 0.10, F.ZC + 0.10, tone=0.5, material=PS, parent="boom-body", sub=1)
    sbox = L.thin(L.poly(F.STICK_BOX).pts(CH), 0.006)
    S.ext("stick-body", "stick", sbox, "w", sw[0], sw[1], tone=0.58, **att)
    S.draw("stick-body", "stick", L.Path_of(L.thin(L.poly(F.STICK).pts(CH), 0.006)), "w", sw[0], sw[1], tone=0.58, views="e", **att)
    S.draw("stick-body", "stick", L.Path_of(sbox), "w", sw[0], sw[1], tone=0.58, views="fp", **att)
    for name, a, b in (("right", sw[0], sw[0] + 0.05), ("left", sw[1] - 0.05, sw[1])):
        S.ext("stick-tail-" + name, "stick", F.STICK_TAIL, "w", a, b, tone=0.58, **att)
        S.draw("stick-tail-" + name, "stick", L.poly(F.STICK_TAIL), "w", a, b, tone=0.58, views="fp", **att)
    wear = dict(material=WS, region="attachment")
    shell = L.thin(L.poly(F.BUCKET_SHELL).pts(CH), 0.006)
    side = L.thin(L.poly(F.BUCKET).pts(CH), 0.006)
    S.ext("bucket-shell", "bucket", L.strip(shell, 0.03), "w", kw[0] + 0.03, kw[1] - 0.03, tone=0.4, member="bucket", **wear)
    for name, a, b in (("right", kw[0], kw[0] + 0.03), ("left", kw[1] - 0.03, kw[1])):
        S.ext("bucket-cheek-" + name, "bucket", side, "w", a, b, tone=0.42, **wear)
    S.ext("bucket-teeth", "bucket", [(1.333, 0.147), (1.47, 0.0), (1.234, 0.155), (1.107, 0.3445)], "w", kw[0], kw[1], tone=0.25, **wear)
    ear = [(-0.17, 0.62), (-0.22, 0.20), (-0.12, -0.10), (0.12, -0.10), (0.12, 0.30), (0.04, 0.62)]
    for name, z in (("right", F.ZC - 0.215), ("left", F.ZC + 0.17)):
        S.ext("bucket-ear-" + name, "bucket", ear, "w", z, z + 0.045, tone=0.38, **wear)
    S.draw("bucket-body", "bucket", L.Path_of(side), "w", kw[0], kw[1], tone=0.4, region="attachment", material=WS)
    S.draw("bucket-teeth", "bucket", L.poly(F.TOOTH), "w", kw[0], kw[1], tone=0.25, region="attachment", material=WS, parent="bucket-body", sub=1)
    S.draw("bucket-ear", "bucket", L.Path_of(ear), "w", F.ZC - 0.215, F.ZC + 0.215, tone=0.38, material=WS, parent="bucket-body", sub=2)
    il = C.stadium(K.IDLER, 0.055).pts(CH)
    bl = C.stadium(K.LINK, 0.06).pts(CH)
    S.ext("idler-link", "idler-link", il, "w", F.ZC - 0.21, F.ZC + 0.21, tone=0.45, member="idler-link", **att)
    S.draw("idler-link", "idler-link", L.Path_of(il), "w", F.ZC - 0.21, F.ZC + 0.21, tone=0.45, region="attachment", material=PS)
    S.ext("bucket-link", "bucket-link", bl, "w", F.ZC - 0.12, F.ZC + 0.12, tone=0.45, member="bucket-link", **att)
    S.draw("bucket-link", "bucket-link", L.Path_of(bl), "w", F.ZC - 0.12, F.ZC + 0.12, tone=0.45, region="attachment", material=PS)
    C.cylinder(S, "boom-cyl", F.CYL_Z, K.CYL["boom"], R["boom-rod"]["lengthM"], role="body", bore=False)
    C.cylinder(S, "stick-cyl", (F.ZC,), K.CYL["stick"], R["stick-rod"]["lengthM"], role="body", bore=False)
    C.cylinder(S, "bucket-cyl", (F.ZC,), K.CYL["bucket"], R["bucket-rod"]["lengthM"], role="body", bore=False)
    C.girder(S, "boom", "boom", L.poly(F.BOOM).pts(0.02), bw, webs=False, wall=0.03)
    C.girder(S, "stick", "stick", L.poly(F.STICK).pts(0.02), sw, webs=False, wall=0.03)
    P = K.PIN
    for id, bone, at, half, member in (("boom-pin", "upper", P["A"], 0.37, "boom"), ("boom-cyl-pin", "upper", P["C"], 0.45, "boom-cyl"), ("boom-rod-pin", "boom", P["D"], 0.43, "boom-rod"), ("stick-pin", "boom", (5.7, 0), 0.26, "stick"), ("stick-rod-pin", "stick", P["F"], 0.165, "stick-rod"), ("bucket-pin", "stick", (2.9, 0), 0.25, "bucket"), ("link-pin", "idler-link", (K.IDLER, 0), 0.23, "bucket-rod")):
        C.pin(S, id, bone, at, half, member, r=0.06)


def build(S, R):
    C.tracks(S, CH, band=0.125, links=0, wheels=0.10, rollers=0, carriers=0, frame_low=0.125)
    C.carbody(S, cross=False, turret=0.68)
    S.cyl("carbody-ring", C.U, (0, 0.47, 0), (0, 1.06, 0), 0.68, material=PS, bore=0.56, tone=0.35, to="structure", member=C.U)
    S.box("carbody-beam", C.U, (0, 0.66, 0), (0.60, 0.34, 2.08), material=PS, tone=0.4, to="structure", member=C.U)
    C.supports(S)
    C.house(S, CH, toolbox=True, cover=0, grille=0.04, rail=False, foot=0.07)
    C.cab(S, CH, low=F.CAB["floor"], panes=("front", "right", "left"), wall=0.06)
    C.counterweight(S, CH)
    C.engine(S, bed=1.2)
    C.frame(S, cross=(-2.30, 0.38), sides=False)
    arm(S, R)
    S.decide("Drawn anew for 1:200 as masses. Each track is one 125 mm band (shoes, grousers and links merged) around one frame beam that takes in the rollers, closed at the ends by a sprocket and an idler disc with the final drive hub; X carbody and one drum for turret and slew bearing. Upper structure: deck, engine house prism, tool box, both grilles as 40 mm plates, exhaust stack, two boom foot plates, counterweight prism and the cab as a closed 60 mm shell with front, right and left pane. Attachment: boom and stick as forked boxes with generalised outlines, one lug block for the stick cylinder, bucket of 30 mm shell and cheeks with one tooth bar and two ears, idler links and H link as one bar each, four cylinders as barrel and rod. Merged or left out against 1:100: link chain, rollers, carrier rollers, idler yoke, hood cover, handrail, rear pane, bosses, the bucket cylinder lug, separate teeth and the paired link plates.", "Für 1:200 neu als Massen gezeichnet. Jede Kette ist ein 125-mm-Band (Platten, Stollen und Glieder zusammengefasst) um einen Rahmenträger, der die Rollen aufnimmt, an den Enden geschlossen durch je eine Scheibe für Antriebs- und Leitrad mit der Nabe des Fahrgetriebes; X-Mittelrahmen und eine Trommel für Turm und Drehkranzlager. Oberwagen: Plattform, Maschinenhausprisma, Werkzeugkasten, beide Gitter als 40-mm-Bleche, Auspuffrohr, zwei Auslegerfußbleche, Gegengewichtsprisma und die Kabine als geschlossene 60-mm-Schale mit Front-, rechter und linker Scheibe. Ausrüstung: Ausleger und Stiel als Gabelkästen mit generalisierten Umrissen, ein Laschenblock für den Stielzylinder, Löffel aus 30-mm-Schale und Wangen mit einer Zahnleiste und zwei Laschen, Umlenkhebel und H-Lasche als je ein Stab, vier Zylinder als Rohr und Stange. Gegenüber 1:100 zusammengefasst oder weggelassen: Kettenstrang, Laufrollen, Stützrollen, Leitradgabel, Haubendeckel, Handlauf, Heckscheibe, Naben, die Lasche des Löffelzylinders, einzelne Zähne und die paarigen Laschenbleche.")
    S.decide("Thermal: every region references its body masses; pane faces on the cab walls and the backs of the grille plates are not outer (C25 b). The engine compartment is the engine block inside the house and the two grille plates; the exhaust emitter is the outlet patch on the stack mouth.", "Thermik: Jede Region verweist auf ihre Körpermassen; Scheibenflächen an den Kabinenwänden und die Rückseiten der Gitterbleche sind nicht außenliegend (C25 b). Der Motorraum ist der Motorblock im Maschinenhaus mit den beiden Gitterblechen; der Abgasemitter ist die Austrittsfläche auf der Rohrmündung.")
    S.decide("Structure: the two track frame beams, one carbody cross beam under the turret ring, eight ground pads; a revolving frame of two centre rails and two cross members with the slew flange, the boom foot plates, the ROPS frame, engine bed and counterweight carriers; boom and stick as one 30 mm flange band each around the side outline, the bucket shell, both links, the cylinders as solid barrels and rods, seven pins and four masses.", "Tragwerk: die zwei Laufwerksrahmenträger, ein Mittelrahmen-Querträger unter dem Turmring, acht Standflächen; ein Oberwagenrahmen aus zwei Mittelholmen und zwei Querträgern mit dem Drehkranzflansch, die Auslegerfußbleche, der ROPS-Rahmen, Motorlager und Gegengewichtsträger; Ausleger und Stiel als je ein 30-mm-Gurtband um den Seitenumriss, die Löffelschale, beide Laschen, die Zylinder als massive Rohre und Stangen, sieben Bolzen und vier Massen.")
