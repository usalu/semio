import sys, os, math
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
C, BA = "column", "balustrade"
STEEL = "steel"
DEG = math.pi / 180
RI, RO, T, LIP = 0.0842, 0.8, 0.005, 0.005
pt = lambda r, a: (r * math.cos(a), r * math.sin(a))
def sector(th0, th1, r0, r1, n=6):
    arc = [pt(r1, th0 + (th1 - th0) * k / n) for k in range(n + 1)]
    return arc + [pt(r0, th1), pt(r0, th0)]
s = [
 L.ext("column", C, STEEL, "body", 0.5, [(0.0842 * math.cos(2 * math.pi * k / 48), 0.0842 * math.sin(2 * math.pi * k / 48)) for k in range(48)], "yz", 0, 2.8, "column-steel",
       holes=[[(0.0779 * math.cos(2 * math.pi * k / 48), 0.0779 * math.sin(2 * math.pi * k / 48)) for k in range(48)]]),
 L.rev("base-plate", C, STEEL, "body", 0.5, [(0, 0.2), (0.02, 0.2)], [1, 0, 0], [0, 0, 0], region="column-steel"),
]
for n in range(1, 16):
    th0, th1 = (n - 1) * 22.5 * DEG, n * 22.5 * DEG
    outline = sector(th0, th1, RI, RO)
    pa = lambda p: (p[1], p[0])
    d0, d1 = math.asin(LIP / RI * 0 + LIP / 0.0892), 0
    lo0, hi0 = th0 + math.asin(LIP / 0.795), th1 - math.asin(LIP / 0.795)
    li, hi_ = th0 + math.asin(LIP / 0.0892), th1 - math.asin(LIP / 0.0892)
    hole = [pt(0.795, lo0 + (hi0 - lo0) * k / 6) for k in range(7)] + [pt(0.0892, hi_), pt(0.0892, li)]
    P = "tread-%d" % n
    s.append(L.ext("tread-top-%d" % n, P, STEEL, "body", 0.5, [pa(p) for p in outline], "zx", -T, 0, "treads"))
    s.append(L.ext("tread-lip-%d" % n, P, STEEL, "body", 0.5, [pa(p) for p in outline], "zx", -0.06, -T, "treads", holes=[[pa(p) for p in hole]]))
R = 0.775
mid = 11.25 * DEG
s.append(L.cap("baluster", BA, STEEL, "detail", 0.3, [R * math.cos(mid), 0.18667, R * math.sin(mid)], [R * math.cos(mid), 0.18667 + 0.93333, R * math.sin(mid)], 0.014, region="balustrade-steel",
               repeat={"count": 15, "rotationAxis": [0, 1, 0], "rotationRad": round(-22.5 * DEG, 7), "translation": [0, 0.18667, 0]}))
seg = 4.5 * DEG
dy = 2.61333 / 70
s.append(L.cap("handrail", BA, STEEL, "detail", 0.3, [R * math.cos(mid), 1.12, R * math.sin(mid)], [R * math.cos(mid + seg), 1.12 + dy, R * math.sin(mid + seg)], 0.014, region="balustrade-steel",
               repeat={"count": 70, "rotationAxis": [0, 1, 0], "rotationRad": round(-seg, 7), "translation": [0, dy, 0]}))
L.write("spiral-stair", s, [
 ("Each of the 15 treads is a pressed steel pan on its own tread bone: a 5 mm top plate and a 55 mm deep 5 mm lip ring (extrude with a hole), both from the 22.5 degree pie outline of the plan, rotated 22.5 degrees per step (tread n spans (n-1) x 22.5 to n x 22.5 degrees counter-clockwise from above, tread 15 ends at -22.5 degrees); this matches the 8 kg tread mass of a hollow pan.", "Jede der 15 Stufen ist eine gepresste Stahlwanne an ihrem eigenen Stufenknochen: eine 5 mm Deckplatte und ein 55 mm tiefer 5 mm Randring (Extrusion mit Loch), beide aus dem 22,5-Grad-Kreisringsektor der Draufsicht, pro Stufe um 22,5 Grad gedreht (Stufe n überdeckt (n-1) x 22,5 bis n x 22,5 Grad gegen den Uhrzeigersinn von oben, Stufe 15 endet bei -22,5 Grad); das passt zur Stufenmasse von 8 kg einer hohlen Wanne."),
 ("The centre column is a hollow CHS 168.3 x 6.3 as a 48-sided extrude with a hole on a 400 x 20 mm revolve base plate; sleeve lines are 2D texture. The 15 balusters (Ø28 mm capsules at radius 0.775 m) are one solid with a repeat that rotates 22.5 degrees and rises 0.18667 m per copy; the handrail is a helix of 70 capsule segments made the same way (4.5 degrees and 37.3 mm per copy), its radius of 14 mm is assumed (the 2D draws only a line).", "Die Mittelsäule ist ein Hohlprofil CHS 168,3 x 6,3 als 48-seitige Extrusion mit Loch auf einer Fussplatte 400 x 20 mm (Rotationskörper); Muffenlinien sind 2D-Textur. Die 15 Stäbe (Kapseln Ø28 mm auf Radius 0,775 m) sind ein Körper mit einer Wiederholung, die pro Kopie um 22,5 Grad dreht und 0,18667 m steigt; der Handlauf ist eine Wendel aus 70 Kapselsegmenten auf dieselbe Art (4,5 Grad und 37,3 mm pro Kopie), sein Radius von 14 mm ist angenommen (die 2D-Ansicht zeichnet nur eine Linie)."),
 ("The walker bones (turn and two prismatic) are load points without volume; walking line, arrow and start marker are pictograms and not modelled.", "Die Läuferknochen (Drehung und zwei Schubgelenke) sind Lastpunkte ohne Volumen; Gehlinie, Pfeil und Startpunkt sind Piktogramme und nicht modelliert."),
])
