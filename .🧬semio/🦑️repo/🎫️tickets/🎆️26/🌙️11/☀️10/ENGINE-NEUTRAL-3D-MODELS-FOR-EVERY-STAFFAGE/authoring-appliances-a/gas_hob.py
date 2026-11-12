from lib import *

PL = Ctx("hob", "steel-enamel", "plate", "body", 0.3)
BR = Ctx("burner", "brass", "burner", "body", 0.5)
KN = Ctx("knob", "bakelite", "knob", "body", 0.6)
TR = Ctx("trivet", "cast-iron", "trivet", "body", 0.45)
FM = Ctx("flame-body", "flame", "flame", "body", 0.92)
CZ = 0.03
ZK = -0.25
S = []

S.append(box("housing", PL(tone=0.4), (-0.14, 0, -0.25), (0.14, 0.031, 0.25), cr=0.002))
S.append(ext("plate", PL, rrect(-0.26, -0.15, 0.26, 0.15, 0.01), "zx", 0.03, 0.04))
vent = PL(role="detail", tone=0.3, region=None)
S.append(cap("vent-rise", vent, (-0.1, 0.006, ZK - 0.0003), (-0.08, 0.016, ZK - 0.0003), 0.0007, repeat={"count": 5, "translation": [0.04, 0, 0]}, maxd=50))
S.append(cap("vent-fall", vent, (-0.08, 0.016, ZK - 0.0003), (-0.06, 0.006, ZK - 0.0003), 0.0007, repeat={"count": 5, "translation": [0.04, 0, 0]}, maxd=50))

S.append(cyl("knob", KN, "w", (0, 0, ZK - 0.0045), 0.009, 0.011))
S.append(box("knob-pointer", KN(role="detail", tone=0.1), (-0.0012, 0, ZK - 0.0095), (0.0012, 0.011, ZK - 0.009), maxd=50))

S.append(rev("burner-ring", BR, [[0, 0.052], [0.007, 0.046]], "v", origin=(0, 0, CZ)))
S.append(rev("burner-cap", BR(tone=0.55), [[0.007, 0.02], [0.018, 0.016]], "v", origin=(0, 0, CZ)))

tongue = [(0, 0.008), (0.009, 0.0072), (0.015, 0.004), (0.02, 0.0008)]
S.append(lf("flame-ring", FM, "v", [(t, r, r, 2, CZ, -0.03) for t, r in tongue], repeat={"count": 12, "rotationAxis": [0, 1, 0], "rotationRad": math.pi / 6, "origin": [0, 0, CZ]}))
S.append(lf("flame-centre", FM(bone="flame-tip", tone=0.97), "v", [(-0.015, 0.009, 0.009, 2, CZ, 0), (-0.003, 0.008, 0.008, 2, CZ, 0), (0.005, 0.004, 0.004, 2, CZ, 0), (0.012, 0.0008, 0.0008, 2, CZ, 0)]))

S.append(ext("trivet-rail", TR, [[-0.037, -0.135], [0.097, -0.135], [0.097, 0.135], [-0.037, 0.135]], "zx", 0.038, 0.048, holes=[[[-0.025, -0.123], [0.085, -0.123], [0.085, 0.123], [-0.025, 0.123]]]))
for name, z in (("trivet-legs-front", -0.03), ("trivet-legs-back", 0.09)):
    S.append(box(name, TR, (-0.127, 0, z - 0.007), (-0.113, 0.04, z + 0.007), repeat={"count": 2, "translation": [0.24, 0, 0]}))

D = [
    text("The hob is a single-burner cooktop of 0.3 m by 0.52 m facing -z: a 0.28 m by 0.5 m housing under a 10 mm enamel plate that overhangs it, the burner (brass ring and cap, centred at x 0, z 0.03) and a four-rail trivet standing on four legs (front legs z -0.03, back legs z 0.09) from plan and front view.",
         "Die Kochmulde ist ein Einbrenner-Kochfeld von 0,3 m mal 0,52 m, nach -z gerichtet: ein Gehäuse von 0,28 m mal 0,5 m unter einer 10 mm starken Emailplatte, die darüber hinausragt, der Brenner (Messingring und -kappe, zentriert bei x 0, z 0,03) und ein vierschieniger Pfannenträger auf vier Beinen (vordere Beine z -0,03, hintere Beine z 0,09) nach Grundriss und Ansicht."),
    text("The three 2D views disagree about the knob: the elevation shows it end-on on the front panel at y 0.019 (matching the revolute bone about w), the plan a circle at z -0.2 and the front view a block on the plate at y 0.04..0.052. The physically sensible reading of the elevation is kept: a bakelite knob on the front panel (z -0.25..-0.259) at x 0.105; the plan and front drawings of it are not followed.",
         "Die drei 2D-Ansichten widersprechen sich beim Knebel: der Aufriss zeigt ihn stirnseitig an der Frontblende bei y 0,019 (passend zum Drehknochen um w), der Grundriss einen Kreis bei z -0,2 und die Ansicht einen Block auf der Platte bei y 0,04..0,052. Beibehalten ist die physikalisch sinnvolle Lesart des Aufrisses: ein Bakelitknebel an der Frontblende (z -0,25..-0,259) bei x 0,105; Grundriss und Ansicht des Knebels werden nicht gefolgt."),
    text("The flame is a ring of twelve teardrop tongues (one solid, repeated about the burner axis by 30 degrees) on the flame-body bone plus a taller centre tongue on the flame-tip bone; the prismatic flame bone slides both into the housing when the burner is off, where the housing hides them. Vent ribs, knob pointer and burner ports (not modelled) are fine detail; the vent zigzag is two ribs with repeat.",
         "Die Flamme ist ein Kranz aus zwölf tropfenförmigen Zungen (ein Körper, um die Brennerachse in 30-Grad-Schritten wiederholt) am Knochen Flammenkörper plus eine höhere Mittelzunge am Knochen Flammenspitze; der translatorische Flammenknochen schiebt beide bei ausgeschaltetem Brenner ins Gehäuse, wo es sie verdeckt. Lüftungsrippen, Knebelzeiger und Brenneröffnungen (nicht modelliert) sind feine Details; der Lüftungs-Zickzack besteht aus zwei Rippen mit repeat."),
]
write("gas-hob", S, D)
