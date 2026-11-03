from lib import *

ID = "gas-hob"
HB, BU, TR, KN, FB, FT = "hob", "burner", "trivet", "knob", "flame-body", "flame-tip"
SE, BR, CI, BK, FL = "steel-enamel", "brass", "cast-iron", "bakelite", "flame"
LB = 0.03


def sh(d, dx, dy):
    out, k = [], 0
    for t in d.split():
        if t.isalpha():
            out.append(t)
            k = 0
        else:
            out.append(f"{rn(float(t) + (dx if k % 2 == 0 else dy))}")
            k += 1
    return " ".join(out)


def at(pts, dx, dy):
    return [[x + dx, y + dy] for x, y in pts]


FLAME_L = "M -0.038 0 C -0.038 0.009 -0.0332 0.015 -0.03 0.02 C -0.0268 0.015 -0.022 0.009 -0.022 0 Z"
FLAME_R = "M 0.022 0 C 0.022 0.009 0.0268 0.015 0.03 0.02 C 0.0332 0.015 0.038 0.009 0.038 0 Z"
FLAME_C = "M -0.009 -0.015 C -0.009 -0.00285 -0.0036 0.00525 0 0.012 C 0.0036 0.00525 0.009 -0.00285 0.009 -0.015 Z"
CROWN100 = "M -0.042 0 C -0.044 0.011 -0.036 0.019 -0.028 0.02 C -0.022 0.017 -0.02 0.015 -0.012 0.023 C -0.006 0.029 -0.003 0.033 0 0.034 C 0.003 0.033 0.006 0.029 0.012 0.023 C 0.02 0.015 0.022 0.017 0.028 0.02 C 0.036 0.019 0.044 0.011 0.042 0 Z"
CROWN200 = "M -0.05 0 C -0.052 0.013 -0.038 0.025 -0.02 0.027 C -0.01 0.031 -0.004 0.037 0 0.041 C 0.004 0.037 0.01 0.031 0.02 0.027 C 0.038 0.025 0.052 0.013 0.05 0 Z"
BURNER100 = [[-0.052, 0], [0.052, 0], [0.044, 0.007], [0.02, 0.007], [0.016, 0.018], [-0.016, 0.018], [-0.02, 0.007], [-0.044, 0.007]]
BURNER200 = [[-0.056, 0], [0.056, 0], [0.046, 0.007], [0.02, 0.007], [0.016, 0.018], [-0.016, 0.018], [-0.02, 0.007], [-0.046, 0.007]]

SPEC = {
    50: {
        "front": [
            path("flame-left", FB, sh(FLAME_L, LB, 0.047), 3.4, 0.92, "flame", FL),
            path("flame-right", FB, sh(FLAME_R, LB, 0.047), 3.4, 0.92, "flame", FL),
            path("flame-centre", FT, sh(FLAME_C, LB, 0.062), 3.4, 0.97, "flame", FL),
            poly("burner-cap", BU, at([[-0.02, 0.007], [0.02, 0.007], [0.016, 0.018], [-0.016, 0.018]], LB, 0.04), 3.3, 0.55, "burner", BR),
            poly("burner-ring", BU, at([[-0.052, 0], [0.052, 0], [0.046, 0.007], [-0.046, 0.007]], LB, 0.04), 3.5, 0.5, "burner", BR),
            rect("plate", HB, -0.26, 0.03, 0.52, 0.01, 3.6, 0.3, "plate", SE, r=0.003),
            rect("housing", HB, -0.25, 0, 0.5, 0.031, 3.55, 0.4, "plate", SE),
            line("plate-lip", HB, [[-0.25, 0.012], [0.25, 0.012]], 3.7, 0.8, "plate", SE),
            rect("knob", KN, -0.211, 0.04, 0.022, 0.012, 3.8, 0.6, "knob", BK),
            rect("trivet-leg-left", TR, 0.083, 0.04, 0.014, 0.04, 3.9, 0.45, "trivet", CI),
            rect("trivet-leg-right", TR, -0.037, 0.04, 0.014, 0.04, 3.9, 0.45, "trivet", CI),
            rect("trivet-rail", TR, -0.037, 0.078, 0.134, 0.01, 3.9, 0.45, "trivet", CI),
        ],
    },
    100: {
        "front": [
            path("flame-crown", FB, sh(CROWN100, LB, 0.047), 3.4, 0.92, "flame", FL),
            poly("burner", BU, at(BURNER100, LB, 0.04), 3.5, 0.5, "burner", BR),
            rect("plate", HB, -0.26, 0, 0.52, 0.04, 3.6, 0.3, regions=["plate", "knob"], material=SE),
            rect("trivet-leg-left", TR, 0.081, 0.04, 0.016, 0.04, 3.9, 0.45, "trivet", CI),
            rect("trivet-leg-right", TR, -0.037, 0.04, 0.016, 0.04, 3.9, 0.45, "trivet", CI),
            rect("trivet-rail", TR, -0.04, 0.078, 0.14, 0.012, 3.9, 0.45, "trivet", CI),
        ],
    },
    200: {
        "front": [
            rect("plate", HB, -0.26, 0, 0.52, 0.04, 3.6, 0.3, regions=["plate", "knob"], material=SE),
            rect("burner-block", BU, -0.04, 0.04, 0.14, 0.05, 3.9, 0.45, regions=["trivet", "burner", "flame"], material=CI),
        ],
        "plan": [
            rect("plan-plate", HB, -0.15, -0.26, 0.3, 0.52, 1, 0.3, regions=["plate", "knob"], material=SE, r=0.012),
            ell("plan-burner", HB, 0, LB, 0.06, 0.06, 2, 0.5, regions=["burner", "flame", "trivet"], material=BR),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, 10.4 mm by 1.8 mm on paper: the whole width of the 0.52 m hob with its low housing, the plate with its lip line, the burner standing off-centre with ring, cap and the two side flames and the centre flame, the knob as a small cylinder on the right edge, and the pan support as two legs and a rail across the front.",
            "de": "Vorderansicht, 10,4 mm auf 1,8 mm: die ganze Breite des 0,52 m tiefen Kochfelds mit niedrigem Gehäuse, die Platte mit Kantenlinie, der außermittig stehende Brenner mit Ring, Kappe sowie den beiden Seitenflammen und der Mittelflamme, der Knebel als kleiner Zylinder am rechten Rand und der Topfträger als zwei Beine mit einer Schiene davor.",
        },
        "omitted": {
            "en": "In the front view the knob pointer is left out; the flames sit on the flame bones and drop out of sight under the burner ring when the hob is off.",
            "de": "In der Vorderansicht entfällt der Knebelzeiger; die Flammen sitzen an den Flammenknochen und sinken bei ausgeschaltetem Feld unter den Brennerring.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 5.2 mm by 0.9 mm: plate as one strip, burner wedge with the flame crown above it and the pan support as two legs and a rail.",
            "de": "Vorderansicht, 5,2 mm auf 0,9 mm: die Platte als ein Streifen, der Brennerkeil mit der Flammenkrone darüber und der Topfträger als zwei Beine mit Schiene.",
        },
        "omitted": {
            "en": "Lip line, knob and the separate housing in the front view.",
            "de": "Kantenlinie, Knebel und das getrennte Gehäuse in der Vorderansicht.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 2.6 mm by 0.45 mm: a plate strip and one block that carries burner, flame and pan support together. Plan, 1.5 mm by 2.6 mm: the plate and one disc in which burner, flame ring and pan support are merged. In the side view the burner is drawn 0.03 m high (0.15 mm on paper plus the line) instead of 0.018 m so that it stays a printable 0.2 mm feature; the front block and the plan disc are already wider than that.",
            "de": "Vorderansicht, 2,6 mm auf 0,45 mm: ein Plattenstreifen und ein Block, der Brenner, Flamme und Topfträger zusammen trägt. Grundriss, 1,5 mm auf 2,6 mm: die Platte und eine Scheibe, in der Brenner, Flammenring und Topfträger verschmolzen sind. In der Seitenansicht ist der Brenner 0,03 m hoch gezeichnet (0,15 mm auf dem Papier plus Linie) statt 0,018 m, damit er ein druckbares 0,2-mm-Detail bleibt; Frontblock und Grundrissscheibe sind bereits breiter.",
        },
        "omitted": {
            "en": "Knob, burner cap and the frame outline in the plan; the plate is a single strip in front and burner, flame and pan support share one block.",
            "de": "Knebel, Brennerkappe und Rahmenumriss im Grundriss; die Platte ist vorn ein einziger Streifen, Brenner, Flamme und Topfträger teilen sich einen Block.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
