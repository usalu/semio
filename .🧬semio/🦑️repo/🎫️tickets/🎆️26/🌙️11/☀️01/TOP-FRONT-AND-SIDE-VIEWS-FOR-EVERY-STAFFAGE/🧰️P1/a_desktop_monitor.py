from lib import *

ID = "desktop-monitor"
BA, NE, PA = "base", "neck", "panel"
SP, HP, BZ, DG, RF = "stand-plastic", "housing-plastic", "bezel-plastic", "display-glass", "rubber-foot"

SPEC = {
    50: {
        "front": [
            rect("foot-left", BA, 0.09, 0, 0.022, 0.002, 3, 0.15, "stand", RF),
            rect("foot-right", BA, -0.112, 0, 0.022, 0.002, 3, 0.15, "stand", RF),
            rect("base-plate", BA, -0.125, 0.002, 0.25, 0.0102, 3.1, 0.5, "stand", SP, r=0.005),
            poly("neck", NE, [[-0.035, 0.0122], [0.035, 0.0122], [0.03, 0.2], [0.032, 0.255], [-0.032, 0.255], [-0.03, 0.2]], 3.2, 0.45, "stand", SP),
            rect("panel-slab", PA, -0.27, 0.0857, 0.54, 0.3217, 3.6, 0.6, "panel-front", BZ, r=0.006, follow=True),
            rect("screen-glass", PA, -0.255, 0.0976, 0.51, 0.2978, 3.7, 0.1, "panel-front", DG, role="glazing", follow=True),
            ell("power-led", PA, -0.24, 0.092, 0.003, 0.003, 3.8, 0.9, "panel-front", BZ, role="detail"),
        ],
        "plan": [
            rect("base-plate", BA, -0.096, -0.125, 0.21, 0.25, 3.1, 0.5, "stand", SP, r=0.012),
            rect("neck", NE, -0.078, -0.035, 0.036, 0.07, 3.2, 0.45, "stand", SP),
            rect("housing", PA, -0.0656, -0.27, 0.0226, 0.54, 3.4, 0.55, "housing", HP, r=0.004),
            rect("panel-edge", PA, -0.043, -0.27, 0.0095, 0.54, 3.6, 0.6, "panel-front", BZ),
            rect("screen-glass", PA, -0.0335, -0.255, 0.0281, 0.51, 3.7, 0.1, "panel-front", DG, role="glazing"),
            line("vents-1", PA, [[-0.062, -0.15], [-0.047, -0.15]], 3.5, 0.3, "housing", HP),
            line("vents-2", PA, [[-0.062, -0.075], [-0.047, -0.075]], 3.5, 0.3, "housing", HP),
            line("vents-3", PA, [[-0.062, 0], [-0.047, 0]], 3.5, 0.3, "housing", HP),
            line("vents-4", PA, [[-0.062, 0.075], [-0.047, 0.075]], 3.5, 0.3, "housing", HP),
            line("vents-5", PA, [[-0.062, 0.15], [-0.047, 0.15]], 3.5, 0.3, "housing", HP),
        ],
    },
    100: {
        "front": [
            rect("base-plate", BA, -0.125, 0, 0.25, 0.0122, 3.1, 0.5, "stand", SP),
            poly("neck", NE, [[-0.035, 0.0122], [0.035, 0.0122], [0.03, 0.2], [0.032, 0.255], [-0.032, 0.255], [-0.03, 0.2]], 3.2, 0.45, "stand", SP),
            rect("panel-slab", PA, -0.27, 0.0857, 0.54, 0.3217, 3.6, 0.6, "panel-front", BZ, follow=True),
            rect("screen-glass", PA, -0.25, 0.1, 0.5, 0.2955, 3.7, 0.1, "panel-front", DG, role="glazing", follow=True),
        ],
        "plan": [
            rect("base-plate", BA, -0.096, -0.125, 0.21, 0.25, 3.1, 0.5, "stand", SP),
            rect("neck", NE, -0.078, -0.035, 0.036, 0.07, 3.2, 0.45, "stand", SP),
            rect("housing", PA, -0.0656, -0.27, 0.0321, 0.54, 3.4, 0.55, ["housing"][0], HP),
            rect("screen-glass", PA, -0.0335, -0.255, 0.0281, 0.51, 3.7, 0.1, "panel-front", DG, role="glazing"),
        ],
    },
    200: {
        "front": [
            poly("stand", BA, [[-0.125, 0], [0.125, 0], [0.125, 0.012], [0.035, 0.012], [0.035, 0.26], [-0.035, 0.26], [-0.035, 0.012], [-0.125, 0.012]], 3, 0.5, "stand", SP),
            rect("panel", PA, -0.27, 0.0857, 0.54, 0.3217, 3.4, 0.58, regions=["panel-front", "housing"], material=HP, follow=True),
        ],
        "plan": [
            rect("stand", BA, -0.096, -0.125, 0.21, 0.25, 3, 0.5, "stand", SP),
            rect("panel", PA, -0.0656, -0.27, 0.0602, 0.54, 3.4, 0.58, regions=["panel-front", "housing"], material=HP),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, 10.8 mm by 8.2 mm on paper: the wide panel with its bezel, the dark screen glass and a power light above the two-footed base plate, with the neck showing between them. The panel is bound to the panel bone and follows its tilt. Plan, 4.2 mm by 10.8 mm: the base plate, the neck behind the panel, the rear housing with five vent marks, the top edge of the panel and the screen seen at a slant because the panel leans back.",
            "de": "Vorderansicht, 10,8 mm auf 8,2 mm: das breite Panel mit Rahmen, dem dunklen Bildschirmglas und einer Betriebsleuchte über der Fußplatte mit zwei Füßen, dazwischen der Hals. Das Panel ist an den Panel-Knochen gebunden und folgt seiner Neigung. Grundriss, 4,2 mm auf 10,8 mm: die Fußplatte, der Hals hinter dem Panel, das hintere Gehäuse mit fünf Lüftungsstrichen, die Oberkante des Panels und der Bildschirm in schräger Sicht, weil das Panel nach hinten geneigt ist.",
        },
        "omitted": {
            "en": "Hinge, cables and the rear vents are hidden in the front view.",
            "de": "Gelenk, Kabel und die hinteren Lüftungsschlitze sind in der Vorderansicht verdeckt.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 5.4 mm by 4.1 mm: base plate, neck and panel with its glass. Plan, 2.1 mm by 5.4 mm: base plate, neck, one housing block and the screen strip.",
            "de": "Vorderansicht, 5,4 mm auf 4,1 mm: Fußplatte, Hals und Panel mit Glas. Grundriss, 2,1 mm auf 5,4 mm: Fußplatte, Hals, ein Gehäuseblock und der Bildschirmstreifen.",
        },
        "omitted": {
            "en": "Feet, power light, vent marks and the separate panel edge.",
            "de": "Füße, Betriebsleuchte, Lüftungsstriche und die getrennte Panelkante.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 2.7 mm by 2 mm: a T-shaped stand and one panel mass. Plan, 1 mm by 2.7 mm: the base plate and one panel bar.",
            "de": "Vorderansicht, 2,7 mm auf 2 mm: ein T-förmiger Ständer und eine Panelmasse. Grundriss, 1 mm auf 2,7 mm: die Fußplatte und ein Panelbalken.",
        },
        "omitted": {
            "en": "Glass, bezel and housing are one mass.",
            "de": "Glas, Rahmen und Gehäuse sind eine Masse.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
