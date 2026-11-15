from author_11_lib import *

H = Ctx("housing", "cast-iron", None, "body", 0.42)
M = Ctx("motor", "aluminium", None, "body", 0.62)
TB = Ctx("terminal", "polyamide", None, "body", 0.78)
R = Ctx("rotor", "window-glass", None, "detail", 0.95)
copper, brass = H(material="copper", tone=0.7), H(material="brass", tone=0.5)
PW = 0.04
steel = H(role="detail", tone=0.25)
TH_C, TH_M, TH_T, TH_W = H(layer="shell"), M(layer="head"), TB(layer="box"), M(material="window-glass", layer="pane", tone=0.18)


def reg(s, r):
    s["region"] = r
    return s


def union(side, sg, mat):
    x0, x1 = (0.054, 0.0775) if sg > 0 else (-0.0775, -0.054)
    prof = [[0, 0.021], [0.013, 0.021], [0.013, 0.0195], [0.0235, 0.0195]] if sg < 0 else [[0, 0.0195], [0.0105, 0.0195], [0.0105, 0.021], [0.0235, 0.021]]
    return rev("union-" + side, mat, prof, "u", origin=(x0, 0, PW))


def dome(sid, ctx, zbase, zapex, r, rows=4):
    n = zapex - zbase
    full = [(zbase, r, r, 2), (zbase + n * 0.4, r * 0.88, r * 0.88, 2), (zbase + n * 0.75, r * 0.6, r * 0.6, 2), (zapex, r * 0.25, r * 0.25, 2)]
    return lf(sid, ctx, "w", full if rows == 4 else [full[0], full[2], full[3]], cs=False, ce=True)


def tube_w(sid, ctx, z0, z1, r):
    return lf(sid, ctx, "w", [(z0, r, r, 2), (z1, r, r, 2)], cs=False, ce=False)


def term_shell(sid, ctx, x0, x1, y1, z0, z1, e=8):
    return lf(sid, ctx, "v", [(0, (z1 - z0) / 2, (x1 - x0) / 2, e, (z0 + z1) / 2, (x0 + x1) / 2), (y1, (z1 - z0) / 2, (x1 - x0) / 2, e, (z0 + z1) / 2, (x0 + x1) / 2)], cs=False, ce=True)


def plume(sid, ctx, r, y0, y1, cz):
    return tag(lf(sid, ctx, "v", [(y0, r, r, 2, cz, 0), (y1, r, r, 2, cz, 0)], cs=False, ce=False), tags=["motor-plume"])


def pin_support(sid, sz, ln):
    return tag(cyl(sid, steel(member="housing", part="support"), "w", (-0.085, 0, PW), ln, sz), tags=["pipe-left"])


def roller_support(sid, r, ln):
    return tag(cyl(sid, steel(member="housing", part="support"), "w", (0.085, 0, PW), ln, r), tags=["pipe-right"])


# ------------------------------------------------------------------ 1:50
body50 = [
    rev("casing", H, [[0, 0.0515], [0.006, 0.0575], [0.044, 0.0575], [0.05, 0.0515]], "w", origin=(0, 0, 0.015)),
    union("l", -1, brass), union("r", 1, brass),
    pipe("stub-l", copper, "u", (-0.0825, 0, PW), 0.015, 0.014, 0.009, n=32),
    pipe("stub-r", copper, "u", (0.0825, 0, PW), 0.015, 0.014, 0.009, n=32),
    rev("motor-head", M, [[0, 0.0415], [0.004, 0.0455], [0.071, 0.0455], [0.075, 0.0415]], "w", origin=(0, 0, -0.06)),
    pipe("bezel", M(role="detail", tone=0.5), "w", (0, 0, -0.061), 0.008, 0.0305, 0.0265, n=40),
    cyl("window", M(material="window-glass", role="glazing", tone=0.18), "w", (0, 0, -0.06425), 0.0015, 0.0265),
    cyl("dial", M(material="polyamide", role="texture", tone=0.25), "w", (0, 0, -0.06275), 0.0015, 0.0265),
    box("terminal-neck", TB(tone=0.7), (-0.02, -0.0075, -0.045), (0.02, 0.0005, -0.005), cr=0.002),
    box("terminal-box", TB, (-0.03, 0, -0.05), (0.03, 0.025, 0), cr=0.004),
    box("terminal-lid", TB(tone=0.82), (-0.0305, 0.025, -0.0505), (0.0305, 0.0425, 0.0005), cr=0.004),
    cyl("gland", TB(role="detail", tone=0.4), "u", (-0.03825, 0.01125, -0.025), 0.0165, 0.0068),
    ell("led", TB(role="detail", tone=0.95), (0, 0.033, -0.047), (0.0045, 0.0045, 0.004)),
    ext("pointer", R, [[-0.007, 0], [0.007, 0], [0, 0.026]], "xy", -0.0625, -0.055),
    ext("tail", R(tone=0.7), [[-0.005, 0], [0.005, 0], [0, -0.012]], "xy", -0.0625, -0.055),
    cyl("hub", R(tone=0.35), "w", (0, 0, -0.05875), 0.0075, 0.0055),
]
th50 = [
    th("th-casing", "casing", "casing", "shell"),
    th("th-union-l", "union-l", "casing", "shell"),
    th("th-union-r", "union-r", "casing", "shell"),
    th("th-stub-l", "stub-l", "casing", "shell"),
    th("th-stub-r", "stub-r", "casing", "shell"),
    th("th-motor-head", "motor-head", "motor", "head"),
    th("th-bezel", "bezel", "motor", "head"),
    th("th-terminal-box", "terminal-box", "terminal-box", "box"),
    th("th-terminal-lid", "terminal-lid", "terminal-box", "box"),
    th("th-rotor-window", "window", "rotor-window", "pane"),
    reg(plume("th-motor-plume", TH_M, 0.04, 0.0345, 0.0425, -0.025), "motor"),
]
st50 = [
    cyl("member-housing", steel(member="housing", part="member", bone="housing"), "u", (0, 0, PW), 0.165, 0.0185),
    stm("member-motor", "motor-head", "motor", "member"),
    stm("member-terminal", "terminal-box", "terminal", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0125), 0.095, 0.008),
    cyl("joint-motor", steel(member="motor", part="joint", bone="motor"), "w", (0, 0, 0.015), 0.015, 0.0415),
    box("joint-terminal", steel(member="terminal", part="joint", bone="terminal"), (-0.02, -0.0075, -0.045), (0.02, 0.0005, -0.005)),
    cyl("joint-rotor", steel(member="rotor", part="joint", bone="rotor"), "w", (0, 0, -0.0525), 0.015, 0.0105),
    ell("mass-housing", steel(member="housing", part="mass", tone=0.15), (0, 0, PW), (0.04, 0.04, 0.0125)),
    ell("mass-motor", steel(member="motor", part="mass", tone=0.15, bone="motor"), (0, 0, -0.0225), (0.035, 0.035, 0.0375)),
    pin_support("support-pipe-left", 0.0075, 0.028),
    roller_support("support-pipe-right", 0.0075, 0.028),
]
D50 = [
    text("Own 1:50 model from the three drawings: the pump head is a chamfered 0.115 m cast disc about the lateral axis w (z 0.015..0.065), the wet-rotor motor a 0.091 m revolved cylinder (z -0.06..0.015) with an 8 mm bezel ring and the glass window and dial sheet in the recess, the terminal box a rounded box on a neck block with lid, cable gland and LED lens; the two unions are stepped brass revolves and the copper stubs hollow rings (18 mm bore, end-on in the front view). Rotor pointer, tail and hub are 7.5 mm bodies behind the glass.",
         "Eigenes 1:50-Modell aus den drei Zeichnungen: der Pumpenkopf ist eine gefaste Gussscheibe von 0,115 m um die Querachse w (z 0,015..0,065), der Nassläufermotor ein gedrehter Zylinder von 0,091 m (z -0,06..0,015) mit 8-mm-Blendenring sowie Glas und Zifferblattblech in der Vertiefung, der Klemmenkasten ein gerundeter Kasten auf einem Halsblock mit Deckel, Kabelverschraubung und LED-Linse; die zwei Verschraubungen sind gestufte Messingdrehteile, die Kupferstutzen hohle Ringe (Bohrung 18 mm, stirnseitig in der Ansicht). Zeiger, Schwanz und Nabe des Rotors sind 7,5-mm-Körper hinter dem Glas."),
    text("Repaired for the 7.5 mm feature: bezel ring 8 mm deep, hub and pointer 7.5 mm thick, LED a domed lens; the 0.7 mm motor ring is merged into the motor head, the glass pane and the dial sheet stay as thin sheets (roles glazing and texture). The unions' z axis follows the plan (w = 0.04 m, the middle of the casing).",
         "Für das 7,5-mm-Merkmal repariert: Blendenring 8 mm tief, Nabe und Zeiger 7,5 mm dick, LED als gewölbte Linse; der 0,7-mm-Motorring geht im Motorkopf auf, Glasscheibe und Zifferblattblech bleiben dünne Bleche (Rollen glazing und texture). Die z-Achse der Verschraubungen folgt dem Grundriss (w = 0,04 m, Gehäusemitte)."),
    text("Thermal (area = exposed surface, buried faces do not count): casing = cast head, unions and copper stubs, motor = motor head and bezel, terminal box = box and lid, rotor window = the glass pane, all by reference to the body solids; the motor plume is a 0.08 m ring at the motor-top anchor (tag motor-plume). Single-layer regions are surfaces; areaM2 of motor, casing and terminal box was corrected at the root to the modelled exposed surfaces (see report).",
         "Thermik (Fläche = freie Oberfläche, verdeckte Flächen zählen nicht): Gehäuse = Gusskopf, Verschraubungen und Kupferstutzen, Motor = Motorkopf und Blende, Klemmenkasten = Kasten und Deckel, Schauglas = die Glasscheibe, alles als Verweis auf die Körper; die Motorfahne ist ein Ring von 0,08 m am Anker motor-top (Markierung motor-plume). Einlagige Regionen sind Flächen; areaM2 von Motor, Gehäuse und Klemmenkasten wurde an der Wurzel auf die modellierten freien Flächen korrigiert (siehe Bericht)."),
    text("Structure: a 0.037 m solid bar through both unions is the housing member, the motor head and the terminal box are the motor and terminal members, a 15 mm shaft is the rotor member; flange, neck and bearing sleeve are the joints; mass ellipsoids sit in casing and motor. The union pin is a 15 mm cross pin through the left stub; the roller support is a 15 mm roller on the right stub.",
         "Tragwerk: ein Vollstab von 0,037 m durch beide Verschraubungen ist der Gehäusestab, Motorkopf und Klemmenkasten sind Motor- und Klemmenstab, eine 15-mm-Welle ist der Rotorstab; Flansch, Hals und Lagerhülse sind die Gelenke; Massenellipsoide sitzen in Gehäuse und Motor. Das Festlager ist ein 15-mm-Querbolzen durch den linken Stutzen; das Rollenlager ist eine 15-mm-Rolle am rechten Stutzen."),
]

# ------------------------------------------------------------------ 1:100
body100 = [
    rev("casing", H, [[0, 0.0515], [0.006, 0.0575], [0.044, 0.0575], [0.05, 0.0515]], "w", origin=(0, 0, 0.015)),
    cyl("stub-l", copper, "u", (-0.0725, 0, PW), 0.035, 0.017),
    cyl("stub-r", copper, "u", (0.0725, 0, PW), 0.035, 0.017),
    rev("motor-head", M, [[0, 0.0415], [0.004, 0.0455], [0.076, 0.0455], [0.08, 0.0415]], "w", origin=(0, 0, -0.065)),
    cyl("window", M(material="window-glass", role="glazing", tone=0.18), "w", (0, 0, -0.06575), 0.0015, 0.026),
    box("terminal-neck", TB(tone=0.7), (-0.02, -0.015, -0.045), (0.02, 0.0005, -0.005)),
    box("terminal-box", TB, (-0.03, 0, -0.05), (0.03, 0.0425, 0)),
]
th100 = [
    th("th-casing", "casing", "casing", "shell"),
    th("th-stub-l", "stub-l", "casing", "shell"),
    th("th-stub-r", "stub-r", "casing", "shell"),
    th("th-motor-head", "motor-head", "motor", "head"),
    th("th-terminal-box", "terminal-box", "terminal-box", "box"),
    th("th-rotor-window", "window", "rotor-window", "pane"),
    reg(plume("th-motor-plume", TH_M, 0.04, 0.0275, 0.0425, -0.025), "motor"),
]
st100 = [
    cyl("member-housing", steel(member="housing", part="member", bone="housing"), "u", (0, 0, PW), 0.18, 0.017),
    stm("member-motor", "motor-head", "motor", "member"),
    stm("member-terminal", "terminal-box", "terminal", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0125), 0.095, 0.008),
    cyl("joint-motor", steel(member="motor", part="joint", bone="motor"), "w", (0, 0, 0.015), 0.015, 0.0415),
    stm("joint-terminal", "terminal-neck", "terminal", "joint"),
    ell("mass-housing", steel(member="housing", part="mass", tone=0.15), (0, 0, PW), (0.04, 0.04, 0.0125)),
    ell("mass-motor", steel(member="motor", part="mass", tone=0.15, bone="motor"), (0, 0, -0.0225), (0.035, 0.035, 0.0375)),
    pin_support("support-pipe-left", 0.008, 0.03),
    roller_support("support-pipe-right", 0.008, 0.03),
]
D100 = [
    text("Own 1:100 model against the 1:100 shapes: the head disc with a plain 6 mm chamfer, the two unions and stubs merged into one 34 mm cylinder each (nut, step and bore are below 15 mm), the motor as one 0.08 m cylinder that now reaches the window face at z -0.065, and the terminal box without lid, gland and LED on a 15 mm neck. Bezel, dial sheet and rotor parts are dropped: they sit behind the window and are smaller than 15 mm.",
         "Eigenes 1:100-Modell gegen die 1:100-Formen: die Kopfscheibe mit einfacher 6-mm-Fase, die zwei Verschraubungen samt Stutzen zu je einem 34-mm-Zylinder zusammengefasst (Mutter, Stufe und Bohrung liegen unter 15 mm), der Motor als ein Zylinder von 0,08 m, der nun bis zur Glasfläche bei z -0,065 reicht, und der Klemmenkasten ohne Deckel, Verschraubung und LED auf einem 15-mm-Hals. Blende, Zifferblattblech und Rotorteile entfallen: sie liegen hinter dem Fenster und sind kleiner als 15 mm."),
    text("Thermal and structure are rebuilt on the 1:100 solids: casing and motor by reference, the terminal box, a 1.5 mm glass disc on the motor face and the plume ring, all by reference to the 1:100 body; structure keeps all four bones as members (bar, motor head, terminal box, 15 mm rotor shaft), flange and neck joints, two mass ellipsoids, pin and roller supports at 15 mm.",
         "Thermik und Tragwerk sind auf den 1:100-Körpern neu aufgebaut: Gehäuse und Motor als Verweis, der Klemmenkasten, eine 1,5-mm-Glasscheibe auf der Motorfläche und der Fahnenring, alles als Verweis auf den 1:100-Körper; das Tragwerk behält alle vier Knochen als Stäbe (Stab, Motorkopf, Klemmenkasten, 15-mm-Rotorwelle), Flansch- und Halsgelenk, zwei Massenellipsoide, Fest- und Rollenlager in 15 mm."),
]

# ------------------------------------------------------------------ 1:200
body200 = [
    rev("body", H, [[0, 0.0], [0.0, 0.0455], [0.08, 0.0455], [0.08, 0.0575], [0.13, 0.0575], [0.13, 0.0]], "w", origin=(0, 0, -0.065)),
    cyl("pipe", copper, "u", (0, 0, PW), 0.18, 0.0195),
    cyl("window", M(material="window-glass", role="glazing", tone=0.18), "w", (0, 0, -0.06575), 0.0015, 0.026),
    box("terminal-box", TB, (-0.03, 0, -0.05), (0.03, 0.0425, 0)),
]
th200 = [
    reg(rev("th-casing", TH_C, [[0, 0.0523], [0.006, 0.0583], [0.044, 0.0583], [0.05, 0.0523]], "w", origin=(0, 0, 0.015)), "casing"),
    reg(rev("th-motor-head", TH_M, [[0, 0.0423], [0.004, 0.0463], [0.076, 0.0463], [0.08, 0.0423]], "w", origin=(0, 0, -0.065)), "motor"),
    reg(lf("th-pipe-l", TH_C, "u", [(-0.09, 0.0203, 0.0203, 2, PW, 0), (-0.0585, 0.0203, 0.0203, 2, PW, 0)], cs=False, ce=False), "casing"),
    reg(lf("th-pipe-r", TH_C, "u", [(0.0585, 0.0203, 0.0203, 2, PW, 0), (0.09, 0.0203, 0.0203, 2, PW, 0)], cs=False, ce=False), "casing"),
    th("th-terminal-box", "terminal-box", "terminal-box", "box"),
    th("th-rotor-window", "window", "rotor-window", "pane"),
    reg(plume("th-motor-plume", TH_M, 0.04, 0.0125, 0.0425, -0.025), "motor"),
]
st200 = [
    cyl("member-housing", steel(member="housing", part="member", bone="housing"), "u", (0, 0, PW), 0.18, 0.0185),
    stm("member-motor", "th-motor-head" if False else "body", "motor", "member"),
    stm("member-terminal", "terminal-box", "terminal", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0125), 0.095, 0.0155),
    ell("mass-housing", steel(member="housing", part="mass", tone=0.15), (0, 0, PW), (0.04, 0.04, 0.0155)),
    ell("mass-motor", steel(member="motor", part="mass", tone=0.15, bone="motor"), (0, 0, -0.0225), (0.035, 0.035, 0.0375)),
    pin_support("support-pipe-left", 0.0155, 0.031),
    roller_support("support-pipe-right", 0.0155, 0.031),
]
D200 = [
    text("Own 1:200 model: casing and motor are one stepped revolve (r 0.0455 for z -0.065..0.015, r 0.0575 for z 0.015..0.065, as the 1:200 plan and front outline), the two unions and stubs are one 0.18 m through pipe of 39 mm diameter, and the terminal box is a plain 0.06 x 0.0425 x 0.05 m block; neck, bezel, glass, lid and gland are merged.",
         "Eigenes 1:200-Modell: Gehäuse und Motor sind ein gestufter Revolve (r 0,0455 für z -0,065..0,015, r 0,0575 für z 0,015..0,065, wie im 1:200-Umriss von Grundriss und Ansicht), die zwei Verschraubungen samt Stutzen ein durchgehendes Rohr von 0,18 m und 39 mm Durchmesser, und der Klemmenkasten ein einfacher Block 0,06 x 0,0425 x 0,05 m; Hals, Blende, Glas, Deckel und Verschraubung sind zusammengefasst."),
    text("Thermal and structure at 1:200: casing and motor skins (0.8 mm outside the merged body), two pipe-end skins, the terminal block, the glass disc and the plume ring; structure keeps the four members (through pipe, stepped body, terminal block, 30 mm shaft), two mass ellipsoids and the pin and roller supports at 30 mm. Flange, neck and bearing joints are merged into the members.",
         "Thermik und Tragwerk bei 1:200: Gehäuse- und Motorhaut (0,8 mm ausserhalb des zusammengefassten Körpers), zwei Rohrendhäute, der Klemmenblock, die Glasscheibe und der Fahnenring; das Tragwerk behält die vier Stäbe (Durchgangsrohr, gestufter Körper, Klemmenblock, 30-mm-Welle), zwei Massenellipsoide sowie Fest- und Rollenlager in 30 mm. Flansch-, Hals- und Lagergelenk sind in den Stäben aufgegangen."),
]
write("circulation-pump", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
