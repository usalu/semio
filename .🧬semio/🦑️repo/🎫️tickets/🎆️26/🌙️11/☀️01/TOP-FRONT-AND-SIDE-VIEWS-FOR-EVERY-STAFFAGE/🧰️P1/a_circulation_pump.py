from lib import *

ID = "circulation-pump"
H, M, T = "housing", "motor", "terminal"
CI, AL, PA, WG, CU, BR = "cast-iron", "aluminium", "polyamide", "window-glass", "copper", "brass"
YC = 0.0575
LC = 0.04

SPEC = {
    50: {
        "front": [
            rect("casing", H, 0.015, 0, 0.05, 0.115, 3, 0.42, "casing", CI, r=0.006),
            rect("motor-head", M, -0.06, 0.012, 0.075, 0.091, 3.3, 0.62, "motor", AL, r=0.004),
            line("motor-ring", M, [[-0.05, 0.016], [-0.05, 0.099]], 3.31, 0.5),
            rect("window", M, -0.065, 0.031, 0.005, 0.053, 3.4, 0.18, "rotor-window", WG),
            rect("terminal-box", T, -0.055, 0.1075, 0.05, 0.0425, 3.5, 0.78, "terminal-box", PA, r=0.004),
            line("lid-seam", T, [[-0.055, 0.1325], [-0.005, 0.1325]], 3.51, 0.3),
            ell("union", H, LC, YC, 0.021, 0.021, 3.8, 0.5, "casing", BR),
            ell("stub", H, LC, YC, 0.014, 0.014, 3.85, 0.7, "casing", CU),
            ell("bore", H, LC, YC, 0.009, 0.009, 3.9, 0.12),
        ],
        "plan": [
            rect("stub-l", H, -0.09, LC - 0.014, 0.015, 0.028, 3.02, 0.7, "casing", CU),
            rect("stub-r", H, 0.075, LC - 0.014, 0.015, 0.028, 3.02, 0.7, "casing", CU),
            rect("union-l", H, -0.0775, LC - 0.021, 0.0235, 0.042, 3.1, 0.5, "casing", BR),
            rect("union-r", H, 0.054, LC - 0.021, 0.0235, 0.042, 3.1, 0.5, "casing", BR),
            rect("casing", H, -0.0575, 0.015, 0.115, 0.05, 3, 0.42, "casing", CI, r=0.006),
            line("nut-edge-l", H, [[-0.0645, LC - 0.018], [-0.0645, LC + 0.018]], 3.15, 0.3),
            line("nut-edge-r", H, [[0.0645, LC - 0.018], [0.0645, LC + 0.018]], 3.15, 0.3),
            rect("motor-head", M, -0.0455, -0.06, 0.091, 0.075, 3.3, 0.62, "motor", AL, r=0.004),
            line("motor-ring", M, [[-0.0455, -0.045], [0.0455, -0.045]], 3.31, 0.5),
            rect("window", M, -0.0305, -0.065, 0.061, 0.005, 3.4, 0.18, "rotor-window", WG),
            rect("terminal-box", T, -0.03, -0.05, 0.06, 0.05, 3.5, 0.78, "terminal-box", PA, r=0.004),
            line("lid-seam", T, [[-0.03, -0.025], [0.03, -0.025]], 3.51, 0.3),
            rect("gland", T, -0.0365, -0.0315, 0.0065, 0.013, 3.45, 0.4),
        ],
    },
    100: {
        "front": [
            rect("casing", H, 0.015, 0, 0.05, 0.115, 3, 0.42, "casing", CI),
            rect("motor-head", M, -0.065, 0.012, 0.08, 0.091, 3.3, 0.62, "motor", AL),
            rect("terminal-box", T, -0.055, 0.1075, 0.05, 0.0425, 3.5, 0.78, "terminal-box", PA),
            ell("stub", H, LC, YC, 0.017, 0.017, 3.8, 0.6, "casing", BR),
        ],
        "plan": [
            rect("stub-l", H, -0.09, LC - 0.017, 0.035, 0.034, 3.02, 0.6, "casing", BR),
            rect("stub-r", H, 0.055, LC - 0.017, 0.035, 0.034, 3.02, 0.6, "casing", BR),
            rect("casing", H, -0.0575, 0.015, 0.115, 0.05, 3, 0.42, "casing", CI),
            rect("motor-head", M, -0.0455, -0.065, 0.091, 0.08, 3.3, 0.62, "motor", AL),
            rect("terminal-box", T, -0.03, -0.05, 0.06, 0.05, 3.5, 0.78, "terminal-box", PA),
        ],
    },
    200: {
        "front": [
            poly("body", H, [[-0.065, 0.012], [0.015, 0.012], [0.015, 0], [0.065, 0], [0.065, 0.115], [0.015, 0.115], [0.015, 0.103], [-0.065, 0.103]], 3, 0.42, regions=["casing", "motor", "rotor-window"], material=CI),
            rect("terminal-box", T, -0.055, 0.1075, 0.05, 0.0425, 3.5, 0.78, "terminal-box", PA),
            ell("pipe", H, LC, YC, 0.0195, 0.0195, 3.8, 0.6, "casing", BR),
        ],
        "plan": [
            rect("pipe", H, -0.09, LC - 0.0195, 0.18, 0.039, 3.02, 0.6, "casing", BR),
            poly("body", H, [[-0.0455, -0.065], [0.0455, -0.065], [0.0455, 0.015], [0.0575, 0.015], [0.0575, 0.065], [-0.0575, 0.065], [-0.0575, 0.015], [-0.0455, 0.015]], 3, 0.42, regions=["casing", "motor", "rotor-window"], material=CI),
            rect("terminal-box", T, -0.03, -0.05, 0.06, 0.05, 3.5, 0.78, "terminal-box", PA),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, looking along the pipe, 2.6 mm by 3 mm on paper: the thin volute disc as a tall slab on the left, the longer motor head with its round window end on the right, the terminal box on top and the union end of the pipe as three rings in front. Plan, 3.6 mm by 2.6 mm: the pipe with a stub and union nut on each side, the volute block, the motor head reaching towards the viewer with two rings and the window end, and the terminal box with its lid seam and the gland on the left wall.",
            "de": "Vorderansicht, entlang der Rohrachse, 2,6 mm auf 3 mm: die flache Spiralgehäuse-Scheibe als hohe Platte links, der längere Motorkopf mit dem runden Fensterende rechts, der Klemmenkasten obenauf und das Verschraubungsende des Rohrs als drei Ringe davor. Grundriss, 3,6 mm auf 2,6 mm: das Rohr mit Anschlussstutzen und Überwurfmutter auf jeder Seite, der Spiralgehäuse-Block, der Motorkopf zum Betrachter hin mit zwei Ringen und dem Fensterende sowie der Klemmenkasten mit Deckelfuge und der Kabelverschraubung an der linken Wand.",
        },
        "omitted": {
            "en": "The rotor, the LED and the pointer lie inside or on the end window and do not appear in front or plan.",
            "de": "Rotor, LED und Zeiger liegen im Innern oder am Endfenster und erscheinen weder in der Vorderansicht noch im Grundriss.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 1.3 mm by 1.5 mm: the volute slab, the longer motor block, the terminal box and one brass disc for the pipe end. Plan, 1.8 mm by 1.3 mm: the pipe stubs, the volute block, the motor block and the terminal box.",
            "de": "Vorderansicht, 1,3 mm auf 1,5 mm: die Spiralgehäuse-Platte, der längere Motorblock, der Klemmenkasten und eine Messingscheibe für das Rohrende. Grundriss, 1,8 mm auf 1,3 mm: die Rohrstutzen, der Spiralgehäuse-Block, der Motorblock und der Klemmenkasten.",
        },
        "omitted": {
            "en": "Front and plan have no window, rings, lid seam or nut edges.",
            "de": "Vorderansicht und Grundriss haben kein Fenster, keine Ringe, keine Deckelfuge und keine Mutterkanten.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 0.65 mm wide: one stepped body of volute and motor with the terminal box and a pipe disc. Plan: the stepped body with the pipe through it and the terminal box.",
            "de": "Vorderansicht, 0,65 mm breit: ein gestufter Körper aus Spiralgehäuse und Motor mit Klemmenkasten und einer Rohrscheibe. Grundriss: der gestufte Körper mit dem durchgehenden Rohr und dem Klemmenkasten.",
        },
        "omitted": {
            "en": "Union nuts, window and the split between volute and motor in front and plan.",
            "de": "Überwurfmuttern, Fenster und die Trennung zwischen Spiralgehäuse und Motor in Vorderansicht und Grundriss.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
