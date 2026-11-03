from lib import *

ID = "gas-meter"
B = "body"
DC, AB, SP = "die-cast-aluminium", "abs-plastic", "steel-pipe"
LC = 0.0075

SPEC = {
    50: {
        "front": [
            rect("stub", B, LC - 0.014, 0.11, 0.028, 0.044, 2.9, 0.5, "pipes", SP),
            rect("union", B, LC - 0.02, 0.118, 0.04, 0.018, 3, 0.35, "pipes", SP, r=0.002),
            rect("face", B, LC - 0.017, 0.154, 0.034, 0.006, 3.05, 0.4, "pipes", SP, role="detail"),
            rect("lower-shell", B, -0.07, -0.11, 0.155, 0.11, 3, 0.55, "shell", DC, r=0.012),
            rect("upper-shell", B, -0.07, 0, 0.155, 0.11, 3.01, 0.6, "shell", DC, r=0.012),
            rect("seam", B, -0.07, -0.006, 0.155, 0.012, 3.1, 0.3, "shell", DC, role="detail"),
            rect("index", B, -0.085, 0, 0.015, 0.098, 3.2, 0.8, "index", AB, r=0.006),
            line("index-edge", B, [[-0.078, 0.058], [-0.078, 0.087]], 3.3, 0.1, "index", AB),
        ],
        "plan": [
            rect("housing", B, -0.12, -0.07, 0.24, 0.155, 3, 0.6, "shell", DC, r=0.012),
            poly("bead", B, [[-0.108, -0.058], [0.108, -0.058], [0.108, 0.073], [-0.108, 0.073]], 3.1, 0.3, "shell", DC, role="detail", filled=False),
            rect("index", B, -0.095, -0.085, 0.19, 0.015, 3.2, 0.8, "index", AB, r=0.006),
            ell("union-1", B, -0.055, LC, 0.02, 0.02, 3, 0.35, "pipes", SP),
            ell("stub-1", B, -0.055, LC, 0.014, 0.014, 3.1, 0.5, "pipes", SP),
            ell("bore-1", B, -0.055, LC, 0.009, 0.009, 3.2, 0.12),
            ell("union-2", B, 0.055, LC, 0.02, 0.02, 3, 0.35, "pipes", SP),
            ell("stub-2", B, 0.055, LC, 0.014, 0.014, 3.1, 0.5, "pipes", SP),
            ell("bore-2", B, 0.055, LC, 0.009, 0.009, 3.2, 0.12),
        ],
    },
    100: {
        "front": [
            rect("stub", B, LC - 0.014, 0.11, 0.028, 0.044, 2.9, 0.5, "pipes", SP),
            rect("union", B, LC - 0.02, 0.118, 0.04, 0.018, 3, 0.35, "pipes", SP),
            rect("housing", B, -0.07, -0.11, 0.155, 0.22, 3, 0.58, "shell", DC),
            rect("index", B, -0.085, 0, 0.015, 0.098, 3.2, 0.8, "index", AB),
        ],
        "plan": [
            rect("housing", B, -0.12, -0.07, 0.24, 0.155, 3, 0.58, "shell", DC),
            rect("index", B, -0.095, -0.085, 0.19, 0.015, 3.2, 0.8, "index", AB),
            ell("union-1", B, -0.055, LC, 0.02, 0.02, 3, 0.35, "pipes", SP),
            ell("union-2", B, 0.055, LC, 0.02, 0.02, 3, 0.35, "pipes", SP),
        ],
    },
    200: {
        "front": [
            rect("stub", B, LC - 0.02, 0.1, 0.04, 0.06, 2.9, 0.45, "pipes", SP),
            poly("housing", B, [[-0.07, -0.11], [0.085, -0.11], [0.085, 0.11], [-0.07, 0.11], [-0.07, 0.098], [-0.085, 0.098], [-0.085, -0.02], [-0.07, -0.02]], 3, 0.6, regions=["shell", "index"], material=DC),
        ],
        "plan": [
            poly("meter", B, [[-0.12, -0.07], [-0.095, -0.07], [-0.095, -0.085], [0.095, -0.085], [0.095, -0.07], [0.12, -0.07], [0.12, 0.085], [-0.12, 0.085]], 3, 0.6, regions=["shell", "index"], material=DC),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, 3.4 mm by 5.4 mm on paper: the meter side-on, the two die-cast shells meeting at a bead seam, the pale index head standing out of the face, and the two pipe stubs with union nut on top, one behind the other. Plan, 4.8 mm by 3.4 mm: the housing with the seam bead running round it, the index strip on the front edge and the two union nuts with their stubs and dark bores.",
            "de": "Vorderansicht, 3,4 mm auf 5,4 mm: der Zähler von der Seite, die beiden Druckguss-Schalen mit der Wulstnaht, der helle Zählwerkkopf, der aus der Front tritt, und die beiden Rohrstutzen mit Überwurfmutter obenauf, einer hinter dem anderen. Grundriss, 4,8 mm auf 3,4 mm: das Gehäuse mit der umlaufenden Wulstnaht, der Zählwerkstreifen an der Vorderkante und die beiden Überwurfmuttern mit Stutzen und dunklen Bohrungen.",
        },
        "omitted": {
            "en": "Digits, dial, needle and data plate lie on the face and are not seen from the side or from above.",
            "de": "Ziffern, Zifferblatt, Zeiger und Typenschild liegen auf der Front und sind von der Seite und von oben nicht zu sehen.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 1.7 mm by 2.7 mm: housing, index head, one stub and one union. Plan, 2.4 mm by 1.7 mm: housing, index strip and two union discs.",
            "de": "Vorderansicht, 1,7 mm auf 2,7 mm: Gehäuse, Zählwerkkopf, ein Stutzen und eine Überwurfmutter. Grundriss, 2,4 mm auf 1,7 mm: Gehäuse, Zählwerkstreifen und zwei Muttern-Scheiben.",
        },
        "omitted": {
            "en": "Seam bead, bores and stub rings.",
            "de": "Wulstnaht, Bohrungen und Stutzenringe.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 0.85 mm by 1.35 mm: the housing and index step merged into one outline with the thick pipe stub on top. Plan, 1.2 mm by 0.85 mm: one outline of housing and index strip; the two stubs are below what the pen can draw and are left out.",
            "de": "Vorderansicht, 0,85 mm auf 1,35 mm: Gehäuse und Zählwerkstufe zu einem Umriss verschmolzen, darauf der dicke Rohrstutzen. Grundriss, 1,2 mm auf 0,85 mm: ein Umriss aus Gehäuse und Zählwerkstreifen; die beiden Stutzen liegen unter dem, was der Stift zeichnen kann, und entfallen.",
        },
        "omitted": {
            "en": "Union nuts and the dial are merged away; in plan the stubs are left out and the outline carries shell and index.",
            "de": "Überwurfmuttern und Zifferblatt entfallen; im Grundriss entfallen die Stutzen und der Umriss trägt Gehäuse und Zählwerk.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
