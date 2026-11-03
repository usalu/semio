from lib import *

ID = "ceiling-diffuser"
B = "body"
PL, GS = "powder-coated-steel", "galvanised-steel"

SPEC = {
    50: {
        "front": [
            rect("collar", B, -0.125, 0, 0.25, 0.05, 2.9, 0.5, "collar", GS),
            line("collar-band", B, [[-0.125, 0.026], [0.125, 0.026]], 2.95, 0.3, "collar", GS),
            rect("flange", B, -0.2975, -0.012, 0.595, 0.012, 3, 0.88, "plate", PL, r=0.002),
            poly("cone-1", B, [[-0.265, -0.012], [0.265, -0.012], [0.245, -0.032], [-0.245, -0.032]], 3.1, 0.8, "plate", PL),
            poly("cone-2", B, [[-0.2, -0.032], [0.2, -0.032], [0.18, -0.052], [-0.18, -0.052]], 3.2, 0.7, "plate", PL),
            poly("cone-3", B, [[-0.135, -0.052], [0.135, -0.052], [0.115, -0.07], [-0.115, -0.07]], 3.3, 0.6, "plate", PL),
            line("slot-left", B, [[-0.262, -0.0125], [-0.205, -0.0325]], 3.4, 0.15, "plate", PL),
            line("slot-right", B, [[0.262, -0.0125], [0.205, -0.0325]], 3.4, 0.15, "plate", PL),
        ],
        "plan": [
            rect("flange", B, -0.2975, -0.2975, 0.595, 0.595, 1, 0.88, "plate", PL, r=0.006),
            poly("flange-edge", B, [[-0.285, -0.285], [0.285, -0.285], [0.285, 0.285], [-0.285, 0.285]], 1.1, 0.5, "plate", PL, role="detail", filled=False),
            ell("screw-1", B, -0.27, -0.27, 0.006, 0.006, 1.2, 0.45, "plate", PL, role="detail"),
            ell("screw-2", B, 0.27, -0.27, 0.006, 0.006, 1.2, 0.45, "plate", PL, role="detail"),
            ell("screw-3", B, -0.27, 0.27, 0.006, 0.006, 1.2, 0.45, "plate", PL, role="detail"),
            ell("screw-4", B, 0.27, 0.27, 0.006, 0.006, 1.2, 0.45, "plate", PL, role="detail"),
            ell("collar", B, 0, 0, 0.125, 0.125, 2, 0.5, "collar", GS),
            ell("opening", B, 0, 0, 0.112, 0.112, 2.1, 0.12),
            ell("collar-bead", B, 0, 0, 0.119, 0.119, 2.2, 0.3, "collar", GS, role="detail", filled=False),
        ],
    },
    100: {
        "front": [
            rect("collar", B, -0.125, 0, 0.25, 0.05, 2.9, 0.5, "collar", GS),
            rect("flange", B, -0.2975, -0.02, 0.595, 0.02, 3, 0.88, "plate", PL),
            poly("cone-1", B, [[-0.255, -0.02], [0.255, -0.02], [0.225, -0.045], [-0.225, -0.045]], 3.1, 0.75, "plate", PL),
            poly("cone-2", B, [[-0.18, -0.045], [0.18, -0.045], [0.12, -0.07], [-0.12, -0.07]], 3.2, 0.65, "plate", PL),
        ],
        "plan": [
            rect("flange", B, -0.2975, -0.2975, 0.595, 0.595, 1, 0.88, "plate", PL),
            ell("collar", B, 0, 0, 0.125, 0.125, 2, 0.5, "collar", GS),
            ell("opening", B, 0, 0, 0.1, 0.1, 2.1, 0.12),
        ],
    },
    200: {
        "front": [
            rect("collar", B, -0.125, 0, 0.25, 0.05, 2.9, 0.5, "collar", GS),
            rect("flange", B, -0.2975, -0.04, 0.595, 0.04, 3, 0.88, "plate", PL),
            poly("cone", B, [[-0.24, -0.04], [0.24, -0.04], [0.12, -0.07], [-0.12, -0.07]], 3.1, 0.7, "plate", PL),
        ],
        "plan": [
            rect("flange", B, -0.2975, -0.2975, 0.595, 0.595, 1, 0.88, "plate", PL),
            ell("collar", B, 0, 0, 0.13, 0.13, 2, 0.5, "collar", GS),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front: the same section as the side view, because the diffuser is square, with a clamp band on the collar. Plan, 11.9 mm square: the flange with its folded edge and four screws, the round collar with its bore and a bead on top; the cones lie hidden below the flange.",
            "de": "Vorderansicht: derselbe Schnitt wie in der Seitenansicht, weil der Auslass quadratisch ist, mit einem Spannband am Kragen. Grundriss, 11,9 mm im Quadrat: der Flansch mit umgelegtem Rand und vier Schrauben, der runde Kragen mit Öffnung und Wulst obenauf; die Kegel liegen verdeckt unter dem Flansch.",
        },
        "omitted": {
            "en": "In the plan the three cones, the slots and the central screw are hidden under the flange.",
            "de": "Im Grundriss sind die drei Kegel, die Schlitze und die zentrale Schraube vom Flansch verdeckt.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front: flange, two cones and collar as in the side view. Plan, 6 mm square: the flange as one square with the collar disc and its dark bore.",
            "de": "Vorderansicht: Flansch, zwei Kegel und Kragen wie in der Seitenansicht. Grundriss, 6 mm im Quadrat: der Flansch als ein Quadrat mit der Kragenscheibe und ihrer dunklen Öffnung.",
        },
        "omitted": {
            "en": "Screws, folded edge and collar bead in plan.",
            "de": "Schrauben, umgelegter Rand und Kragenwulst im Grundriss.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front: flange, one cone and collar. Plan, 3 mm square: a flange square with one collar disc.",
            "de": "Vorderansicht: Flansch, ein Kegel und Kragen. Grundriss, 3 mm im Quadrat: ein Flanschquadrat mit einer Kragenscheibe.",
        },
        "omitted": {
            "en": "The collar bore in plan.",
            "de": "Die Kragenöffnung im Grundriss.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
