"""🫖 Round 5, appliances/electric-kettle: the digital 1:500 volume authored anew. It was a bulk pass with a barrel-bellied jug wider than the real one, a flat fin spout, an octagon plate handle, water poking through the wall, a lid and structure copied from 1:200 and a one-sentence decision.

Usage: python3 r5-electric-kettle-500.py  (edits volumes/appliances/electric-kettle.json in place)
Frames as in r3-electric-kettle-kit.py: every bone horizontal, revolve profiles are (height along the world-up axis, radius); the jug cone is the 1:50 reference (r 78 mm at the skirt, 72 mm under the rim at 175 mm).
@see ./r5-brief.md
@see ./r3-appliances-electric-kettle.md
"""
import json
import pathlib

SRC = pathlib.Path(__file__).with_name("volumes") / "appliances" / "electric-kettle.json"

Q_ROOT = [-0.312877, 0.63412, 0.63412, -0.312877]
Q_TIP = [-0.323559, 0.628736, 0.628736, -0.323559]
SPOUT = [((-0.056, 0.146), 0.022, 0.030, Q_ROOT), ((-0.114, 0.189), 0.009, 0.016, Q_TIP)]
HANDLE = [
    ((-0.006, 0.066), [0.5, 0.5, 0.5, 0.5]),
    ((0.026, 0.066), [0.607717, 0.361498, 0.361498, 0.607717]),
    ((0.0445, 0.0), [0.707107, 0.0, 0.0, 0.707107]),
    ((0.026, -0.058), [0.619486, -0.340935, -0.340935, 0.619486]),
    ((-0.004, -0.0615), [0.521511, -0.477521, -0.477521, 0.521511]),
]


def revolve(id, bone, material, tone, profile, **extra):
    """🌀 A turned solid about the world-up axis of its bone, profile as (height, radius)."""
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": extra.pop("role", "body"), "tone": tone,
            "profile": profile, "axis": [0, 1, 0], "origin": [0.0, 0.0, 0.0], **extra}


def spout(id, grow, **extra):
    """🫗 The pouring spout as a two-section loft from the wall root to the mouth, grown by `grow` for a sleeve."""
    return {"id": id, "kind": "loft", "bone": "body", "material": "stainless", "role": "body", "tone": 0.55,
            "sections": [{"at": [x, y, 0.0], "rx": rx + grow, "ry": ry + grow, "exponent": 2.2, "rotation": q}
                         for (x, y), rx, ry, q in SPOUT], "capStart": True, "capEnd": True, **extra}


def handle(id, exponent, **extra):
    """✊ The C handle as a five-section loft (top fixing, top bend, grip, bottom bend, bottom fixing)."""
    return {"id": id, "kind": "loft", "bone": "handle", "material": "pp-plastic", "role": extra.pop("role", "body"), "tone": extra.pop("tone", 0.2),
            "sections": [{"at": [x, y, 0.0], "rx": 0.016, "ry": 0.019, "exponent": exponent, "rotation": q} for (x, y), q in HANDLE],
            "capStart": True, "capEnd": True, **extra}


BODY = [
    revolve("base", "base", "pp-plastic", 0.2, [[0.0, 0.0], [0.0, 0.086], [0.006, 0.09], [0.03, 0.089], [0.03, 0.0]]),
    revolve("jug", "body", "stainless", 0.55, [[0.0, 0.0], [0.0, 0.079], [0.15, 0.0732], [0.172, 0.0712], [0.175, 0.067], [0.175, 0.0]]),
    spout("spout", 0.0),
    handle("handle", 2.2),
    revolve("lid", "lid", "stainless", 0.6, [[-0.006, 0.0], [-0.006, 0.064], [0.0, 0.064], [0.0, 0.07], [0.006, 0.07], [0.016, 0.046], [0.019, 0.018], [0.032, 0.018], [0.032, 0.0]]),
]

THERMAL = [
    {"id": "th-base", "of": "base", "region": "plate", "layer": "plate-skin"},
    {"id": "th-jug", "of": "jug", "region": "shell", "layer": "shell-skin"},
    {"id": "th-spout", "of": "spout", "region": "shell", "layer": "shell-skin"},
    revolve("th-water", "water", "water", 0.45, [[0.0, 0.0], [0.0, 0.0725], [0.06, 0.0705], [0.06, 0.0]], region="water", layer="water-skin"),
    {"id": "th-lid", "of": "lid", "region": "lid", "layer": "lid-skin"},
    {"id": "th-handle", "of": "handle", "region": "handle", "layer": "handle-skin"},
    {"id": "th-steam", "kind": "loft", "bone": "body", "material": "water", "role": "body", "tone": 0.45,
     "sections": [{"at": [-0.11278, 0.18813, 0.0], "rx": 0.0095, "ry": 0.0165, "exponent": 2.0, "rotation": Q_TIP},
                  {"at": [-0.11498, 0.1897, 0.0], "rx": 0.0095, "ry": 0.0165, "exponent": 2.0, "rotation": Q_TIP}],
     "capStart": False, "capEnd": True, "tags": ["steam"], "region": "water", "layer": "water-skin"},
    {**spout("th-steam-plume", 0.002, tags=["steam-plume"], region="shell", layer="shell-skin"), "capStart": False},
]

STRUCTURE = [
    revolve("member-base", "base", "pp-plastic", 0.3, [[0.0, 0.07], [0.0, 0.088], [0.028, 0.088], [0.03, 0.084], [0.03, 0.0], [0.022, 0.0], [0.022, 0.07], [0.0, 0.07]],
            role="detail", member="base", part="member"),
    revolve("member-body", "body", "stainless", 0.3, [[0.0, 0.0], [0.0, 0.078], [0.175, 0.0705], [0.175, 0.0645], [0.008, 0.0715], [0.008, 0.0]],
            role="detail", member="body", part="member"),
    {"id": "member-handle", "of": "handle", "member": "handle", "part": "member"},
    revolve("member-lid", "lid", "stainless", 0.3, [[0.0, 0.0], [0.0, 0.066], [0.006, 0.066], [0.006, 0.0]], role="detail", member="lid", part="member"),
    revolve("member-water", "water", "water", 0.45, [[0.0, 0.0], [0.0, 0.06], [0.008, 0.06], [0.008, 0.0]], role="detail", member="water", part="member"),
    {"id": "mass-water", "kind": "ellipsoid", "bone": "water", "material": "water", "role": "detail", "tone": 0.15,
     "radii": [0.035, 0.012, 0.035], "transform": {"translation": [0.0, 0.03, 0.0]}, "member": "water", "part": "mass"},
    revolve("support-base-contact", "base", "pp-plastic", 0.3, [[0.0, 0.066], [0.0, 0.09], [0.006, 0.09], [0.006, 0.066]],
            role="detail", member="base", part="support", anchor="base-contact", tags=["base-contact"]),
]

DECISIONS = [
    {"en": "Body: a digital model only, the drawing is unavailable at 1:500 (the kettle is 3 x 3 x 2.4 smallest features). Five masses keep the silhouette of every finer scale: the 30 mm power base, the jug as the true cone from r 79 mm at the foot to r 71 mm under the rolled rim (skirt, gauge and rim merged), the spout as one taper from its 60 mm root on the shoulder to the 18 mm mouth, the C handle as a round-section bow through its two fixings, the lid with its dome and knob turned in one piece, its knob ending at the 0.24 m overall height.",
     "de": "Körper: nur ein digitales Modell, die Zeichnung ist bei 1:500 nicht verfügbar (der Wasserkocher misst 3 x 3 x 2,4 kleinste Merkmale). Fünf Massen bewahren den Umriss aller feineren Maßstäbe: der 30 mm hohe Sockel, die Kanne als der wahre Kegel von r 79 mm am Fuß bis r 71 mm unter dem gerollten Rand (Schürze, Wasserstandsanzeige und Rand darin aufgegangen), der Ausguss als eine Verjüngung von seiner 60 mm breiten Wurzel an der Schulter zur 18 mm Mündung, der C-Griff als Bogen mit rundem Querschnitt durch seine beiden Befestigungen, der Deckel mit Kuppel und Knauf in einem Stück gedreht, der Knauf endet bei der Gesamthöhe von 0,24 m."},
    {"en": "Thermal: base, jug, spout, lid and handle by reference, the 1 L fill as its own turned volume 60 mm deep inside the jug, the steam outlet flush in the mouth and the plume sleeve 2 mm over the spout.",
     "de": "Thermisch: Sockel, Kanne, Ausguss, Deckel und Griff per Verweis, die 1-L-Füllung als eigener Drehkörper von 60 mm Tiefe in der Kanne, der Dampfauslass bündig in der Mündung und die Fahnenhülse 2 mm über dem Ausguss."},
    {"en": "Structure: the base housing as its rim wall with the top deck the jug stands on, the jug as a cup of 6 mm wall on an 8 mm floor, the handle as its own member, the lid as a plate, the water as a slab on the floor with its mass at the centre of the fill, and the contact ring under the base rim.",
     "de": "Tragwerk: das Sockelgehäuse als Randwand mit der Deckplatte, auf der die Kanne steht, die Kanne als Becher mit 6 mm Wand auf 8 mm Boden, der Griff als eigener Stab, der Deckel als Platte, das Wasser als Scheibe auf dem Boden mit seiner Masse im Schwerpunkt der Füllung und der Kontaktring unter dem Sockelrand."},
]

doc = json.loads(SRC.read_text())
vol = doc["volumes"]["500"]
vol["body"], vol["thermal"], vol["structure"], vol["decisions"] = BODY, THERMAL, STRUCTURE, DECISIONS
SRC.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
print("[r5] electric-kettle 1:500", len(BODY), len(THERMAL), len(STRUCTURE))
