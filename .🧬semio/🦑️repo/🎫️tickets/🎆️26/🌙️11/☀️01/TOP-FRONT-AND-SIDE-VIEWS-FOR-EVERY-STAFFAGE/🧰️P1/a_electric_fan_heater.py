from lib import *

ID = "electric-fan-heater"
B, K = "body", "knob"
AC, PC, SM, PD, RF = "abs-cream", "pc-heat", "steel-mesh", "pp-dial", "rubber-foot"

SPEC = {
    50: {
        "front": [
            rect("foot-left", B, 0.07, 0, 0.03, 0.016, 2.8, 0.15, "feet", RF),
            rect("foot-right", B, -0.1, 0, 0.03, 0.016, 2.8, 0.15, "feet", RF),
            rect("handle", B, -0.065, 0.268, 0.13, 0.042, 2.9, 0.75, "casing", AC, r=0.012),
            rect("housing", B, -0.115, 0.016, 0.235, 0.256, 3, 0.78, "casing", AC, r=0.02),
            rect("top-cover", B, -0.115, 0.25, 0.235, 0.022, 3.1, 0.6, "top-cover", AC, r=0.01),
            rect("outlet-grille", B, -0.105, 0.06, 0.21, 0.17, 3.2, 0.2, "grille", SM, r=0.004),
            *[line(f"louvre-{i}", B, [[-0.1, 0.075 + 0.02 * i], [0.1, 0.075 + 0.02 * i]], 3.4, 0.9, "grille", SM) for i in range(8)],
            line("seam-lower", B, [[-0.115, 0.045], [0.12, 0.045]], 3.3, 0.3, "casing", AC),
            rect("dial", K, -0.12, 0.161, 0.005, 0.048, 3.5, 0.2, "control", PD),
            line("handle-grip", B, [[-0.05, 0.289], [0.05, 0.289]], 3.3, 0.3, "casing", AC),
        ],
        "plan": [
            rect("housing", B, -0.09, -0.115, 0.185, 0.235, 3, 0.78, "casing", AC, r=0.02),
            rect("top-cover", B, -0.08, -0.105, 0.165, 0.215, 3.1, 0.6, "top-cover", AC, r=0.015),
            rect("handle", B, -0.045, -0.065, 0.1, 0.13, 3.3, 0.75, "casing", AC, r=0.015),
            line("handle-grip", B, [[0.005, -0.055], [0.005, 0.055]], 3.4, 0.3, "casing", AC),
            rect("outlet-grille", B, 0.078, -0.105, 0.02, 0.21, 3.2, 0.2, "grille", SM, r=0.004),
            rect("inlet-grille", B, -0.098, -0.105, 0.01, 0.21, 3.2, 0.3, "intake", PC, r=0.003),
            rect("dial", K, -0.024, -0.12, 0.048, 0.005, 3.5, 0.2, "control", PD),
            line("highlight", B, [[-0.06, 0.1], [-0.06, -0.1]], 3.5, 0.95, "casing", AC),
        ],
    },
    100: {
        "front": [
            rect("foot-left", B, 0.07, 0, 0.03, 0.02, 2.8, 0.15, "feet", RF),
            rect("foot-right", B, -0.1, 0, 0.03, 0.02, 2.8, 0.15, "feet", RF),
            rect("handle", B, -0.065, 0.268, 0.13, 0.042, 2.9, 0.75, "casing", AC),
            rect("housing", B, -0.115, 0.02, 0.235, 0.252, 3, 0.78, "casing", AC, r=0.015),
            line("top-cover-edge", B, [[-0.105, 0.258], [0.11, 0.258]], 3.1, 0.3, "top-cover", AC),
            rect("outlet-grille", B, -0.105, 0.06, 0.21, 0.17, 3.2, 0.2, "grille", SM),
            *[line(f"louvre-{i}", B, [[-0.1, 0.075 + 0.0467 * i], [0.1, 0.075 + 0.0467 * i]], 3.4, 0.9, "grille", SM) for i in range(4)],
        ],
        "plan": [
            rect("housing", B, -0.09, -0.115, 0.185, 0.235, 3, 0.78, "casing", AC, r=0.015),
            rect("handle", B, -0.045, -0.065, 0.1, 0.13, 3.3, 0.75, "casing", AC),
            rect("outlet-grille", B, 0.078, -0.105, 0.02, 0.21, 3.2, 0.2, "grille", SM),
            rect("inlet-grille", B, -0.098, -0.105, 0.01, 0.21, 3.2, 0.3, "intake", PC),
        ],
    },
    200: {
        "front": [
            poly("block", B, [[-0.115, 0], [0.12, 0], [0.12, 0.25], [0.1, 0.272], [0.065, 0.272], [0.065, 0.31], [-0.065, 0.31], [-0.065, 0.272], [-0.095, 0.272], [-0.115, 0.25]], 3, 0.78, regions=["casing", "top-cover", "intake", "control", "feet"], material=AC),
            rect("outlet-grille", B, -0.105, 0.06, 0.21, 0.17, 3.2, 0.2, "grille", SM),
        ],
        "plan": [
            rect("block", B, -0.09, -0.115, 0.185, 0.235, 3, 0.78, regions=["casing", "top-cover", "intake", "control", "feet"], material=AC),
            rect("outlet-grille", B, 0.078, -0.105, 0.02, 0.21, 3.2, 0.2, "grille", SM),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, 4.8 mm by 6.2 mm on paper: the broad cream housing with a rounded top cover, the handle bar, the louvred outlet grille with eight louvre lines, two rubber feet and the dial standing out of the right wall. Plan, 3.9 mm by 4.8 mm: housing and top cover, the handle bar, the outlet grille at the front, the inlet grille at the back, the dial bump on the right wall and a highlight along the top.",
            "de": "Vorderansicht, 4,8 mm auf 6,2 mm: das breite cremefarbene Gehäuse mit gerundeter Oberschale, der Tragegriff, das Auslassgitter mit acht Lamellenlinien, zwei Gummifüße und der Drehknopf, der aus der rechten Wand steht. Grundriss, 3,9 mm auf 4,8 mm: Gehäuse und Oberschale, der Tragegriff, das Auslassgitter vorn, das Eintrittsgitter hinten, der Knopfwulst an der rechten Wand und ein Glanzstrich oben.",
        },
        "omitted": {
            "en": "The mode switch is omitted in front and plan (below 0.01 m); the handle opening is not seen from the front or from above.",
            "de": "Der Betriebsartenschalter entfällt in Vorderansicht und Grundriss (unter 0,01 m); die Grifföffnung ist von vorn und von oben nicht zu sehen.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 2.4 mm by 3.1 mm: block with handle, grille with four louvre lines and two feet. Plan, 2 mm by 2.4 mm: housing, handle and the two grilles.",
            "de": "Vorderansicht, 2,4 mm auf 3,1 mm: Block mit Griff, Gitter mit vier Lamellenlinien und zwei Füße. Grundriss, 2 mm auf 2,4 mm: Gehäuse, Griff und die beiden Gitter.",
        },
        "omitted": {
            "en": "Dial, top cover and highlight in front and plan.",
            "de": "Drehknopf, Oberschale und Glanzstrich in Vorderansicht und Grundriss.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 1.2 mm by 1.55 mm: one block with handle bump and the grille. Plan, 1 mm by 1.2 mm: one block and the outlet strip.",
            "de": "Vorderansicht, 1,2 mm auf 1,55 mm: ein Block mit Griffbuckel und das Gitter. Grundriss, 1 mm auf 1,2 mm: ein Block und der Auslassstreifen.",
        },
        "omitted": {
            "en": "Feet, louvres and inlet grille.",
            "de": "Füße, Lamellen und Eintrittsgitter.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
