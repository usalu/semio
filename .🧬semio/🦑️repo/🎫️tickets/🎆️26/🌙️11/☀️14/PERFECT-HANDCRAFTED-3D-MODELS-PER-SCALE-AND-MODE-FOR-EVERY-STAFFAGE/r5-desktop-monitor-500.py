"""🖥️ Round 5, appliances/desktop-monitor: the digital 1:500 volume authored anew for its 75 mm feature (it was a bulk copy of 1:200 thermal and structure with a lopsided base outline and a one-sentence decision).

Usage: python3 r5-desktop-monitor-500.py  (edits volumes/appliances/desktop-monitor.json in place)
Frames as in r3-desktop-monitor-kit.py: panel bone u up the panel, v front (-) to back (+), w across; base bone u along the depth, extrude plane zx = (w, u).
@see ./r5-brief.md
@see ./r3-appliances-desktop-monitor.md
"""
import json
import pathlib

SRC = pathlib.Path(__file__).with_name("volumes") / "appliances" / "desktop-monitor.json"

PANEL_U = (-0.167, 0.156)
PANEL = [[-0.04, -0.27], [-0.04, 0.27], [-0.026, 0.27], [-0.005, 0.184], [-0.005, -0.184], [-0.026, -0.27]]
HOUSING = [[-0.038, -0.27], [-0.038, 0.27], [-0.026, 0.27], [-0.005, 0.184], [-0.005, -0.184], [-0.026, -0.27]]
BASE = [[-0.125, 0.03], [-0.095, 0.0], [0.095, 0.0], [0.125, 0.03], [0.125, 0.18], [0.095, 0.21], [-0.095, 0.21], [-0.125, 0.18]]
NECK = [[-0.004, -0.006], [0.247, -0.006], [0.256, 0.011], [0.0, 0.03]]

BODY = [
    {"id": "panel", "bone": "panel", "material": "bezel-plastic", "tone": 0.3, "role": "body", "kind": "extrude",
     "plane": "yz", "from": PANEL_U[0], "to": PANEL_U[1], "profile": PANEL},
    {"id": "neck", "bone": "neck", "material": "stand-plastic", "tone": 0.42, "role": "body", "kind": "extrude",
     "plane": "xy", "from": -0.032, "to": 0.032, "profile": NECK},
    {"id": "base-plate", "bone": "base", "material": "stand-plastic", "tone": 0.42, "role": "body", "kind": "extrude",
     "plane": "zx", "from": 0.0, "to": 0.012, "profile": BASE},
]

THERMAL = [
    {"id": "th-front-stack", "bone": "panel", "material": "bezel-plastic", "tone": 0.18, "role": "body",
     "region": "panel-front", "layer": "front-stack", "sectionFaces": ["+v"], "kind": "box",
     "size": [0.323, 0.002, 0.536], "transform": {"translation": [-0.0055, -0.039, 0.0]}},
    {"id": "th-housing", "bone": "panel", "material": "housing-plastic", "tone": 0.55, "role": "body",
     "region": "housing", "layer": "housing-shell", "kind": "extrude", "plane": "yz",
     "from": PANEL_U[0], "to": PANEL_U[1], "profile": HOUSING},
    {"id": "th-rear-plume", "kind": "box", "bone": "panel", "material": "housing-plastic", "role": "body", "tone": 0.55,
     "size": [0.04, 0.0015, 0.3], "transform": {"translation": [0.105, -0.00425, 0.0]},
     "region": "housing", "layer": "housing-shell", "tags": ["rear-plume"]},
    {"id": "th-neck", "of": "neck", "region": "stand", "layer": "stand-body"},
    {"id": "th-base-plate", "of": "base-plate", "region": "stand", "layer": "stand-body"},
]

STRUCTURE = [
    {"id": "member-base", "of": "base-plate", "member": "base", "part": "member"},
    {"id": "member-neck", "of": "neck", "member": "neck", "part": "member"},
    {"id": "member-panel-frame", "bone": "panel", "material": "stand-plastic", "tone": 0.3, "role": "detail",
     "member": "panel", "part": "member", "kind": "box", "size": [0.1, 0.01, 0.46],
     "transform": {"translation": [-0.01, -0.025, 0.0]}},
    {"id": "joint-tilt-hinge", "bone": "panel", "material": "stand-plastic", "tone": 0.3, "role": "detail",
     "member": "panel", "part": "joint", "kind": "box", "size": [0.02, 0.02, 0.064],
     "transform": {"translation": [0.0, 0.0, 0.0]}},
    {"id": "mass-panel", "kind": "ellipsoid", "bone": "panel", "material": "stand-plastic", "role": "detail", "tone": 0.15,
     "radii": [0.03, 0.01, 0.03], "transform": {"translation": [-0.01, -0.022, 0.0]}, "member": "panel", "part": "mass"},
    {"id": "support-base-contact", "kind": "box", "bone": "base", "material": "rubber-foot", "role": "detail", "tone": 0.3,
     "size": [0.2, 0.01, 0.24], "transform": {"translation": [0.105, 0.005, 0.0]},
     "member": "base", "part": "support", "anchor": "base-contact"},
]

DECISIONS = [
    {"en": "Body: a digital model only, the drawing is unavailable at 1:500. The monitor keeps its three assemblies as three solids: the panel as one prism of the true head outline 540 x 323 mm whose section is the 14 mm edge thickening to the 35 mm electronics hump, the tapered riser 64 mm wide, and the 250 x 210 mm base plate with even chamfered corners. Bezel, glass, vents, VESA bracket and pads are below 75 mm and merged into these masses.",
     "de": "Körper: nur ein digitales Modell, die Zeichnung ist bei 1:500 nicht verfügbar. Der Monitor behält seine drei Baugruppen als drei Körper: das Panel als ein Prisma mit dem wahren Umriss 540 x 323 mm, dessen Schnitt vom 14 mm dicken Rand zum 35 mm tiefen Elektronikbuckel wächst, die 64 mm breite, sich verjüngende Stütze und die 250 x 210 mm große Fußplatte mit gleichmäßig gefasten Ecken. Blende, Glas, Lüftungsschlitze, VESA-Halter und Füße liegen unter 75 mm und gehen in diesen Massen auf."},
    {"en": "Thermal: the panel front as a 2 mm skin on the face of the prism, the housing as the rest of the prism behind it with the plume outlet flush on the back, riser and base plate by reference as the stand.",
     "de": "Thermisch: die Panelfront als 2 mm dünne Haut auf der Stirn des Prismas, das Gehäuse als der Rest des Prismas dahinter mit dem Auslass der Wärmefahne bündig auf der Rückseite, Stütze und Fußplatte als Ständer per Verweis."},
    {"en": "Structure: base plate and riser are the members themselves; the LCD module frame is one plate across the VESA field, the tilt hinge a joint at the head of the riser, the panel mass at its centre and the contact pad under the plate.",
     "de": "Tragwerk: Fußplatte und Stütze sind selbst die Stäbe; der Rahmen des LCD-Moduls ist eine Platte über dem VESA-Feld, das Neigegelenk ein Gelenk am Kopf der Stütze, die Panelmasse in ihrem Schwerpunkt und das Kontaktfeld unter der Platte."},
]

doc = json.loads(SRC.read_text())
vol = doc["volumes"]["500"]
vol["body"], vol["thermal"], vol["structure"], vol["decisions"] = BODY, THERMAL, STRUCTURE, DECISIONS
SRC.write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
print("[r5] desktop-monitor 1:500", len(BODY), len(THERMAL), len(STRUCTURE))
