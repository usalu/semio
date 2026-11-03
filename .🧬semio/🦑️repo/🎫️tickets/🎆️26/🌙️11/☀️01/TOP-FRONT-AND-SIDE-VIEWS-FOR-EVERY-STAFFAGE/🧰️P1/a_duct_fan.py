from lib import *

ID = "duct-fan"
P, R = "plate", "ring"
AB = "abs-plastic"

SPEC = {
    50: {
        "front": [
            rect("plate", P, -0.045, -0.11, 0.012, 0.22, 3, 0.88, "casing", AB, r=0.003),
            rect("ring", R, -0.033, -0.1, 0.078, 0.2, 3.05, 0.7, "casing", AB),
            rect("lip", R, 0.038, -0.104, 0.007, 0.208, 3.1, 0.6, "casing", AB),
            line("seam", R, [[-0.015, -0.1], [-0.015, 0.1]], 3.2, 0.3, "casing", AB),
        ],
        "plan": [
            rect("plate", P, -0.11, -0.045, 0.22, 0.012, 3, 0.88, "casing", AB, r=0.003),
            rect("ring", R, -0.1, -0.033, 0.2, 0.078, 3.05, 0.7, "casing", AB),
            rect("lip", R, -0.104, 0.038, 0.208, 0.007, 3.1, 0.6, "casing", AB),
            line("seam", R, [[-0.1, -0.015], [0.1, -0.015]], 3.2, 0.3, "casing", AB),
        ],
    },
    100: {
        "front": [
            rect("plate", P, -0.045, -0.11, 0.014, 0.22, 3, 0.88, "casing", AB),
            rect("ring", R, -0.031, -0.1, 0.076, 0.2, 3.05, 0.7, "casing", AB),
        ],
        "plan": [
            rect("plate", P, -0.11, -0.045, 0.22, 0.014, 3, 0.88, "casing", AB),
            rect("ring", R, -0.1, -0.031, 0.2, 0.076, 3.05, 0.7, "casing", AB),
        ],
    },
    200: {
        "front": [
            poly("body", P, [[-0.045, -0.11], [-0.03, -0.11], [-0.03, -0.1], [0.045, -0.1], [0.045, 0.1], [-0.03, 0.1], [-0.03, 0.11], [-0.045, 0.11]], 3, 0.8, "casing", AB),
        ],
        "plan": [
            poly("body", P, [[-0.11, -0.045], [-0.11, -0.03], [-0.1, -0.03], [-0.1, 0.045], [0.1, 0.045], [0.1, -0.03], [0.11, -0.03], [0.11, -0.045]], 3, 0.8, "casing", AB),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front and plan, 1.8 mm by 4.4 mm on paper, show the fan side-on: the thin plate on the room side, the round spigot behind it with a flared lip and one seam line. Blades, hub and opening are inside the spigot.",
            "de": "Vorderansicht und Grundriss, 1,8 mm auf 4,4 mm, zeigen den Ventilator von der Seite: die dünne Platte auf der Raumseite, dahinter der runde Stutzen mit aufgeweiteter Lippe und einer Fugenlinie. Flügel, Nabe und Öffnung liegen im Stutzen.",
        },
        "omitted": {
            "en": "Screws, blades and hub are hidden in front and plan.",
            "de": "Schrauben, Flügel und Nabe sind in Vorderansicht und Grundriss verdeckt.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front and plan, 0.9 mm by 2.2 mm: plate and spigot as two masses.",
            "de": "Vorderansicht und Grundriss, 0,9 mm auf 2,2 mm: Platte und Stutzen als zwei Massen.",
        },
        "omitted": {
            "en": "Lip and seam line in front and plan.",
            "de": "Lippe und Fugenlinie in Vorderansicht und Grundriss.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front and plan, 0.45 mm by 1.1 mm: one T-shaped mark of plate and spigot.",
            "de": "Vorderansicht und Grundriss, 0,45 mm auf 1,1 mm: ein T-förmiges Zeichen aus Platte und Stutzen.",
        },
        "omitted": {
            "en": "Everything but the T silhouette in front and plan.",
            "de": "In Vorderansicht und Grundriss alles außer der T-Silhouette.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
