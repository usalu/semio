from lib import *

ID = "electric-kettle"
BA, BO, HA, LI = "base", "body", "handle", "lid"
ST, PP = "stainless", "pp-plastic"

SPEC = {
    50: {
        "front": [
            rect("base", BA, -0.09, 0, 0.18, 0.03, 3.6, 0.5, "plate", PP, r=0.004),
            poly("body", BO, [[-0.075, 0.03], [0.075, 0.03], [0.07, 0.205], [-0.07, 0.205]], 3.4, 0.5, "shell", ST),
            line("body-shoulder", BO, [[-0.0725, 0.12], [0.0725, 0.12]], 3.45, 0.3, "shell", ST),
            rect("handle", HA, -0.016, 0.045, 0.032, 0.155, 3.5, 0.2, "handle", PP, r=0.01),
            line("handle-grip", HA, [[0, 0.06], [0, 0.185]], 3.55, 0.6, "handle", PP),
            rect("lid", LI, -0.066, 0.205, 0.132, 0.016, 3.6, 0.45, "lid", ST),
            rect("lid-button", LI, -0.012, 0.221, 0.024, 0.016, 3.7, 0.2, "lid", PP),
        ],
        "plan": [
            ell("base", BA, 0, 0, 0.09, 0.09, 1, 0.5, "plate", PP),
            ell("body", BO, 0, 0, 0.075, 0.075, 2, 0.4, "shell", ST),
            ell("body-shoulder", BO, 0, 0, 0.07, 0.07, 2.1, 0.3, "shell", ST, role="detail", filled=False),
            poly("spout", BO, [[-0.06, -0.026], [-0.116, -0.012], [-0.116, 0.012], [-0.06, 0.026]], 2.5, 0.4, "shell", ST),
            rect("handle", HA, 0.07, -0.016, 0.055, 0.032, 2.6, 0.2, "handle", PP, r=0.012),
            ell("lid", LI, 0, 0, 0.066, 0.066, 3, 0.45, "lid", ST),
            ell("lid-button", LI, 0, 0, 0.012, 0.012, 3.1, 0.2, "lid", PP),
        ],
    },
    100: {
        "front": [
            rect("base", BA, -0.09, 0, 0.18, 0.03, 3.6, 0.5, "plate", PP),
            poly("body", BO, [[-0.075, 0.03], [0.075, 0.03], [0.07, 0.205], [-0.07, 0.205]], 3.4, 0.4, regions=["shell", "water"], material=ST),
            rect("handle", HA, -0.016, 0.045, 0.032, 0.155, 3.5, 0.2, "handle", PP),
            rect("lid", LI, -0.066, 0.205, 0.132, 0.03, 3.6, 0.45, "lid", ST),
        ],
        "plan": [
            ell("base", BA, 0, 0, 0.09, 0.09, 1, 0.5, "plate", PP),
            ell("body", BO, 0, 0, 0.075, 0.075, 2, 0.4, regions=["shell", "water"], material=ST),
            poly("spout", BO, [[-0.06, -0.026], [-0.116, -0.012], [-0.116, 0.012], [-0.06, 0.026]], 2.5, 0.4, "shell", ST),
            rect("handle", HA, 0.07, -0.016, 0.055, 0.032, 2.6, 0.2, "handle", PP),
            ell("lid", LI, 0, 0, 0.066, 0.066, 3, 0.45, "lid", ST),
        ],
    },
    200: {
        "front": [
            poly("kettle", BO, [[-0.095, 0], [0.095, 0], [0.095, 0.035], [0.078, 0.035], [0.072, 0.205], [-0.072, 0.205], [-0.078, 0.035], [-0.095, 0.035]], 3.4, 0.45, regions=["plate", "shell", "water"], material=ST),
            rect("handle", HA, -0.017, 0.04, 0.034, 0.16, 3.5, 0.2, "handle", PP),
            rect("lid", LI, -0.07, 0.205, 0.14, 0.04, 3.6, 0.45, "lid", ST),
        ],
        "plan": [
            ell("base", BA, 0, 0, 0.095, 0.095, 1, 0.5, "plate", PP),
            poly("jug", BO, [[-0.118, -0.014], [-0.0728, -0.028], [-0.0675, -0.039], [-0.039, -0.0675], [0, -0.078], [0.039, -0.0675], [0.0675, -0.039], [0.0759, -0.018], [0.128, -0.018], [0.128, 0.018], [0.0759, 0.018], [0.0675, 0.039], [0.039, 0.0675], [0, 0.078], [-0.039, 0.0675], [-0.0675, 0.039], [-0.0728, 0.028], [-0.118, 0.014]], 2, 0.4, regions=["shell", "lid", "handle"], material=ST),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, 3.6 mm by 4.8 mm on paper: the jug seen from the handle side, the tapered stainless body on its round power base, the handle bar standing in front of it with a grip line, the lid and its button, which follows the lid when it lifts. Plan, 4.9 mm by 3.6 mm: the base disc, the round body with its shoulder ring, the spout at the left, the handle at the right and the lid with its button.",
            "de": "Vorderansicht, 3,6 mm auf 4,8 mm: der Kocher von der Griffseite, der sich nach oben verjüngende Edelstahlkörper auf dem runden Stromsockel, davor der Griffbügel mit einer Grifflinie, der Deckel und sein Knopf, der dem Deckel beim Anheben folgt. Grundriss, 4,9 mm auf 3,6 mm: die Sockelscheibe, der runde Körper mit Schulterring, links die Tülle, rechts der Griff und der Deckel mit Knopf.",
        },
        "omitted": {
            "en": "The spout is hidden behind the body in front; the water and the gauge strip are not visible from the front or from above.",
            "de": "Die Tülle ist in der Vorderansicht vom Körper verdeckt; Wasser und Anzeigestreifen sind von vorn und von oben nicht zu sehen.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 1.8 mm by 2.4 mm: base, body, handle bar and lid as four masses. Plan, 2.45 mm by 1.8 mm: base disc, body disc, spout, handle and lid disc.",
            "de": "Vorderansicht, 1,8 mm auf 2,4 mm: Sockel, Körper, Griffbügel und Deckel als vier Massen. Grundriss, 2,45 mm auf 1,8 mm: Sockelscheibe, Körperscheibe, Tülle, Griff und Deckelscheibe.",
        },
        "omitted": {
            "en": "Lid button, grip line and shoulder ring.",
            "de": "Deckelknopf, Grifflinie und Schulterring.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 0.9 mm by 1.2 mm: base and body merged into one tapered mass with a thickened lid on top and the handle bar in front. Plan, 1.2 mm by 0.9 mm: the base disc under one jug outline in which body, lid, spout and handle are merged.",
            "de": "Vorderansicht, 0,9 mm auf 1,2 mm: Sockel und Körper zu einer sich verjüngenden Masse verschmolzen, darauf ein verstärkter Deckel, davor der Griffbügel. Grundriss, 1,2 mm auf 0,9 mm: die Sockelscheibe unter einem Kocherumriss, in dem Körper, Deckel, Tülle und Griff verschmolzen sind.",
        },
        "omitted": {
            "en": "All detail lines and the separate water, spout, handle and lid shapes in plan; the handle and spout are drawn slightly thicker than real.",
            "de": "Alle Detaillinien und die getrennten Wasser-, Tülle-, Griff- und Deckelformen im Grundriss; Griff und Tülle sind etwas dicker als in Wirklichkeit gezeichnet.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
