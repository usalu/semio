"""🧯 Writes the asset document `🔣️.json` of the used forced-draught oil burner from the numbers of `author-geometry.py`.

Run: python3 author-asset.py   (the authoritative chisel; the document is regenerated from scratch every time)
"""
import json
import math
import pathlib
import importlib.util

HERE = pathlib.Path(__file__).parent


def load(name):
    """📥 Import a sibling author module by file name."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), HERE / name)
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


G = load("author-geometry.py")
P = load("author-project.py")
TXT = json.loads((HERE / "author-text.json").read_text())
AREAS = json.loads((HERE / "author-areas.json").read_text()) if (HERE / "author-areas.json").exists() else {}
r6 = G.r6


def T(key):
    """🗣️ Bilingual text by key."""
    return TXT[key]


MATERIALS = [
    ("pine", 0.9, 0.62, 470, 0.13, 1600, 11e9, 18e6, "literature", "en-338"),
    ("housing-alu", 0.82, 0.55, 2700, 96, 900, 70e9, 240e6, "authored-estimate", "incropera-2018"),
    ("motor-steel", 0.88, 0.6, 7850, 50, 460, None, None, "authored-estimate", "incropera-2018"),
    ("pump-alu", 0.3, 0.5, 2700, 120, 900, None, None, "authored-estimate", "incropera-2018"),
    ("polyamide", 0.92, 0.85, 1350, 0.3, 1500, None, None, "authored-estimate", "iso-10456"),
    ("flange-steel", 0.85, 0.8, 7850, 50, 460, 210e9, 235e6, "literature", "iso-10456"),
    ("ceramic-fibre", 0.9, 0.4, 400, 0.12, 1050, None, None, "authored-estimate", "incropera-2018"),
    ("stainless-tube", 0.6, 0.7, 7900, 16, 500, 193e9, 205e6, "literature", "incropera-2018"),
    ("head-steel", 0.95, 0.95, 7850, 25, 500, None, None, "authored-estimate", "incropera-2018"),
    ("braided-hose", 0.35, 0.5, 1900, 0.5, 1200, None, None, "authored-estimate", "incropera-2018"),
    ("brass", 0.4, 0.5, 8500, 110, 380, None, None, "literature", "incropera-2018"),
]


def materials():
    """🧪 Asset materials with emissivity, absorptance and bulk data."""
    out = []
    for mid, e, a, rho, k, c, E, f, src, ref in MATERIALS:
        m = {"id": mid, "title": T("mat-" + mid), "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c}
        if E:
            m["elasticModulusPa"] = E
            m["strengthPa"] = f
        m.update({"source": src, "reference": ref})
        out.append(m)
    return out


MASS = {"crate": 6.3, "burner": 11.6, "hoses": 0.42, "air-damper": 0.12}


def rig():
    """🦴 Yard root, crate that stays behind, burner on its lift joint, swinging hoses and the air shutter."""
    hx, hy = G.HOSE_ORIGIN
    return {"kind": "mechanical", "root": "yard", "bones": [
        {"id": "yard", "parent": None, "title": T("bone-yard"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "crate", "parent": "yard", "title": T("bone-crate"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": G.CRATE_L, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.0, "maxM": 0},
         "mass": {"kg": MASS["crate"], "comX": 0.31, "comY": 0.07, "distribution": "uniform", "source": "authored-estimate", "reference": "en-338"}},
        {"id": "burner", "parent": "yard", "title": T("bone-burner"), "x": G.XB, "y": G.YB, "angleDeg": 0, "lengthM": 0.15, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6},
         "mass": {"kg": MASS["burner"], "comX": -0.132, "comY": -0.005, "distribution": "point", "source": "authored-estimate", "reference": "en-267"}},
        {"id": "hoses", "parent": "burner", "title": T("bone-hoses"), "x": hx, "y": hy, "z": G.HOSE_Z, "angleDeg": G.HOSE_REST_DEG, "lengthM": G.HOSE_LEN + G.CAP_LEN, "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 5},
         "mass": {"kg": MASS["hoses"], "comX": 0.18, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "din-4798"}},
        {"id": "air-damper", "parent": "burner", "title": T("bone-air-damper"), "x": G.DAMPER_ORIGIN[0], "y": G.DAMPER_ORIGIN[1], "angleDeg": -90, "lengthM": 0.09, "joint": {"type": "revolute", "minDeg": -45, "maxDeg": 0},
         "mass": {"kg": MASS["air-damper"], "comX": 0.045, "comY": -0.0075, "distribution": "point", "source": "authored-estimate"}},
    ]}


def anchors():
    """⚓ Supports of every pose, the grips of the carry and the flame outlet."""
    return [
        {"id": "skid-rear", "bone": "crate", "x": 0.06, "y": 0, "kind": "support", "title": T("anchor-skid-rear"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": 0.15},
        {"id": "skid-front", "bone": "crate", "x": 0.56, "y": 0, "kind": "support", "title": T("anchor-skid-front"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": 0.15},
        {"id": "burner-seat", "bone": "burner", "x": -0.15, "y": -0.13, "kind": "support", "title": T("anchor-burner-seat"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.15, "footprintWidthM": 0.17, "frictionCoefficient": 0.4},
        {"id": "grip-front", "bone": "burner", "x": -0.06, "y": -0.13, "kind": "support", "title": T("anchor-grip-front"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.1},
        {"id": "grip-rear", "bone": "burner", "x": -0.22, "y": -0.13, "kind": "support", "title": T("anchor-grip-rear"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.1},
        {"id": "door-flange", "bone": "burner", "x": 0.008, "y": 0, "kind": "support", "title": T("anchor-door-flange"), "support": "fixed", "normalDeg": 180, "footprintLengthM": 0.17, "footprintWidthM": 0.17},
        {"id": "tube-mouth", "bone": "burner", "x": 0.15, "y": 0, "kind": "emitter", "title": T("anchor-tube-mouth")},
    ]


REGIONS = [
    ("crate", "surface", "passive", "pine", "horizontal-plate-up", 0.5, 0.62, 9400, None),
    ("housing", "surface", "operating", "housing-alu", "vertical-plate", 0.4, 0.24, 8100, ("motor", 6)),
    ("motor", "surface", "operating", "motor-steel", "cylinder", 0.4, 0.11, 7200, ("motor", 22)),
    ("pump", "surface", "operating", "pump-alu", "vertical-plate", 0.4, 0.09, 8100, ("motor", 4)),
    ("controls", "surface", "operating", "polyamide", "vertical-plate", 0.45, 0.12, 6100, ("motor", 6)),
    ("flange", "surface", "operating", "flange-steel", "vertical-plate", 0.4, 0.17, 3600, ("burner", 18)),
    ("gasket", "surface", "operating", "ceramic-fibre", "vertical-plate", 0.4, 0.17, 3400, ("burner", 4)),
    ("blast-tube", "surface", "operating", "stainless-tube", "cylinder", 0.4, 0.089, 7900, ("burner", 260)),
    ("combustion-head", "heater", "operating", "head-steel", "vertical-plate", 0.4, 0.085, 3100, ("burner", 40)),
    ("oil-hoses", "surface", "passive", "braided-hose", "cylinder", 0.3, 0.016, 2300, None),
    ("hose-fittings", "surface", "passive", "brass", "cylinder", 0.3, 0.023, 6400, None),
]


def thermal():
    """🌡️ Regions with material data, operating keys, the flame-gas emitter and assumptions."""
    regs = []
    for rid, kind, model, mat, geom, sky, lc, hc, op in REGIONS:
        r = {"id": rid, "title": T("reg-" + rid), "kind": kind, "model": model, "material": mat, "areaM2": AREAS.get(rid, 0.05), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": hc}
        if op:
            r["operating"] = {"key": op[0], "heatW": op[1]}
        if rid == "motor":
            r["ratedPowerW"] = 150
            r["ratedPowerBasis"] = "electrical-input"
        if rid == "combustion-head":
            r["ratedPowerW"] = 40000
            r["ratedPowerBasis"] = "fuel-input"
        if rid == "crate":
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        r.update({"source": "authored-estimate", "reference": "en-267" if rid in ("combustion-head", "blast-tube", "motor") else "incropera-2018"})
        regs.append(r)
    return {
        "environment": {"airTempC": 18, "meanRadiantTempC": 19, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [
            {"id": "burner", "title": T("op-burner"), "levels": [{"v": 0, "title": T("lv-off")}, {"v": 0.75, "title": T("lv-30kw")}, {"v": 1, "title": T("lv-40kw")}]},
            {"id": "motor", "title": T("op-motor"), "levels": [{"v": 0, "title": T("lv-off")}, {"v": 1, "title": T("lv-motor-on")}]},
        ],
        "regions": regs,
        "emitters": [{"id": "flame-gas", "kind": "exhaust", "title": T("em-flame-gas"), "anchor": "tube-mouth", "directionDeg": 0, "diameterM": 0.085, "operatingKey": "burner", "volumeFlowM3S": 0.068, "temperatureRiseK": 1100, "vapourKgPerKg": 0.064, "source": "authored-estimate", "reference": "en-267"}],
        "assumptions": [T("th-assume-1"), T("th-assume-2"), T("th-assume-3"), T("th-assume-4")],
    }


def binding(state):
    """🔗 Exhaustive region bindings of one pose state: stored, lifted, firing or off."""
    reg = {}
    for rid, *_ in REGIONS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if state == "stored" and rid == "crate":
            b = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 15}}
        if state == "stored" and rid == "housing":
            b = {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 15}}
        if state in ("firing", "off") and rid == "gasket":
            b = {"exposedFraction": 0.2, "sunlitFraction": 0, "contact": {"fraction": 0.8, "tempC": 70 if state == "firing" else 25}}
        if state in ("firing", "off") and rid in ("blast-tube", "combustion-head"):
            b = {"exposedFraction": 1, "sunlitFraction": 0}
        reg[rid] = b
    on = state == "firing"
    th = {"regions": reg, "emitters": {"flame-gas": 1 if on else 0}, "operating": {"burner": 0.75 if on else 0, "motor": 1 if on else 0}}
    return th


HANG = -90 - G.HOSE_REST_DEG


def poses():
    """🧍 Stored in the crate, carried out of it, reinstalled firing and reinstalled at rest."""
    return [
        {"id": "stored", "title": T("pose-stored"), "description": T("pose-stored-d"), "bones": {}, "ground": "authored", "contacts": ["skid-rear", "skid-front", "burner-seat"], "thermal": binding("stored"), "loads": []},
        {"id": "lifted", "title": T("pose-lifted"), "description": T("pose-lifted-d"), "bones": {"burner": {"dy": G.LIFT_DY}, "hoses": {"angleDeg": r6(HANG)}}, "ground": "authored", "contacts": ["skid-rear", "skid-front", "grip-front", "grip-rear"], "thermal": binding("lifted"), "loads": []},
        {"id": "reinstalled", "title": T("pose-reinstalled"), "description": T("pose-reinstalled-d"), "bones": {"crate": {"dx": G.CRATE_ASIDE}, "burner": {"dy": G.INSTALL_DY}, "hoses": {"angleDeg": r6(HANG)}, "air-damper": {"angleDeg": -35}}, "ground": "authored", "contacts": ["skid-rear", "skid-front", "door-flange"], "thermal": binding("firing"),
         "loads": [{"id": "hose-pull", "kind": "equipment", "title": T("load-hose-pull"), "bone": "burner", "x": -0.2, "y": -0.085, "fxN": 0, "fyN": -8, "source": "authored-estimate"}]},
        {"id": "reinstalled-off", "title": T("pose-off"), "description": T("pose-off-d"), "bones": {"crate": {"dx": G.CRATE_ASIDE}, "burner": {"dy": G.INSTALL_DY}, "hoses": {"angleDeg": r6(HANG)}}, "ground": "authored", "contacts": ["skid-rear", "skid-front", "door-flange"], "thermal": binding("off"),
         "loads": [{"id": "hose-pull", "kind": "equipment", "title": T("load-hose-pull"), "bone": "burner", "x": -0.2, "y": -0.085, "fxN": 0, "fyN": -8, "source": "authored-estimate"}]},
    ]


def hose_angle_on_floor(lift):
    """📐 Hose swing that keeps the capped end on the crate floor while the burner rises by `lift` metres."""
    drop = 0.13 + G.HOSE_ORIGIN[1] - G.CAP_R
    reach = G.HOSE_LEN + G.CAP_LEN
    s = min(1.0, (drop + lift) / reach)
    total = -math.degrees(math.asin(s))
    return r6(max(-90.0, total) - G.HOSE_REST_DEG)


def clips():
    """🎞️ The lift out of the crate and the firing cycle of the reinstalled burner."""
    keys = []
    for t, lift in [(0, 0), (1, 0.0), (1.8, 0.08), (2.6, 0.16), (3.4, 0.24), (4.2, 0.3), (4.6, 0.325), (5, G.LIFT_DY)]:
        keys.append({"t": t, "pose": "stored" if t == 0 else "lifted", "ease": "linear", "bones": {"burner": {"dy": r6(lift)}, "hoses": {"angleDeg": hose_angle_on_floor(lift) if t < 5 else r6(HANG)}}})
    lift = {"id": "lift", "title": T("clip-lift"), "description": T("clip-lift-d"), "durationS": 5, "fps": 25, "loop": False, "rootMotion": "none", "keys": keys,
            "contacts": [{"anchor": "skid-rear", "fromS": 0, "toS": 5}, {"anchor": "skid-front", "fromS": 0, "toS": 5}, {"anchor": "burner-seat", "fromS": 0, "toS": 1}, {"anchor": "grip-front", "fromS": 0.5, "toS": 5}, {"anchor": "grip-rear", "fromS": 0.5, "toS": 5}],
            "phases": [{"id": "grip", "fromS": 0, "toS": 1, "title": T("ph-grip")}, {"id": "raise", "fromS": 1, "toS": 5, "title": T("ph-raise")}],
            "events": [{"t": 1, "kind": "contact-off", "title": T("ev-seat-off"), "target": "burner-seat"}, {"t": 4.6, "kind": "motion", "title": T("ev-hoses-free")}], "tracks": []}
    on = {"crate": {"dx": G.CRATE_ASIDE}, "burner": {"dy": G.INSTALL_DY}, "hoses": {"angleDeg": r6(HANG)}}
    def k(t, pose, damper, ease="smooth"):
        b = {key: dict(v) for key, v in on.items()}
        b["air-damper"] = {"angleDeg": damper}
        return {"t": t, "pose": pose, "ease": ease, "bones": b}
    fire = {"id": "firing-cycle", "title": T("clip-fire"), "description": T("clip-fire-d"), "durationS": 180, "fps": 2, "loop": False, "rootMotion": "none",
            "keys": [k(0, "reinstalled-off", 0), k(3, "reinstalled-off", 0), k(8, "reinstalled", -35), k(150, "reinstalled", -35), k(165, "reinstalled-off", -35), k(170, "reinstalled-off", 0), k(180, "reinstalled-off", 0)],
            "contacts": [{"anchor": "door-flange", "fromS": 0, "toS": 180}, {"anchor": "skid-rear", "fromS": 0, "toS": 180}, {"anchor": "skid-front", "fromS": 0, "toS": 180}],
            "phases": [{"id": "standby", "fromS": 0, "toS": 3, "title": T("ph-standby")}, {"id": "pre-purge", "fromS": 3, "toS": 20, "title": T("ph-prepurge")}, {"id": "firing", "fromS": 20, "toS": 150, "title": T("ph-firing")}, {"id": "post-purge", "fromS": 150, "toS": 165, "title": T("ph-postpurge")}, {"id": "off", "fromS": 165, "toS": 180, "title": T("ph-off")}],
            "events": [{"t": 3, "kind": "operating", "title": T("ev-motor-on"), "target": "motor"}, {"t": 20, "kind": "ignition", "title": T("ev-ignition"), "target": "combustion-head"}, {"t": 150, "kind": "shutdown", "title": T("ev-shutdown"), "target": "combustion-head"}, {"t": 165, "kind": "operating", "title": T("ev-motor-off"), "target": "motor"}],
            "tracks": [{"target": "operating:motor", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3, "v": 1}, {"t": 165, "v": 0}]},
                       {"target": "operating:burner", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 20, "v": 0.75}, {"t": 150, "v": 0}]},
                       {"target": "emitter:flame-gas", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 20, "v": 1}, {"t": 150, "v": 0}]}]}
    return [lift, fire]


def thermal_markers(scale, have):
    """🏷️ Thermal-only solids for regions whose parts the coarse body merged, at their real places."""
    b = "burner"
    out = []
    if "motor" not in have:
        out.append(G.box("th-motor-face", b, "motor-steel", 0.35, (-0.20, -0.02, 0.171), (0.11, 0.11, 0.002), region="motor"))
    if "pump" not in have:
        out.append(G.box("th-pump-face", b, "pump-alu", 0.7, (-0.20, -0.03, -0.141), (0.07, 0.09, 0.002), region="pump"))
    if "flange" not in have:
        out.append(G.revolve("th-flange-ring", b, "flange-steel", 0.2, [[0, 0.0445], [0, 0.085], [0.002, 0.085], [0.002, 0.0445], [0, 0.0445]], [1, 0, 0], (0.006, 0, 0), region="flange", sheet=True))
    if "gasket" not in have:
        out.append(G.revolve("th-gasket-ring", b, "ceramic-fibre", 0.85, [[0, 0.0445], [0, 0.085], [0.002, 0.085], [0.002, 0.0445], [0, 0.0445]], [1, 0, 0], (0.008, 0, 0), region="gasket", sheet=True))
    if "combustion-head" not in have:
        out.append(G.revolve("th-head-disc", b, "head-steel", 0.12, [[0, 0], [0, 0.04], [0.002, 0.04], [0.002, 0]], [1, 0, 0], (0.15, 0, 0), region="combustion-head", sheet=True))
    if "hose-fittings" not in have:
        for i, w in enumerate((0.011, -0.011)):
            out.append(G.revolve(f"th-cap-{'ab'[i]}", "hoses", "brass", 0.6, [[0, G.HOSE_R], [0, G.CAP_R], [G.CAP_LEN, G.CAP_R], [G.CAP_LEN, G.HOSE_R], [0, G.HOSE_R]], [1, 0, 0], (G.HOSE_LEN - 0.008, 0, w), region="hose-fittings"))
    return out


def strip(bodies):
    """🧽 Body solids without the authoring-only `region` member."""
    return [{k: v for k, v in s.items() if k != "region"} for s in bodies]


def thermal_list(scale, bodies):
    """🌡️ Thermal solids: every body solid of the scale referenced into its region, plus markers for merged regions."""
    refs = [{"id": "th-" + s["id"], "of": s["id"], "region": s["region"]} for s in bodies if s.get("region") and not s.get("texture")]
    have = {s["region"] for s in bodies if s.get("region") and not s.get("texture")}
    a0, a1 = {50: (0.138, 0.15), 100: (0.12, 0.152), 200: (0.12, 0.152), 500: (0.077, 0.152)}[scale]
    flame = G.revolve("th-flame-gas", "burner", "head-steel", 0.9, [[0, 0], [0, 0.04], [a1 - a0, 0.04], [a1 - a0, 0]], [1, 0, 0], (a0, 0, 0), region="combustion-head", role="detail", tags=["flame-gas"])
    return refs + thermal_markers(scale, have) + [flame]


def structure_list(scale, bodies):
    """🏗️ Structure solids: members on their bones, the flange joint, supports at every support anchor and the mass centres."""
    ids = {s["id"] for s in bodies}
    out = []
    member_of = {"crate": "crate", "burner": "burner", "hoses": "hoses", "air-damper": "air-damper"}
    for s in bodies:
        if s.get("texture"):
            continue
        part = "joint" if s["id"] in ("flange", "gasket", "nipple-a", "nipple-b") else "member"
        st = {"id": "st-" + s["id"], "of": s["id"], "member": member_of[s["bone"]], "part": part}
        if s["id"] == "housing" and scale >= 200:
            st["members"] = ["air-damper"]
        out.append(st)
    sz = max(0.0075, {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[scale])
    for a, x in (("skid-rear", 0.06), ("skid-front", 0.56)):
        out.append({"id": "support-" + a, "kind": "extrude", "bone": "crate", "material": "pine", "role": "body", "tone": 0.4, "profile": [[x - 0.05, 0.0], [x + 0.05, 0.0], [x, {50: 0.02, 100: 0.03, 200: 0.04, 500: 0.075}[scale]]], "plane": "xy", "from": 0.135, "to": 0.21, "member": "crate", "part": "support", "anchor": a})
    rm = {50: 0.015, 100: 0.02, 200: 0.03, 500: 0.0375}[scale]
    for bone, c in (("crate", (0.31, 0.07, 0)), ("burner", (-0.132, -0.005, 0))):
        out.append({"id": "mass-" + bone, "kind": "ellipsoid", "bone": bone, "material": "pine" if bone == "crate" else "housing-alu", "role": "detail", "tone": 0.5, "radii": [rm, rm, rm], "transform": {"translation": list(c)}, "member": bone, "part": "mass"})
    return out


def representation(scale):
    """🖊️ One authored representation: the three views drawn from the solids of the same scale, plus detail strokes."""
    bodies = G.body(scale)
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    el = [P.elevation_shape(s) for s in bodies if not s.get("texture") or scale == 50]
    fr = [P.front_shape(s) for s in bodies if not s.get("texture")]
    pl = [P.plan_shape(s) for s in bodies if not s.get("texture")]
    extra = {100: {"flange": ["flange", "gasket"]}, 200: {"flange": ["flange", "gasket"], "blast-tube": ["blast-tube", "combustion-head"], "hose-a": ["oil-hoses", "hose-fittings"], "hose-b": ["oil-hoses", "hose-fittings"]}}.get(scale, {})
    for sh in el + fr + pl:
        if scale == 200 and sh["id"].startswith("hose-"):
            sh["penMm"] = 0.3
        if sh["id"] in extra:
            sh.pop("region", None)
            sh["regions"] = extra[sh["id"]]
    det = load("author-details.py")
    e2, f2, p2 = det.details(scale)
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": det.hatches(scale),
            "geometry": {"elevation": el + e2, "front": fr + f2, "plan": pl + p2},
            "decisions": {k: T(f"rep{scale}-{k}") for k in ("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays")}}


def volume(scale):
    """🧊 The handcrafted volume of one scale."""
    bodies = G.body(scale)
    return {"status": "authored", "denominator": scale, "body": strip(bodies), "thermal": thermal_list(scale, bodies), "structure": structure_list(scale, bodies),
            "decisions": [T(f"vol{scale}-body"), T(f"vol{scale}-thermal"), T(f"vol{scale}-structure")]}


def asset():
    """🧯 The whole asset document."""
    lo, hi = P.world_bounds(G.body(50))
    dims = {"heightM": r6(hi[1] - lo[1]), "widthM": r6(hi[0] - lo[0]), "depthM": r6(hi[2] - lo[2]), "massKg": r6(sum(MASS.values()))}
    refs = json.loads((HERE / "author-references.json").read_text())
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {
        "id": "used-forced-draught-oil-burner", "category": "components", "title": T("title"), "summary": T("summary"),
        "tags": ["component", "reclaimed", "used", "hvac", "heating", "oil-burner", "forced-draught-burner", "monoblock-burner", "burner", "die-cast-aluminium", "crate", "light-heating-oil"],
        "provenance": {"author": "Semio staffage component authors (used HVAC wave, author 03)", "method": "handcrafted", "created": "2026-11-22", "references": refs, "notes": T("prov-notes")},
        "dimensions": dims, "materials": materials(), "rig": rig(), "anchors": anchors(),
        "representations": {"50": representation(50), "100": representation(100), "200": representation(200), "500": {
            "status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T("rep500-reason"), "limitingFeature": T("rep500-limit"), "limitingSizeM": 0.089,
            "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}},
        "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal(),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["skid-rear", "skid-front"], "assumptions": [T("st-assume-1"), T("st-assume-2"), T("st-assume-3")], "clinical": False},
        "volumes": {"50": volume(50), "100": volume(100), "200": volume(200), "500": volume(500)},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored"}}


if __name__ == "__main__":
    doc = asset()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    d = doc["asset"]["dimensions"]
    print("[DEBUG] wrote", d)
