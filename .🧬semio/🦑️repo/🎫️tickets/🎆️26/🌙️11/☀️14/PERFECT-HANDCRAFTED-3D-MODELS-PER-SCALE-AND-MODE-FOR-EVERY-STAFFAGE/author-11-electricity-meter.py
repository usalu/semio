from author_11_lib import *

CS = Ctx("body", "abs-plastic", None, "body", 0.88)
WN = Ctx("body", "polycarbonate", None, "body", 0.1)
PL = Ctx("body", "abs-plastic", None, "body", 0.78)
DK = Ctx("disc", "aluminium-disc", None, "texture", 0.86)
steel = Ctx("body", "abs-plastic", None, "detail", 0.25)
ZF, ZB = -0.058, 0.035


def support(aid, kind, y, sz, ln):
    c = steel(member="body", part="support")
    s = cyl("support-" + aid, c, "w" if kind == "pin" else "u", (0, y, 0.05), ln, sz)
    return tag(s, tags=[aid])


def reg(s, r):
    s["region"] = r
    return s


TCAS, TWIN, TDISC, TPLATE = CS(layer="shell"), WN(layer="pane"), DK(layer="sheet", role="texture"), PL(layer="plate")

# ------------------------------------------------------------------ 1:50
ZR = -0.066
body50 = [
    box("back-plate", CS(tone=0.78), (-0.085, -0.1425, ZB), (0.085, 0.1425, 0.065), cr=0.004),
    box("body", CS, (-0.085, -0.1, ZF), (0.085, 0.1425, ZB), cr=0.012),
    box("terminals", CS(tone=0.55), (-0.072, -0.1425, ZF), (0.072, -0.1, ZB), cr=0.004),
    ell("screws", CS(role="detail", tone=0.2), (-0.058, -0.121, ZF - 0.0005), (0.005, 0.005, 0.004), repeat={"count": 2, "translation": [0.116, 0, 0]}),
    ell("eye", CS(role="detail", tone=0.15), (0, 0.123, ZF - 0.0005), (0.008, 0.008, 0.004)),
    pipe("rim", WN(tone=0.25), "w", (0, 0.01, -0.0616), 0.0078, 0.05, 0.0465, n=48),
    cyl("window-glass", WN(role="glazing"), "w", (0, 0.01, -0.0645), 0.0015, 0.0465),
    box("counter", WN, (-0.045, 0.078, ZR), (0.045, 0.104, ZF - 0.0005), cr=0.003),
    box("plate", PL, (-0.065, -0.085, ZR), (0.065, -0.045, ZF - 0.0005), cr=0.003),
    cyl("disc", DK, "w", (0, 0, -0.061), 0.001, 0.042),
    ell("hub", DK(role="detail", tone=0.3), (0, 0, -0.0615), (0.006, 0.006, 0.004)),
    box("mark", DK(role="texture", tone=0.1), (0.02, -0.005, -0.0618), (0.04, 0.005, -0.0609)),
]
th50 = [
    th("th-body", "body", "casing", "shell"),
    th("th-back-plate", "back-plate", "casing", "shell"),
    th("th-terminals", "terminals", "casing", "shell"),
    th("th-rim", "rim", "window", "pane"),
    th("th-window-glass", "window-glass", "window", "pane"),
    th("th-counter", "counter", "window", "pane"),
    th("th-disc", "disc", "disc", "sheet"),
    th("th-plate", "plate", "plate", "plate"),
]
st50 = [
    stm("member-body", "body", "body", "member"),
    stm("member-back-plate", "back-plate", "body", "member"),
    cyl("member-disc", steel(member="disc", part="member", bone="disc"), "w", (0, 0, -0.03), 0.06, 0.006),
    cyl("joint-disc", steel(member="disc", part="joint", bone="disc"), "w", (0, 0, -0.0535), 0.0085, 0.01),
    ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, 0, 0.0), (0.06, 0.09, 0.025)),
    support("wall-screw-top", "pin", 0.123, 0.008, 0.03),
    support("wall-screw-bottom", "roller", -0.121, 0.008, 0.016),
]
D50 = [
    text("Own 1:50 model from the three drawings, depth stack from plan and front: a 30 mm back plate (z 0.035..0.065), a 93 mm housing with 12 mm rounded edges, a terminal block below it, and an 8 mm raised front (z -0.066..-0.058) with the 0.1 m window ring, the counter window and the name plate; the Ferraris disc (1 mm sheet, 0.084 m, role texture) lies between glass and housing on the revolute disc bone with a domed hub and a 1 mm mark; two terminal screw heads and the optical eye are domed lenses.",
         "Eigenes 1:50-Modell aus den drei Zeichnungen, Tiefenaufbau aus Grundriss und Ansicht: eine Rückplatte von 30 mm (z 0,035..0,065), ein 93 mm tiefes Gehäuse mit 12 mm gerundeten Kanten, darunter der Klemmenblock und eine 8 mm erhabene Front (z -0,066..-0,058) mit dem Fensterring von 0,1 m, dem Zählwerkfenster und dem Typenschild; die Ferraris-Scheibe (1-mm-Blech, 0,084 m, Rolle texture) liegt zwischen Glas und Gehäuse am Drehknochen Scheibe mit gewölbter Nabe und einer 1-mm-Marke; zwei Klemmenschraubenköpfe und das optische Auge sind gewölbte Linsen."),
    text("The 0.5 mm printed relief (digits, name-plate lines) and the plan seam lines are below the 7.5 mm feature and merged. Thermal (area = exposed surface): casing = housing, back plate and terminal block, window = ring, glass and counter, disc = the sheet, plate = the name plate, all by reference; areaM2 was corrected at the root where the old estimate disagreed with the model (see report). The wall-screw anchors got z 0.05 (middle of the back plate).",
         "Der 0,5-mm-Reliefdruck (Ziffern, Typenschildlinien) und die Fugenlinien im Grundriss liegen unter dem 7,5-mm-Merkmal und sind zusammengefasst. Thermik (Fläche = freie Oberfläche): Gehäuse = Gehäuse, Rückplatte und Klemmenblock, Fenster = Ring, Glas und Zählwerk, Scheibe = das Blech, Schild = das Typenschild, alles als Verweis; areaM2 wurde an der Wurzel korrigiert, wo die alte Schätzung dem Modell widersprach (siehe Bericht). Die Wandschrauben-Anker erhielten z 0,05 (Mitte der Rückplatte)."),
    text("Structure: housing and back plate are the body members, a 12 mm spindle through the housing is the disc member with a 20 mm bearing as its joint, one mass ellipsoid carries the 0.85 kg, and the two wall screws are a 16 mm cross pin (top) and a roller bar (bottom) in the back plate.",
         "Tragwerk: Gehäuse und Rückplatte sind die Körperstäbe, eine 12-mm-Spindel durch das Gehäuse ist der Scheibenstab mit einem Lager von 20 mm als Gelenk, ein Massenellipsoid trägt die 0,85 kg, und die zwei Wandschrauben sind ein 16-mm-Querbolzen (oben) und eine Rollenstange (unten) in der Rückplatte."),
]

# ------------------------------------------------------------------ 1:100
ZR1 = -0.0735
body100 = [
    box("back-plate", CS(tone=0.78), (-0.085, -0.1425, ZB), (0.085, 0.1425, 0.065)),
    box("body", CS, (-0.085, -0.1, ZF), (0.085, 0.1425, ZB), cr=0.0),
    box("terminals", CS(tone=0.55), (-0.072, -0.1425, ZF), (0.072, -0.1, ZB)),
    pipe("rim", WN(tone=0.25), "w", (0, 0.01, -0.06575), 0.0155, 0.05, 0.0465, n=40),
    cyl("window-glass", WN(role="glazing"), "w", (0, 0.01, -0.0725), 0.0015, 0.0465),
    box("counter", WN, (-0.045, 0.076, ZR1), (0.045, 0.106, ZF - 0.0005)),
    box("plate", PL, (-0.065, -0.085, ZR1), (0.065, -0.045, ZF - 0.0005)),
    cyl("disc", DK, "w", (0, 0, -0.061), 0.001, 0.042),
    box("mark", DK(role="texture", tone=0.1), (0.012, -0.01, -0.0618), (0.042, 0.01, -0.0609)),
]
th100 = [
    th("th-body", "body", "casing", "shell"),
    th("th-back-plate", "back-plate", "casing", "shell"),
    th("th-terminals", "terminals", "casing", "shell"),
    th("th-rim", "rim", "window", "pane"),
    th("th-counter", "counter", "window", "pane"),
    th("th-disc", "disc", "disc", "sheet"),
    th("th-plate", "plate", "plate", "plate"),
]
st100 = [
    stm("member-body", "body", "body", "member"),
    stm("member-back-plate", "back-plate", "body", "member"),
    cyl("member-disc", steel(member="disc", part="member", bone="disc"), "w", (0, 0, -0.03), 0.06, 0.008),
    cyl("joint-disc", steel(member="disc", part="joint", bone="disc"), "w", (0, 0, -0.0525), 0.0155, 0.0105),
    ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, 0, 0.0), (0.06, 0.09, 0.025)),
    support("wall-screw-top", "pin", 0.123, 0.008, 0.03),
    support("wall-screw-bottom", "roller", -0.121, 0.008, 0.0165),
]
D100 = [
    text("Own 1:100 model against the 1:100 shapes: plain boxes without edge rounding, a 15.5 mm window ring, the counter and the name plate as 15 mm raised blocks (z -0.0735..-0.058, 8 mm more than drawn because 15 mm is the feature), the glass sheet, the disc sheet and a mark; screws, eye, hub and digits are merged away.",
         "Eigenes 1:100-Modell gegen die 1:100-Formen: schlichte Quader ohne Kantenrundung, ein 15,5-mm-Fensterring, Zählwerk und Typenschild als 15 mm erhabene Blöcke (z -0,0735..-0,058, 8 mm mehr als gezeichnet, weil 15 mm das Merkmal ist), das Glasblech, das Scheibenblech und eine Marke; Schrauben, Auge, Nabe und Ziffern sind zusammengefasst."),
    text("Thermal and structure rebuilt by reference on these solids; the spindle is 16 mm, one 20 mm bearing ring is the joint, one mass ellipsoid and the two wall screws at 16 mm.",
         "Thermik und Tragwerk als Verweis auf diese Körper neu aufgebaut; die Spindel misst 16 mm, ein Lagerring von 20 mm ist das Gelenk, dazu ein Massenellipsoid und die zwei Wandschrauben in 16 mm."),
]

# ------------------------------------------------------------------ 1:200
body200 = [
    box("body", CS, (-0.085, -0.1425, -0.045), (0.085, 0.1425, 0.065)),
    box("terminals", CS(tone=0.55), (-0.072, -0.1425, ZF), (0.072, -0.1, ZB)),
    cyl("boss", WN(tone=0.25), "w", (0, 0.01, -0.05), 0.03, 0.052),
    box("counter", WN, (-0.045, 0.07, -0.065), (0.045, 0.11, -0.035)),
    cyl("disc", DK, "w", (0, 0, -0.0662), 0.001, 0.042),
    box("mark", DK(role="texture", tone=0.1), (0.008, -0.02, -0.0667), (0.044, 0.02, -0.0658)),
]
th200 = [
    th("th-body", "body", "casing", "shell"),
    th("th-terminals", "terminals", "casing", "shell"),
    th("th-boss", "boss", "window", "pane"),
    th("th-counter", "counter", "window", "pane"),
    th("th-disc", "disc", "disc", "sheet"),
    reg(box("th-plate", TPLATE, (-0.07, -0.1425, -0.0585), (0.07, -0.1125, 0.0)), "plate"),
]
st200 = [
    stm("member-body", "body", "body", "member"),
    cyl("member-disc", steel(member="disc", part="member", bone="disc"), "w", (0, 0, -0.03), 0.06, 0.0155),
    ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, 0, 0.0), (0.06, 0.09, 0.0305)),
    support("wall-screw-top", "pin", 0.123, 0.0155, 0.031),
    support("wall-screw-bottom", "roller", -0.121, 0.0155, 0.031),
]
D200 = [
    text("Own 1:200 model: one 110 mm housing block that includes the back plate, the terminal block, a 30 mm round window boss of 0.104 m, a 30 mm counter block and the disc as a thin sheet with a mark hovering 0.7 mm in front of the boss face (so that both of its faces are exposed as at the finer scales); fewer solids, edges, ring, glass, plate, screws, eye and hub are merged.",
         "Eigenes 1:200-Modell: ein 110-mm-Gehäuseblock mit eingeschlossener Rückplatte, der Klemmenblock, ein 30 mm hoher runder Fensterbuckel von 0,104 m, ein 30-mm-Zählwerkblock und die Scheibe als dünnes Blech mit Marke, 0,7 mm vor der Buckelfläche schwebend (damit wie bei den feineren Massstäben beide Flächen frei sind); weniger Körper, Kanten, Ring, Glas, Schild, Schrauben, Auge und Nabe sind zusammengefasst."),
    text("Thermal and structure at 1:200: body, terminal block, boss, counter and disc by reference, the name plate as a 30 mm skin block inside the terminal block; members are the housing and a 31 mm spindle, one mass ellipsoid and the two wall screws at 31 mm; the joint and the back plate member are merged.",
         "Thermik und Tragwerk bei 1:200: Gehäuse, Klemmenblock, Buckel, Zählwerk und Scheibe als Verweis, das Typenschild als 30-mm-Hautblock im Klemmenblock; Stäbe sind das Gehäuse und eine 31-mm-Spindel, ein Massenellipsoid und die zwei Wandschrauben in 31 mm; das Gelenk und der Rückplattenstab sind zusammengefasst."),
]
write("electricity-meter", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
