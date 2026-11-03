from lib import *

ID = "gas-boiler"
C, CO, CN, MK = "casing", "collar", "connections", "mode-knob"
PS, PCB, SG, FP, BR = "painted-steel", "polycarbonate", "sight-glass", "flue-pipe", "brass"
LF = 0.04
LS = 0.07

SPEC = {
    50: {
        "front": [
            rect("casing", C, -0.155, 0.05, 0.33, 0.62, 3, 0.88, "casing", PS, r=0.014),
            rect("bezel", C, -0.162, 0.4155, 0.007, 0.089, 3.25, 0.55, "sight-glass", SG),
            rect("panel", C, -0.165, 0.065, 0.01, 0.145, 3.1, 0.35, "controls", PCB, r=0.004),
            rect("knob", C, -0.175, 0.114, 0.01, 0.052, 3.4, 0.72, material=PCB),
            rect("set-knob", C, -0.17, 0.122, 0.005, 0.036, 3.4, 0.72, material=PCB),
            line("door", C, [[-0.155, 0.225], [-0.155, 0.643]], 3.06, 0.4),
            rect("collar-base", CO, LF - 0.0625, 0.67, 0.125, 0.03, 2.9, 0.7, "flue-collar", FP, r=0.004),
            rect("collar-pipe", CO, LF - 0.04, 0.7, 0.08, 0.02, 2.9, 0.55, "flue-collar", FP),
            rect("stub", CN, LS - 0.011, 0, 0.022, 0.052, 2.9, 0.7, "connections", BR),
            rect("nut", CN, LS - 0.017, 0.018, 0.034, 0.02, 2.95, 0.45, "connections", BR),
        ],
        "plan": [
            rect("casing", C, -0.22, -0.155, 0.44, 0.33, 3, 0.88, "casing", PS, r=0.02),
            line("top-edge", C, [[-0.205, -0.14], [0.205, -0.14], [0.205, 0.16], [-0.205, 0.16], [-0.205, -0.14]], 3.05, 0.4),
            rect("panel", C, -0.205, -0.165, 0.41, 0.01, 3.1, 0.35, "controls", PCB),
            rect("bezel", C, -0.0445, -0.162, 0.089, 0.007, 3.25, 0.55, "sight-glass", SG),
            rect("knob", C, 0.014, -0.175, 0.052, 0.01, 3.4, 0.72, material=PCB),
            rect("set-knob", C, 0.102, -0.17, 0.036, 0.005, 3.4, 0.72, material=PCB),
            ell("collar-base", CO, 0, LF, 0.0625, 0.0625, 3.5, 0.7, "flue-collar", FP),
            ell("collar-pipe", CO, 0, LF, 0.04, 0.04, 3.6, 0.55, "flue-collar", FP),
            ell("flue-bore", CO, 0, LF, 0.032, 0.032, 3.7, 0.12),
        ],
    },
    100: {
        "front": [
            rect("casing", C, -0.155, 0.05, 0.33, 0.62, 3, 0.88, "casing", PS, r=0.014),
            rect("panel", C, -0.165, 0.065, 0.01, 0.145, 3.1, 0.35, "controls", PCB),
            rect("knob", C, -0.175, 0.112, 0.01, 0.056, 3.4, 0.72, material=PCB),
            rect("collar-base", CO, LF - 0.0625, 0.67, 0.125, 0.03, 2.9, 0.7, "flue-collar", FP),
            rect("collar-pipe", CO, LF - 0.04, 0.7, 0.08, 0.02, 2.9, 0.55, "flue-collar", FP),
            rect("stub", CN, LS - 0.013, 0, 0.026, 0.052, 2.9, 0.7, "connections", BR),
        ],
        "plan": [
            rect("casing", C, -0.22, -0.155, 0.44, 0.33, 3, 0.88, "casing", PS),
            rect("panel", C, -0.205, -0.165, 0.41, 0.01, 3.1, 0.35, "controls", PCB),
            rect("knob", C, 0.012, -0.175, 0.056, 0.01, 3.4, 0.72, material=PCB),
            ell("collar-base", CO, 0, LF, 0.0625, 0.0625, 3.5, 0.7, "flue-collar", FP),
            ell("collar-pipe", CO, 0, LF, 0.04, 0.04, 3.6, 0.55, "flue-collar", FP),
        ],
    },
    200: {
        "front": [
            rect("casing", C, -0.15, 0.05, 0.325, 0.62, 3, 0.88, regions=["casing", "sight-glass", "burner"], material=PS, r=0.014),
            rect("panel", C, -0.165, 0.065, 0.015, 0.145, 3.1, 0.35, "controls", PCB),
            rect("collar", CO, LF - 0.0625, 0.67, 0.125, 0.05, 2.9, 0.6, "flue-collar", FP),
            rect("stubs", CN, LS - 0.02, 0, 0.04, 0.05, 2.9, 0.6, "connections", BR),
        ],
        "plan": [
            rect("casing", C, -0.22, -0.15, 0.44, 0.325, 3, 0.88, regions=["casing", "sight-glass", "burner"], material=PS, r=0.014),
            rect("panel", C, -0.2, -0.165, 0.4, 0.015, 3.1, 0.35, "controls", PCB),
            ell("collar", CO, 0, LF, 0.0625, 0.0625, 3.5, 0.6, "flue-collar", FP),
        ],
    },
}

TEXT = {
    50: {
        "silhouette": {
            "en": "Front, side-on, 7 mm by 14.4 mm on paper: the casing in profile with the dark control panel, the knobs and the sight-glass bezel standing out of the face, the concentric flue collar on top and one connection stub with its nut below; the five stubs hide behind each other. Plan, 8.8 mm by 7 mm: the rounded casing top with its edge line, the control strip and the knobs on the front edge and the flue collar with its bore.",
            "de": "Vorderansicht, von der Seite, 7 mm auf 14,4 mm: das Gehäuse im Profil mit dem dunklen Bedienfeld, den Knöpfen und dem Schauglasrahmen, die aus der Front treten, oben der konzentrische Abgaskragen und unten ein Anschlussstutzen mit Mutter; die fünf Stutzen verdecken sich gegenseitig. Grundriss, 8,8 mm auf 7 mm: die gerundete Gehäuseoberseite mit Kantenlinie, an der Vorderkante der Bedienstreifen und die Knöpfe, dazu der Abgaskragen mit Öffnung.",
        },
        "omitted": {
            "en": "Burner, flames, display and gauge lie behind the casing and are not seen from the side or from above.",
            "de": "Brenner, Flammen, Display und Manometer liegen hinter dem Gehäuse und sind von der Seite und von oben nicht zu sehen.",
        },
    },
    100: {
        "silhouette": {
            "en": "Front, 3.5 mm by 7.2 mm: casing, panel strip, knob, collar in two steps and one stub. Plan, 4.4 mm by 3.5 mm: casing, panel strip, knob and the collar rings.",
            "de": "Vorderansicht, 3,5 mm auf 7,2 mm: Gehäuse, Bedienstreifen, Knopf, Kragen in zwei Stufen und ein Stutzen. Grundriss, 4,4 mm auf 3,5 mm: Gehäuse, Bedienstreifen, Knopf und die Kragenringe.",
        },
        "omitted": {
            "en": "Sight-glass bezel, set knob, nut, door line, edge line and flue bore.",
            "de": "Schauglasrahmen, Einstellknopf, Mutter, Türlinie, Kantenlinie und Abgasöffnung.",
        },
    },
    200: {
        "silhouette": {
            "en": "Front, 1.75 mm by 3.6 mm: one casing block, the control strip, a collar and a stub bar. Plan, 2.2 mm by 1.75 mm: the casing, the control strip and one collar disc.",
            "de": "Vorderansicht, 1,75 mm auf 3,6 mm: ein Gehäuseblock, der Bedienstreifen, ein Kragen und ein Stutzenbalken. Grundriss, 2,2 mm auf 1,75 mm: das Gehäuse, der Bedienstreifen und eine Kragenscheibe.",
        },
        "omitted": {
            "en": "Knobs and collar steps.",
            "de": "Knöpfe und Kragenstufen.",
        },
    },
}

if __name__ == "__main__":
    apply(ID, SPEC, TEXT)
