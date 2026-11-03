import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️M2")
from lib import *

A = "scroll-compressor"
doc = load(A)
STEEL, CU, PA, AL = "steel-shell", "copper", "pa-plastic", "cast-aluminium"
DOME = "M -0.1 0.34 C -0.1 0.382 -0.05 0.4 0 0.4 C 0.05 0.4 0.1 0.382 0.1 0.34 Z"

front50 = [
    R("feet", "shell", 3.5, 0.4, -0.14, 0, 0.28, 0.03, "feet", STEEL),
    R("shell", "shell", 3.4, 0.5, -0.1, 0.03, 0.2, 0.31, "shell", STEEL),
    R("sight-ring", "shell", 3.6, 0.3, -0.108, 0.13, 0.008, 0.1, "shell", STEEL),
    R("scroll-disc", "rotor", 3.7, 0.8, -0.106, 0.14, 0.004, 0.08, "scroll", AL, role="glazing"),
    D("dome", "dome", 3.3, 0.7, DOME, "dome", STEEL),
    R("discharge-stub", "dome", 3.2, 0.7, -0.014, 0.398, 0.028, 0.032, "discharge", CU),
    R("terminal", "terminal", 3.5, 0.3, -0.025, 0.25, 0.05, 0.06, "terminal", PA),
]
plan50 = [
    R("feet", "shell", 3.1, 0.4, -0.14, -0.14, 0.28, 0.28, "feet", STEEL),
    E("shell", "shell", 3.4, 0.5, 0, 0, 0.1, 0.1, "shell", STEEL),
    R("suction-stub", "shell", 3.45, 0.2, -0.15, -0.013, 0.052, 0.026, "suction", CU),
    R("sight-ring", "shell", 3.6, 0.3, -0.05, -0.11, 0.1, 0.01, "shell", STEEL),
    R("scroll-disc", "rotor", 3.7, 0.8, -0.04, -0.106, 0.08, 0.006, "scroll", AL, role="glazing"),
    E("dome", "dome", 3.8, 0.7, 0, 0, 0.092, 0.092, "dome", STEEL),
    R("terminal", "terminal", 3.5, 0.3, 0.1, -0.03, 0.04, 0.06, "terminal", PA),
    E("discharge-stub", "dome", 3.9, 0.7, 0, 0, 0.014, 0.014, "discharge", CU),
]
put(doc, 50, front50, plan50,
    "the foot plate and shell are seen as their round cylinder and square plate, the terminal box stands in front of the shell, the suction stub is hidden behind it; the sight ring and the scroll disc show edge-on as thin strips on the right flank, and from above the dome with the discharge stub covers the shell.",
    "Fußplatte und Gehäuse erscheinen als Zylinder und quadratische Platte, der Anschlusskasten steht vor dem Gehäuse, der Saugstutzen ist dahinter verdeckt; Schauglasring und Spiralscheibe zeigen sich als schmale Streifen an der rechten Flanke, und von oben deckt der Dom mit dem Druckstutzen das Gehäuse.")

front100 = [
    R("foot-plate", "shell", 3.5, 0.4, -0.14, 0, 0.28, 0.03, "feet", STEEL),
    R("shell", "shell", 3.4, 0.5, -0.1, 0.03, 0.2, 0.31, ["shell", "feet"], STEEL),
    R("scroll-disc", "rotor", 3.7, 0.8, -0.12, 0.13, 0.02, 0.1, "scroll", AL),
    D("dome", "dome", 3.3, 0.7, DOME, "dome", STEEL),
    R("discharge-stub", "dome", 3.2, 0.7, -0.016, 0.398, 0.032, 0.034, "discharge", CU),
    R("terminal", "terminal", 3.5, 0.3, -0.025, 0.25, 0.05, 0.06, "terminal", PA),
]
plan100 = [
    R("foot-plate", "shell", 3.1, 0.4, -0.14, -0.14, 0.28, 0.28, "feet", STEEL),
    E("shell", "shell", 3.4, 0.5, 0, 0, 0.1, 0.1, ["shell", "feet"], STEEL),
    R("suction-stub", "shell", 3.45, 0.2, -0.15, -0.017, 0.052, 0.034, "suction", CU),
    R("scroll-disc", "rotor", 3.7, 0.8, -0.05, -0.12, 0.1, 0.02, "scroll", AL),
    E("dome", "dome", 3.8, 0.7, 0, 0, 0.092, 0.092, "dome", STEEL),
    R("terminal", "terminal", 3.5, 0.3, 0.1, -0.03, 0.044, 0.06, "terminal", PA),
    E("discharge-stub", "dome", 3.9, 0.7, 0, 0, 0.016, 0.016, "discharge", CU),
]
put(doc, 100, front100, plan100,
    "the shell and the foot plate stay one shape each, the scroll disc is a 20 mm raised pad on the right flank and the dome with the discharge stub tops the shell from above.",
    "Gehäuse und Fußplatte bleiben je eine Form, die Spiralscheibe ist ein 20 mm hohes Polster an der rechten Flanke, und der Dom mit dem Druckstutzen schließt das Gehäuse von oben ab.")

front200 = [
    R("foot-plate", "shell", 3.5, 0.4, -0.14, 0, 0.28, 0.04, "feet", STEEL),
    R("shell", "shell", 3.4, 0.5, -0.1, 0.04, 0.2, 0.3, ["shell", "feet", "scroll"], STEEL),
    D("dome", "dome", 3.3, 0.7, DOME, "dome", STEEL),
    R("discharge-stub", "dome", 3.2, 0.7, -0.02, 0.396, 0.04, 0.034, "discharge", CU),
    R("terminal", "terminal", 3.5, 0.3, -0.025, 0.25, 0.05, 0.06, "terminal", PA),
]
plan200 = [
    R("foot-plate", "shell", 3.1, 0.4, -0.14, -0.14, 0.28, 0.28, "feet", STEEL),
    E("shell", "shell", 3.4, 0.5, 0, 0, 0.1, 0.1, ["shell", "feet", "scroll"], STEEL),
    R("suction-stub", "shell", 3.45, 0.2, -0.16, -0.02, 0.062, 0.04, "suction", CU),
    E("dome", "dome", 3.8, 0.7, 0, 0, 0.09, 0.09, "dome", STEEL),
    R("terminal", "terminal", 3.5, 0.3, 0.1, -0.03, 0.05, 0.06, "terminal", PA),
    E("discharge-stub", "dome", 3.9, 0.7, 0, 0, 0.02, 0.02, "discharge", CU),
]
put(doc, 200, front200, plan200,
    "five shapes from the front and six from above: foot plate, shell, dome, discharge stub and terminal box, plus the suction stub in the top view; the scroll disc is merged into the shell.",
    "fünf Formen von vorn und sechs von oben: Fußplatte, Gehäuse, Dom, Druckstutzen und Anschlusskasten, dazu der Saugstutzen in der Draufsicht; die Spiralscheibe geht im Gehäuse auf.")
save(A, doc)
print("ok")
