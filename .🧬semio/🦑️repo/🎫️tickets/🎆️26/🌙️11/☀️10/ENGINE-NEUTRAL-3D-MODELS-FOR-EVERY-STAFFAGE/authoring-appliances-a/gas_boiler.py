from lib import *

CS = Ctx("casing", "painted-steel", "casing", "body", 0.88)
CT = Ctx("casing", "polycarbonate", "controls", "body", 0.35)
SG = Ctx("casing", "sight-glass", "sight-glass", "body", 0.55)
BN = Ctx("casing", "burner-mesh", "burner", "body", 0.45)
FL = Ctx("collar", "flue-pipe", "flue-collar", "body", 0.7)
CN = Ctx("connections", "brass", "connections", "body", 0.7)
ZF = -0.155
ZP = -0.13
S = []
WIN = (0, 0.46)

S.append(box("casing", CS, (-0.22, 0.05, ZP), (0.22, 0.67, 0.175), cr=0.016))
S.append(ext("front-plate", CS, rrect(-0.22, 0.05, 0.22, 0.67, 0.014), "xy", ZF, ZP, holes=[ngon(0.0372, 40, *WIN)]))
S.append(ext("door", CS(role="detail", tone=0.4), rrect(-0.205, 0.225, 0.205, 0.655, 0.012), "xy", ZF - 0.0015, ZF, holes=[ngon(0.0445, 40, *WIN)]))
S.append(box("slits", CS(role="detail", tone=0.4, region=None), (-0.092, 0.628, ZF - 0.003), (-0.088, 0.652, ZF - 0.0015), repeat={"count": 4, "translation": [0.06, 0, 0]}, maxd=50))
S.append(box("logo-1", CS(role="detail", tone=0.4, region=None), (-0.06, 0.5895, ZF - 0.002), (0.06, 0.5905, ZF - 0.0015), maxd=50))
S.append(box("logo-2", CS(role="detail", tone=0.4, region=None), (-0.04, 0.5745, ZF - 0.002), (0.04, 0.5755, ZF - 0.0015), maxd=50))
S.append(cyl("led", CS(role="detail", tone=0.95, region=None), "w", (0.17, 0.245, ZF - 0.001), 0.001, 0.006, maxd=50))

S.append(pipe("bezel", SG, "w", (0, 0.46, -0.1585), 0.007, 0.0445, 0.0372, n=40))
S.append(cyl("sight-glass", SG(role="glazing", tone=0.2), "w", (0, 0.46, -0.1545), 0.002, 0.0372))
S.append(cyl("chamber", CS(region=None, tone=0.1), "w", (0, 0.46, -0.1305), 0.001, 0.0372))
S.append(box("burner-tube", BN, (-0.026, 0.43, -0.139), (0.026, 0.441, -0.136), cr=0.0014))
fl = ((-0.024, 0), (-0.022, 0.011), (-0.015, 0.022), (-0.009, 0.011), (0, 0.032), (0.009, 0.011), (0.015, 0.022), (0.022, 0.011), (0.024, 0))
S.append(ext("flame-tall", BN(bone="flame-tall", role="detail", tone=0.75), [list(p) for p in fl], "xy", -0.1415, -0.1395))
fs = ((-0.018, 0), (-0.018, 0.006), (-0.012, 0.014), (-0.007, 0.008), (0, 0.017), (0.007, 0.008), (0.012, 0.014), (0.018, 0.006), (0.018, 0))
S.append(ext("flame", BN(bone="flame", role="detail", tone=0.95), [list(p) for p in fs], "xy", -0.1435, -0.1415))

S.append(box("panel", CT, (-0.205, 0.065, -0.165), (0.205, 0.21, ZF), cr=0.004))
S.append(box("display", CT(role="detail", tone=0.95), (-0.17, 0.125, -0.1655), (-0.06, 0.185, -0.165)))
for name, p, q in (("tick-off", (0.0065, 0.1341), (-0.0014, 0.1327)), ("tick-heat", (0.04, 0.174), (0.04, 0.182)), ("tick-water", (0.074, 0.14), (0.082, 0.14))):
    S.append(cap(name, CT(role="detail", tone=0.4, region=None), (p[0], p[1], -0.1655), (q[0], q[1], -0.1655), 0.0007, maxd=50))
S.append(cyl("knob", CT(bone="mode-knob", region=None, tone=0.72), "w", (0, 0, -0.17), 0.01, 0.026))
S.append(box("pointer", CT(bone="mode-knob", region=None, role="detail", tone=0.2), (-0.0015, 0, -0.1755), (0.0015, 0.022, -0.175), maxd=50))
S.append(cyl("set-knob", CT(region=None, tone=0.72), "w", (0.12, 0.14, -0.1675), 0.005, 0.018))
S.append(box("set-notch", CT(region=None, role="detail", tone=0.2), (0.1185, 0.14, -0.1705), (0.1215, 0.154, -0.17), maxd=50))
S.append(cyl("gauge", SG(region=None, role="detail", tone=0.92), "w", (0.173, 0.14, -0.1655), 0.001, 0.016, maxd=50))
S.append(cap("gauge-needle", CT(region=None, role="detail", tone=0.2), (0.173, 0.14, -0.1668), (0.181, 0.149, -0.1668), 0.0008, maxd=50))

S.append(rev("collar-base", FL, [[0, 0.0585], [0.004, 0.0625], [0.026, 0.0625], [0.03, 0.0585]], "v", origin=(0, 0, 0.04)))
S.append(pipe("collar-pipe", FL(tone=0.55), "v", (0, 0.04, 0.04), 0.02, 0.04, 0.032, n=40))

S.append(cyl("stubs", CN, "v", (-0.16, 0.026, 0.07), 0.052, 0.011, repeat={"count": 5, "translation": [0.08, 0, 0]}))
S.append(ext("nuts", CN(tone=0.45), hexagon(0.07, -0.16, 0.0196, 0), "zx", 0.018, 0.038, repeat={"count": 5, "translation": [0.08, 0, 0]}))

D = [
    text("The boiler is wall-hung and faces -z: plan and front give the depth stack (front plate to z -0.155, control panel to -0.165, knobs to -0.175, rear to +0.175). The case is a rounded box plus a 25 mm front plate with a round hole, so the burner chamber behind the sight glass is a real recess in which burner tube and flames sit; the retracting flame bones therefore slide into the front plate and are hidden when the burner is off.",
         "Der Kessel ist wandhängend und blickt nach -z: Grundriss und Ansicht geben den Tiefenaufbau (Frontplatte bis z -0,155, Bedienfeld bis -0,165, Knöpfe bis -0,175, Rückseite bei +0,175). Das Gehäuse ist ein abgerundeter Quader plus eine 25 mm Frontplatte mit rundem Loch, sodass die Brennkammer hinter dem Schauglas eine echte Vertiefung ist, in der Brennerrohr und Flammen sitzen; die Flammenknochen fahren daher in die Frontplatte und sind bei ausgeschaltetem Brenner verdeckt."),
    text("The five brass connections stand in one row along x at z = 0.07 (front view) with hex nuts; the flue collar is a base disc with a hollow pipe whose 0.032 m bore is the drawn dark opening, centred at z = 0.04 on top of the case.",
         "Die fünf Messinganschlüsse stehen in einer Reihe entlang x bei z = 0,07 (Seitenansicht) mit Sechskantmuttern; der Abgaskragen ist eine Grundscheibe mit Hohlrohr, dessen Bohrung von 0,032 m die gezeichnete dunkle Öffnung ist, bei z = 0,04 auf der Gehäuseoberseite."),
    text("The 2D door is only an outline: it becomes a 1.5 mm door plate with a hole for the sight glass bezel. Slits, logo bars, LED, display text, ticks, pointer, set-knob notch and gauge are fine relief (maxDenominator 50); the two display text lines and the plan top-edge line are not modelled. The mode knob (revolute) carries knob and pointer; flames and flame-tall are prismatic bones.",
         "Die 2D-Tür ist nur eine Kontur: sie wird eine 1,5 mm starke Türplatte mit Loch für die Schauglasblende. Schlitze, Logobalken, LED, Anzeigetext, Marken, Zeiger, Einstellknopfkerbe und Manometer sind feines Relief (maxDenominator 50); die zwei Anzeigetextzeilen und die Randlinie im Grundriss sind nicht modelliert. Der Modusknopf (Drehgelenk) trägt Knopf und Zeiger; Flamme und Flamme-hoch sind translatorische Knochen."),
]
write("gas-boiler", S, D)
