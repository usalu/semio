from lib import *

ID = "decentralised-hrv-fan"
P, R = "plate", "ring"
AB = "abs-plastic"

SPEC = {
    50: {
        "front": [
            rect("plate", P, -0.12, -0.13, 0.02, 0.26, 3, 0.88, "casing", AB, r=0.004),
            rect("ring", R, -0.1, -0.098, 0.22, 0.196, 3.05, 0.7, "casing", AB),
            rect("rim", R, 0.105, -0.104, 0.015, 0.208, 3.1, 0.6, "casing", AB),
            line("cover-joint", R, [[-0.085, -0.098], [-0.085, 0.098]], 3.2, 0.3, "casing", AB),
            line("wall-line", R, [[0.02, -0.098], [0.02, 0.098]], 3.2, 0.5, "casing", AB),
        ],
        "plan": [
            rect("plate", P, -0.13, -0.12, 0.26, 0.02, 3, 0.88, "casing", AB, r=0.004),
            rect("ring", R, -0.098, -0.1, 0.196, 0.22, 3.05, 0.7, "casing", AB),
            rect("rim", R, -0.104, 0.105, 0.208, 0.015, 3.1, 0.6, "casing", AB),
            line("cover-joint", R, [[-0.098, -0.085], [0.098, -0.085]], 3.2, 0.3, "casing", AB),
            line("wall-line", R, [[-0.098, 0.02], [0.098, 0.02]], 3.2, 0.5, "casing", AB),
        ],
    },
    100: {
        "front": [
            rect("plate", P, -0.12, -0.13, 0.02, 0.26, 3, 0.88, "casing", AB),
            rect("ring", R, -0.1, -0.098, 0.22, 0.196, 3.05, 0.7, "casing", AB),
            rect("rim", R, 0.1, -0.104, 0.02, 0.208, 3.1, 0.6, "casing", AB),
        ],
        "plan": [
            rect("plate", P, -0.13, -0.12, 0.26, 0.02, 3, 0.88, "casing", AB),
            rect("ring", R, -0.098, -0.1, 0.196, 0.22, 3.05, 0.7, "casing", AB),
            rect("rim", R, -0.104, 0.1, 0.208, 0.02, 3.1, 0.6, "casing", AB),
        ],
    },
    200: {
        "front": [
            poly("body", P, [[-0.12, -0.13], [-0.1, -0.13], [-0.1, -0.098], [0.12, -0.098], [0.12, 0.098], [-0.1, 0.098], [-0.1, 0.13], [-0.12, 0.13]], 3, 0.8, "casing", AB),
        ],
        "plan": [
            poly("body", P, [[-0.13, -0.12], [-0.13, -0.1], [-0.098, -0.1], [-0.098, 0.12], [0.098, 0.12], [0.098, -0.1], [0.13, -0.1], [0.13, -0.12]], 3, 0.8, "casing", AB),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front and plan, 4.8 mm by 5.2 mm on paper, show the unit side-on as a T: the thin cover plate on the room side and the round sleeve that runs 0.22 m into the wall, with a flared rim at its outer end, a cover joint and the line where it passes the wall face. Fan, ceramic core and opening are inside the sleeve and not visible from outside.",
            "de": "Vorderansicht und Grundriss, 4,8 mm auf 5,2 mm, zeigen das Gerät von der Seite als T: die dünne Abdeckplatte auf der Raumseite und die runde Hülse, die 0,22 m in die Wand läuft, mit einem aufgeweiteten Rand am Außenende, einer Deckelfuge und der Linie, an der sie die Wandfläche durchstößt. Ventilator, Keramikkern und Öffnung liegen in der Hülse und sind von außen nicht zu sehen.",
        },
        "omitted": {
            "en": "In front and plan the fan, the core and the screws are omitted because they are hidden.",
            "de": "In Vorderansicht und Grundriss entfallen Ventilator, Kern und Schrauben, weil sie verdeckt sind.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front and plan, 2.4 mm by 2.6 mm: plate, sleeve and rim as three separate masses.",
            "de": "Vorderansicht und Grundriss, 2,4 mm auf 2,6 mm: Platte, Hülse und Rand als drei getrennte Massen.",
        },
        "omitted": {
            "en": "Cover joint and wall line in front and plan.",
            "de": "Deckelfuge und Wandlinie in Vorderansicht und Grundriss.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front and plan, 1.2 mm by 1.3 mm: one T-shaped mass of plate and sleeve.",
            "de": "Vorderansicht und Grundriss, 1,2 mm auf 1,3 mm: eine T-förmige Masse aus Platte und Hülse.",
        },
        "omitted": {
            "en": "The rim at the outer end in front and plan.",
            "de": "Der Rand am Außenende in Vorderansicht und Grundriss.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
