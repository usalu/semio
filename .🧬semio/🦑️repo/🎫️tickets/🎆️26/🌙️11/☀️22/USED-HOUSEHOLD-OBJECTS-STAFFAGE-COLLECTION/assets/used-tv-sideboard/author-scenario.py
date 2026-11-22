"""🎬 The scenario of the used TV sideboard: provenance, poses (stored taped shut, carried by two people, in use under a TV, flap open, drawer out), the two clips, the thermal and the structural model.

Executed inside `author-model.py`.
"""

G = 9.80665
FOOT_IDS = [f[0] for f in FEET]
GRIP_IDS = [f[0] for f in GRIPS]


def provenance():
    """📚 Author, method and the references the numbers rely on."""
    refs = [
        ("en-312-2010", "EN 312:2010 Particleboards - Specifications. Type P2, boards for interior fitments (including furniture) for use in dry conditions; requirements for 13-20 mm: bending strength 11 N/mm2, modulus of elasticity in bending 1600 N/mm2, internal bond 0.35 N/mm2."),
        ("en-622-5-2009", "EN 622-5:2009 Fibreboards - Specifications - Part 5: Requirements for dry process boards (MDF); thin high-density boards (HDF, 800-900 kg/m3) as used for nailed-on furniture backs and drawer bottoms."),
        ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values: particleboard 600-650 kg/m3 0.13-0.14 W/(m K) 1700 J/(kg K), fibreboard, polypropylene, beech, steel."),
        ("en-16122-2012", "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (structure and tops, flaps, extension elements such as drawers, stability)."),
        ("en-14749-2016", "EN 14749:2016 Furniture - Domestic and kitchen storage units and worktops - Safety requirements and test methods (requirement levels for the EN 16122 tests, tipping of storage units)."),
        ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (densities of stored materials, Annex A)."),
        ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley: Table A.11 total normal emissivity (anodised aluminium about 0.8, white paint and paper 0.9), solar absorptivity of white coatings 0.2-0.3."),
        ("lowboard-catalogue", "Catalogue-typical flat-pack TV benches of the large Scandinavian and German furniture retailers, 2000-2015 (no single product): 160-180 cm wide, 39-48 cm deep, 40-50 cm high, 16 mm white foil- or melamine-faced particleboard carcass on plastic glide feet, 3 mm HDF back nailed on, drop-down flap doors on flap hinges with stays, one drawer on roller runners of 12.5 mm installation width, round cable hole of 60-80 mm in the back, bar handles with 128 mm hole spacing, 30-45 kg."),
        ("tv-catalogue", "Catalogue-typical 55 inch LED televisions with a centre foot, 2015-2020: 15-19 kg with stand, stand footprint about 0.2 x 0.6 m; AV receivers 3-5 kg."),
        ("clearance-practice", "Practice of municipal reuse depots and second-hand department stores (Gebrauchtwarenkaufhaus, Haushaltsaufloesung): case furniture is emptied, wiped down, its doors and drawers taped shut with 50 mm packing tape for transport, priced with a paper label on the top and stood on its own feet; two people carry a lowboard level by the ends, gripping under the bottom board."),
    ]
    return {"author": "Semio staffage content (used household objects wave, author 05)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in refs], "notes": t("provenance-notes")}


def bindings(kind):
    """🌬️ The exhaustive region bindings of a pose family: stored in the depot, carried, in use in a living room."""
    if kind == "stored":
        b = {r: {"exposedFraction": 1.0, "sunlitFraction": 0.0} for r in REGIONS}
        b["feet"] = {"exposedFraction": 0.75, "sunlitFraction": 0.0, "contact": {"fraction": 0.25, "tempC": 16}}
        return b
    if kind == "carried":
        return {r: {"exposedFraction": 1.0, "sunlitFraction": 0.0} for r in REGIONS}
    b = {
        "top-foil": {"exposedFraction": 0.85, "sunlitFraction": 0.3, "contact": {"fraction": 0.15, "tempC": 25}},
        "carcass-foil": {"exposedFraction": 1.0, "sunlitFraction": 0.05},
        "fronts": {"exposedFraction": 1.0, "sunlitFraction": 0.2},
        "back-panel": {"exposedFraction": 0.6, "sunlitFraction": 0.0},
        "feet": {"exposedFraction": 0.75, "sunlitFraction": 0.0, "contact": {"fraction": 0.25, "tempC": 21}},
        "handles": {"exposedFraction": 1.0, "sunlitFraction": 0.2},
        "knob": {"exposedFraction": 1.0, "sunlitFraction": 0.2},
    }
    return b


def use_loads():
    """📦 The loads of the sideboard in use: a 55 inch TV on its centre foot, an AV receiver in the niche, media behind the flaps, small items in the drawer."""
    def load(lid, kind, kg, bone, x, y, z, refid):
        return {"id": lid, "kind": kind, "title": t(f"load-{lid}"), "bone": bone, "x": x, "y": y, "z": z, "fxN": 0.0, "fyN": round(-kg * G, 2), "source": "authored-estimate", "reference": refid}
    return [
        load("tv", "equipment", 18.0, "carcass", 0.2, g.YT1, 0.0, "tv-catalogue"),
        load("receiver", "equipment", 4.0, "carcass", 0.19, g.YM1, 0.0, "tv-catalogue"),
        load("media-left", "imposed", 6.0, "carcass", 0.19, g.YB1, 0.59, "en-16122-2012"),
        load("media-right", "imposed", 6.0, "carcass", 0.19, g.YB1, -0.59, "en-16122-2012"),
        load("drawer-items", "imposed", 2.0, "drawer", 0.18, 0.02, 0.0, "en-16122-2012"),
    ]


def pose(pid, kind, bones=None, root=None, ground="auto", contacts=None, loads=None, key=False):
    """🧍 One pose."""
    out = {"id": pid, "title": t(f"pose-{pid}"), "description": t(f"pose-{pid}-d")}
    if key:
        out["key"] = True
    out["bones"] = bones or {}
    if root is not None:
        out["root"] = root
    out["ground"] = ground
    out["contacts"] = contacts if contacts is not None else FOOT_IDS
    out["thermal"] = {"regions": bindings(kind), "emitters": {}}
    out["loads"] = loads or []
    return out


def poses():
    """🧍 Stored (default), carried, in use, flap open, drawer out."""
    return [
        pose("stored", "stored"),
        pose("carried", "carried", root={"y": 0.7}, ground="authored", contacts=GRIP_IDS),
        pose("in-use", "use", loads=use_loads()),
        pose("flap-open", "use", bones={"flap-l": {"angleDeg": -90}}, loads=use_loads()),
        pose("drawer-out", "use", bones={"drawer": {"dx": 0.3}}, loads=use_loads()),
    ]


def clips():
    """🎞️ `carry` (stored to carried) and `flap-and-drawer` (the life in use: the left flap dropped and lifted, the drawer pulled and pushed)."""
    carry = {"id": "carry", "title": t("clip-carry"), "description": t("clip-carry-d"), "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.6, "pose": "stored", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}],
             "contacts": [{"anchor": a, "fromS": 0, "toS": 0.6} for a in FOOT_IDS] + [{"anchor": a, "fromS": 0.4, "toS": 3.0} for a in GRIP_IDS],
             "phases": [{"id": "grip", "fromS": 0, "toS": 0.6, "title": t("phase-grip")}, {"id": "lift", "fromS": 0.6, "toS": 3.0, "title": t("phase-lift")}],
             "events": [{"t": 0.4, "kind": "contact-on", "title": t("event-grip"), "target": "grip-left"}, {"t": 0.6, "kind": "contact-off", "title": t("event-lift-off"), "target": "foot-front-left"}],
             "tracks": []}
    life = {"id": "flap-and-drawer", "title": t("clip-life"), "description": t("clip-life-d"), "durationS": 8.0, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 1.5, "pose": "flap-open", "ease": "smooth"}, {"t": 3.0, "pose": "flap-open", "ease": "smooth"}, {"t": 4.5, "pose": "in-use", "ease": "smooth"}, {"t": 5.5, "pose": "drawer-out", "ease": "smooth"}, {"t": 7.0, "pose": "drawer-out", "ease": "smooth"}, {"t": 8.0, "pose": "in-use", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 8.0} for a in FOOT_IDS],
            "phases": [{"id": "flap-down", "fromS": 0, "toS": 1.5, "title": t("phase-flap-down")}, {"id": "flap-held", "fromS": 1.5, "toS": 3.0, "title": t("phase-flap-held")}, {"id": "flap-up", "fromS": 3.0, "toS": 4.5, "title": t("phase-flap-up")}, {"id": "drawer-pull", "fromS": 4.5, "toS": 5.5, "title": t("phase-drawer-pull")}, {"id": "drawer-held", "fromS": 5.5, "toS": 7.0, "title": t("phase-drawer-held")}, {"id": "drawer-push", "fromS": 7.0, "toS": 8.0, "title": t("phase-drawer-push")}],
            "events": [{"t": 0, "kind": "motion", "title": t("event-flap"), "target": "flap-l"}, {"t": 4.5, "kind": "motion", "title": t("event-drawer"), "target": "drawer"}],
            "tracks": []}
    return [carry, life]


def region(rid, kind, material, geometry, L, svf, layer, extra=None):
    """🔥 One passive thermal region with one layer."""
    lid, lkind, lmat, thick, k, rho, c = layer
    hc = round(rho * c * thick)
    out = {"id": rid, "title": t(f"region-{rid}"), "kind": kind, "model": "passive", "material": material, "areaM2": AREA[rid], "characteristicLengthM": L, "geometry": geometry, "skyViewFactor": svf,
           "layers": [{"id": lid, "title": t(f"layer-{lid}"), "kind": lkind, "material": lmat, "thicknessM": thick, "resistanceM2KW": round(thick / k, 6), "heatCapacityJM2K": hc, "source": "literature", "reference": "iso-10456-2007"}],
           "heatCapacityJM2K": hc}
    out.update(extra or {})
    out["source"] = "authored-estimate"
    out["reference"] = "iso-10456-2007"
    return out


def thermal_model():
    """🌡️ Passive regions of the aged foil, the HDF back, the PP feet, the aluminium handles and the beech knob; living-room reference case."""
    regs = [
        region("top-foil", "surface", "chipboard-yellowed-foil", "horizontal-plate-up", 0.4, 0.5, ("top-board", "timber", "chipboard-yellowed-foil", 0.016, 0.13, 650, 1700)),
        region("carcass-foil", "surface", "chipboard-white-foil", "vertical-plate", 0.41, 0.4, ("carcass-board", "timber", "chipboard-white-foil", 0.016, 0.13, 650, 1700), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("fronts", "surface", "chipboard-white-foil", "vertical-plate", 0.39, 0.4, ("front-board", "timber", "chipboard-white-foil", 0.016, 0.13, 650, 1700)),
        region("back-panel", "surface", "hdf-back", "vertical-plate", 0.41, 0.3, ("back-board", "timber", "hdf-back", 0.003, 0.14, 850, 1700)),
        region("feet", "surface", "pp-feet", "cylinder", 0.04, 0.05, ("foot-shell", "polymer", "pp-feet", 0.005, 0.22, 905, 1900)),
        region("handles", "surface", "aluminium-handle", "cylinder", 0.012, 0.4, ("handle-metal", "metal", "aluminium-handle", 0.003, 160, 2700, 900)),
        region("knob", "surface", "beech-knob", "cylinder", 0.03, 0.4, ("knob-wood", "timber", "beech-knob", 0.008, 0.16, 720, 1600)),
    ]
    return {"environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.15, "pressurePa": 101325},
            "operating": [], "regions": regs, "emitters": [], "passengers": [],
            "assumptions": [t("th-assume-materials"), t("th-assume-ageing"), t("th-assume-cases")]}


def structure_model():
    """🏗️ Quasi-static: the rigid carcass on six glide feet or two hands, the flaps hinged at their bottom edge, the drawer on its runners."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [t("st-assume-carcass"), t("st-assume-loads"), t("st-assume-flap"), t("st-assume-carry")], "clinical": False}
