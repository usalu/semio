"""📏 Physical limits of the numeric snapshot fields: `decide` maps (unit, name, label) to `minimum`/`maximum` or `exclusiveMinimum`/`maximum`. The bounds describe the physical regime (lengths above zero, ratios 0..1, angles in their geometric range, absolute zero), never a normative value, and are wide enough that no code-specific value is excluded. The schema JSON is the source of truth; `apply` inserts the members as text next to the node's `type`."""
import json, math, os, re

sys_path = os.path.dirname(os.path.abspath(__file__))
PI = math.pi
BOUND_KEYS = ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum")

#: unit symbol -> (class, display name). Suffix of the wire name wins when no unit is declared.
UNIT_CLASS = {
    "m": "length", "mm": "length_mm", "cm": "length_cm", "m²": "area", "m2": "area", "mm²": "area_mm2", "m³": "volume", "m3": "volume", "m⁴": "inertia", "m4": "inertia",
    "m⁶": "warping", "m2/m": "area_per_length", "m²/m": "area_per_length", "1/m": "curvature", "1/m²": "curvature",
    "N": "force", "N·m": "moment", "Nm": "moment", "N·m/m": "moment_per_length", "N·m²": "stiffness_ei", "N·m²/m": "stiffness_ei", "N/m": "line_load", "N/m³": "unit_weight",
    "Pa": "stress", "MPa": "stress_mpa", "N/mm²": "stress_mpa", "kPa": "stress_kpa", "kg": "mass", "kg/m": "mass_per_length", "kg/m²": "mass_per_area", "kg/m³": "density", "kg/m3": "density",
    "rad": "angle_rad", "°": "angle_deg", "deg": "angle_deg", "s": "time_s", "min": "time_min", "d": "time_d", "a": "time_a", "1/a": "rate_a", "Hz": "frequency",
    "K": "temperature_k", "°C": "temperature_c", "m/s": "speed", "m/s²": "acceleration", "m/s2": "acceleration", "J": "moment", "W": "power", "W/m²": "power", "W/(m·K)": "conductivity", "J/(kg·K)": "specific_heat",
    "1": "one", "": "one",
}
SUFFIX_CLASS = (
    ("NM3", "unit_weight"), ("Nm2", "stiffness_ei"), ("NPerM", "line_load"), ("Mpa", "stress_mpa"), ("MPa", "stress_mpa"), ("Kpa", "stress_kpa"), ("PerM", "line_load"),
    ("Pa", "stress"), ("Nm", "moment"), ("MS", "speed"), ("M2", "area"), ("M3", "volume"), ("M4", "inertia"), ("Mm", "length_mm"), ("Rad", "angle_rad"), ("Deg", "angle_deg"),
    ("PeriodS", "time_s"), ("TimeS", "time_s"), ("DurationS", "time_s"), ("Years", "time_a"), ("Kg", "mass"), ("KgM3", "density"), ("Percent", "percent"), ("M", "length"),
)
#: unit class -> (hard cap of the physical regime, SI value of one stored unit for the cap)
CAP = {
    "length": 1.0e4, "length_mm": 1.0e7, "length_cm": 1.0e6, "area": 1.0e8, "area_mm2": 1.0e14, "volume": 1.0e9, "inertia": 1.0e4, "warping": 1.0e8, "area_per_length": 1.0e4, "curvature": 1.0e9,
    "force": 1.0e12, "moment": 1.0e13, "moment_per_length": 1.0e11, "stiffness_ei": 1.0e16, "line_load": 1.0e14, "unit_weight": 1.0e9,
    "stress": 1.0e12, "stress_mpa": 1.0e6, "stress_kpa": 1.0e9, "mass": 1.0e12, "mass_per_length": 1.0e7, "mass_per_area": 1.0e6, "density": 2.3e4,
    "time_s": 1.0e10, "time_min": 1.0e7, "time_d": 1.0e5, "time_a": 1.0e4, "rate_a": 1.0e9, "frequency": 1.0e6, "temperature_k": 1.0e4, "temperature_c": 1.0e4, "speed": 1.0e4, "acceleration": 1.0e4,
    "energy": 1.0e15, "power": 1.0e12, "conductivity": 1.0e3, "specific_heat": 1.0e5,
}
#: classes that are an absolute quantity or an intrinsically positive magnitude (never negative)
MAGNITUDE = {"area", "area_mm2", "volume", "inertia", "warping", "area_per_length", "mass", "mass_per_length", "mass_per_area", "density", "unit_weight", "frequency", "conductivity", "specific_heat", "stiffness_ei", "temperature_k"}


def tokens(name):
    return [t.lower() for t in re.findall(r"[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+", name)]


def unit_class(unit, name, text=""):
    if unit is not None and unit in UNIT_CLASS:
        return UNIT_CLASS[unit]
    for suffix, cls in SUFFIX_CLASS:
        if name.endswith(suffix) and len(name) > len(suffix) and name[-len(suffix) - 1].islower():
            return cls
    return None


def has(words, name, label):
    bag = set(tokens(name)) | {re.sub(r"[^a-z0-9]", "", w) for w in label.lower().split()}
    return bool(bag & set(words))


def sentinel_zero(label, doc):
    return bool(re.search(r"\(0\b|\b0 =|\b0 means|\b0 for|0 none|\(0/1\)|0 = ", f"{label} {doc}"))


def decide_integer(node, name, label, doc):
    """Integers keep the bounds they have: they are the storage width of the Rust field (`u8`, `u32`, `usize`) and the cross-language width tests pin the full range, so narrowing them changes types and tests. A bound that is missing is a finding of `verify limits`."""
    return {}


def positive(cls, name, label, doc, extra=()):
    """True when a quantity of this class is a strictly positive magnitude (and a zero is not part of its documented sentinel)."""
    if sentinel_zero(label, doc):
        return False
    return has(("thickness", "diameter", "width", "depth", "height", "span", "length", "radius", "spacing", "perimeter", "pitch", "gauge", "modulus", "density", "mass", "frequency", "inertia", "stiffness", "period", "area", "weight") + tuple(extra), name, label)


SIGNED_LENGTH = ("position", "offset", "eccentricity", "eccentric", "level", "elevation", "centre", "center", "shift", "altitude", "precamber", "deflection", "drift", "displacement", "settlement", "hogging", "coordinate", "tolerance", "difference", "elongation", "start")
SIGNED_FORCE = ("force", "moment", "shear", "axial", "normal", "tension", "reaction", "horizontal", "lateral", "longitudinal", "eccentric", "fx", "fy", "fz", "x", "y", "z", "load", "pressure", "stress", "sigma", "torque", "momentc1", "demand")
MAGNITUDE_FORCE = ("resistance", "capacity", "strength", "weight", "preload", "yield", "modulus", "stiffness", "cohesion", "fck", "fyk", "fu", "fy", "ultimate", "gk", "qk", "self", "mass", "rd")


def dimensional(cls, name, label, doc):
    cap = CAP.get(cls)
    sentinel = sentinel_zero(label, doc)
    if cls in ("length", "length_mm", "length_cm", "curvature"):
        coordinate = re.search(r"(?:Position|Offset|Centre|Center|Start|Level|Elevation|Shift|Coordinate)[XYZ]?(?:M|Mm)?$|^(?:x|y|z)(?:M|Mm)?$|^(?:locationX|loadX|loadZ|positionX|positionY|fibreY|fibreZ|stressX|stressZ|craneX)$|(?:X|Y|Z)(?:M|Mm)?$", name)
        if (set(tokens(name)) & set(SIGNED_LENGTH)) or coordinate:
            return {"minimum": -cap, "maximum": cap}
        if positive(cls, name, label, doc):
            return {"exclusiveMinimum": 0, "maximum": cap}
        return {"minimum": 0, "maximum": cap}
    if cls in ("area", "area_mm2", "volume", "inertia", "warping", "area_per_length", "mass", "mass_per_length", "mass_per_area", "density", "frequency", "conductivity", "specific_heat"):
        if not sentinel and has(("gross", "plan", "compartment", "tributary", "bearing", "loaded", "cross", "section", "density", "unit", "frequency", "mass", "volume", "inertia", "area"), name, label) and not has(("steel", "reinforcement", "bar", "tension", "compression", "stirrup", "as", "rebar", "diagonal", "tendon", "net", "vent"), name, label):
            return {"exclusiveMinimum": 0, "maximum": cap}
        return {"minimum": 0, "maximum": cap}
    if cls == "unit_weight":
        return {"exclusiveMinimum": 0, "maximum": cap} if not sentinel else {"minimum": 0, "maximum": cap}
    if cls == "stiffness_ei":
        return {"exclusiveMinimum": 0, "maximum": cap} if not sentinel else {"minimum": 0, "maximum": cap}
    if cls in ("force", "moment", "moment_per_length", "line_load"):
        if has(MAGNITUDE_FORCE, name, label) and not has(("moment", "force", "horizontal"), name, ""):
            return {"minimum": 0, "maximum": cap}
        if has(("resistance", "capacity", "weight", "preload", "wheel", "axle", "tandem", "udl", "yield"), name, label):
            return {"minimum": 0, "maximum": cap}
        return {"minimum": -cap, "maximum": cap}
    if cls in ("stress", "stress_mpa", "stress_kpa"):
        if has(("cohesion", "resistance", "bond", "cptqc", "qc", "undrained", "shaft", "capacity", "usedyield"), name, label):
            return {"minimum": 0, "maximum": cap}
        if has(("strength", "modulus", "yield", "fck", "fyk", "fu", "fy", "ultimate", "fcd", "fpk", "fuk", "detail", "category", "fyp", "fup", "fb", "fyd"), name, label) or re.match(r"^f[A-Z]|^e[A-Z]?[a-z]*Modulus", name):
            return {"exclusiveMinimum": 0, "maximum": cap} if not sentinel else {"minimum": 0, "maximum": cap}
        return {"minimum": -cap, "maximum": cap}
    if cls == "angle_rad":
        return {"minimum": 0, "maximum": PI / 2} if set(tokens(name)) & {"friction", "slope", "pitch", "inclination", "web", "guy", "brace", "apex"} else {"minimum": -2 * PI, "maximum": 2 * PI}
    if cls == "angle_deg":
        return {"minimum": 0, "maximum": 90} if set(tokens(name)) & {"friction", "slope", "pitch", "inclination", "web", "guy", "brace", "hopper", "phi", "prime", "delta"} else {"minimum": -360, "maximum": 360}
    if cls in ("time_s", "time_min", "time_d", "time_a", "rate_a"):
        return {"minimum": 0, "maximum": cap} if sentinel or has(("start", "age", "delay"), name, label) else {"exclusiveMinimum": 0, "maximum": cap}
    if cls == "temperature_k":
        return {"minimum": -cap, "maximum": cap} if has(("delta", "change", "difference", "gradient", "range"), name, label) else {"minimum": 0, "maximum": cap}
    if cls == "temperature_c":
        return {"minimum": -cap, "maximum": cap} if has(("delta", "change", "difference", "gradient", "range"), name, label) else {"minimum": -273.15, "maximum": cap}
    if cls == "speed":
        return {"minimum": 0, "maximum": cap}
    if cls == "acceleration":
        return {"minimum": -cap, "maximum": cap}
    if cls in ("energy", "power"):
        return {"minimum": 0, "maximum": cap}
    if cls == "percent":
        return {"minimum": 0, "maximum": 100}
    return None


FRACTION_WORDS = ("fraction", "share", "ratio", "lapshare", "losses", "loss", "elongation", "strain", "humidity", "filling", "damping", "porosity", "utilisation")
NOT_FRACTION = ("stiffness", "period", "deflection", "limit", "stress", "periodratio", "overstrength", "amplifier", "slenderness", "aspect", "span", "moment", "bridge", "alpha", "traffic", "length")


def dimensionless(name, label, doc, unit):
    text = f"{label} {doc}"
    tk = set(tokens(name))
    sentinel = sentinel_zero(label, doc)
    if unit in ("%", "percent") or re.search(r"Percent$", name) or "percent" in tk or re.search(r"\bin percent\b|\[%\]|\(%\)", text):
        return {"minimum": 0, "maximum": 100}
    if re.search(r"coefficient c_?pe|pressure coefficient|c_pe|c_pi|\bcpe\b", text, re.I) or tk & {"cpe", "cpi"} or re.match(r"^cP[ei]", name):
        return {"minimum": -5.0, "maximum": 5.0}
    if re.search(r"poisson", text, re.I) or tk & {"poisson", "nu"} and re.search(r"poisson", text, re.I):
        return {"minimum": -1.0, "maximum": 0.5}
    if re.search(r"0\s*(?:…|\.\.\.|to)\s*1\b|fraction|share|filling ratio|relative humidity|loss ratio|prestress loss|damping ratio|damping|reinforcement ratio|ρ|diagonal reinforcement|cracked|portion|lapped|ratio of the (?:gross|effective)|share of", text, re.I) and not re.search(r"hydrostatic head|stiffness ratio K_b|Lateral to vertical|slenderness|period ratio|Overstrength|moment ratio|Stress ratio|stress level|deflection limit", text, re.I):
        if re.search(r"relative humidity|\bRH\b", text):
            return {"minimum": 0, "maximum": 1.0}
        if re.search(r"damping|ζ|ξ", text) and not re.search(r"0\s*(?:…|to)\s*1", text):
            return {"minimum": 0, "maximum": 1.0}
        return {"minimum": 0, "maximum": 1.0}
    if re.search(r"angle|friction coefficient|\bμ\b|friction|slip factor", text, re.I) and re.search(r"coefficient|factor|μ", text):
        return {"minimum": 0, "maximum": 5.0}
    if re.search(r"partial factor|γ|gamma", text) or tk & {"gamma", "partialfactor"}:
        return {"exclusiveMinimum": 0, "maximum": 100.0} if not sentinel else {"minimum": 0, "maximum": 100.0}
    if re.search(r"\bψ\b|\bpsi\b|moment ratio", text, re.I) and re.search(r"end moment|stress ratio|ψ\b", text):
        return {"minimum": -1.0, "maximum": 1.0} if re.search(r"end moment|stress ratio", text, re.I) else {"minimum": 0, "maximum": 1.0}
    return None


def P(high):
    return {"exclusiveMinimum": 0, "maximum": high}


def N(high):
    return {"minimum": 0, "maximum": high}


def S(high):
    return {"minimum": -high, "maximum": high}


def R(low, high):
    return {"minimum": low, "maximum": high}


#: Fields whose unit or meaning is not derivable from the unit symbol, the wire-name suffix or the label: `name` or `Def.name` -> bounds. Zero is closed (`N`) where the schema text documents `0 = ...` or the field is a declared optional input.
OVERRIDES = {
    "deflectionLimitRatio": P(1e5), "deflectionLimitRatioW3": P(1e5), "deflectionLimit": P(1e5), "frameHeight": N(1e4), "frameDrift": S(1e3), "storeyHeight": N(1e4), "storeyDrift": S(1e3),
    "materialStrengthK": N(1e12), "materialConversionFactor": N(10), "materialDesignStrength": N(1e12), "nominalDimension": N(1e4), "dimensionTolerance": S(1e3), "designDimension": N(1e4),
    "memberAlphaCr": N(1e4), "secondOrderAnalysis": R(0, 1), "influence": S(1e6), "trackGauge": N(10), "lineSpeed": N(1e3), "ballastDepth": N(10), "deckMass": N(1e7), "naturalFrequency": N(1e3),
    "endRotation": S(PI), "basicWindSpeed": N(1e3), "FatigueSpectrum.cycles": P(1e15), "TestSeries.mean": S(1e12), "stdDev": N(1e12), "variationKnown": N(10), "TestSeries.conversionFactor": P(10),
    "required": N(1e12), "theoryVariation": N(10), "value": S(1e18), "betaComputed": S(100),
    "sptN": N(1e3), "cptNkt": N(100), "bendingStiffnessEi": N(1e16), "horizontalSubgradeModulus": N(1e12), "dynamicTestCount": N(1e4), "hammerEfficiency": R(0, 1), "ocr": P(100),
    "seismicKh": R(0, 1), "seismicKv": R(-1, 1), "testFactor": P(10), "voidRatio": P(20), "compressionIndex": N(10), "swellingIndex": N(10), "secondaryCompressionIndex": N(1), "designLife": N(1e10),
    "permeability": N(10), "wellCapacity": N(1e3), "measurement": N(1e12),
    "cE": P(1e3), "cT": P(1e3), "cS": P(1e3), "cD": P(1e3), "strouhal": N(10), "assumedCscd": P(1e3), "alphaMicro": P(1e3), "TieRun.psi": R(0, 1),
    "factorVertical": N(10), "factorLongitudinal": N(10), "factorLateral": N(10), "kst": N(1e4), "orographyFactor": P(1e3), "tMax": R(-273.15, 1e4), "tMin": R(-273.15, 1e4), "t0": R(-273.15, 1e4),
    "assumedHNet": N(1e12), "fireOpeningFactor": P(10), "fireThermalInertia": P(1e5), "fireLoadDensityQf": P(1e13), "assumedQfD": P(1e13), "siloBulkDensity": P(1e6), "siloMu": N(5), "siloK": P(10), "railAlpha": P(10),
    "fCk": P(1e12), "fYk": P(1e12), "eS": P(1e13), "ReinforcementGrade.k": P(10), "epsUk": P(1), "fPk": P(1e12), "fP01k": P(1e12), "hdOverH": N(1e3), "deltaCDev": N(10),
    "rambergOsgoodN": P(1e3), "toughnessStressRatio": N(10), "slipFactorKs": P(10), "SteelJoint.alphaFactor": N(100), "JointRow.alphaFactor": N(100), "stiffnessRatio": N(1e3), "transformationParameter": N(10),
    "jointCoefficient": N(10), "mu0": N(10), "kSigma": P(1e3), "springFactor": R(0, 1), "purlinLateralRatio": N(10), "stiffenerAlpha": P(10), "flangeInducedK": N(10), "alphaCr": N(1e4),
    "lateralPressureCoefficient": N(10), "SiloShell.k": P(10), "boundaryFactorAxial": N(100), "boundaryFactorHoop": N(100), "boundaryFactorShear": N(100), "threadFactor": N(10), "lambda": P(10), "phi2": P(10),
    "observedTraffic": N(1e12), "secondLaneTraffic": N(1e12), "lambda1": N(10), "lambdaMax": N(10), "forceCoefficient": P(10), "dynamicFactor": P(10), "slenderness": N(1e4), "bendingStiffness": N(1e16),
    "kFactor": N(1e3), "kwFactor": N(100), "lateralForceCoefficient": N(10), "permittedAmplitudeRatio": N(10), "ringMode": N(1e3), "interlockBendingFactor": N(10), "interlockStiffnessFactor": N(10),
    "CraneRunway.phi": P(10), "craneNu": N(10), "craneKq": N(10), "damageExponent": N(100), "fatiguePhi": N(10), "WeldGroup.betaW": P(100), "weldBetaW": N(10), "ParametricSection.tolerance": N(1),
    "FatigueBand.cycles": P(1e15), "FatigueStress.cycles": P(1e15), "concentration": N(100), "nCycles": P(1e15), "phiInfinity": N(20),
    "slope": N(10), "psi2": R(0, 1), "bridgeBeta": N(100), "bridgeA": S(1e3), "bridgeB": S(1e3),
    "c1": P(100), "m1": N(100), "m2": N(100),
    "vS30MS": N(1e4), "nSpt": N(1e3), "cUKpa": N(1e6), "alluviumVSMS": N(1e4), "belowAlluviumVSMS": N(1e4), "q0": P(100), "alphaUOverAlpha1": P(100), "kW": P(10), "omegaWd": N(10),
    "wallStartXM": S(1e4), "wallStartYM": S(1e4), "ct": P(1), "nu": R(0, 1), "masonryWallAreaRatio": R(0, 1), "accidentalEccentricityRatio": R(0, 1), "planEnvelopeGapRatio": R(0, 1),
    "nonstructuralWeightN": N(1e12), "nonstructuralQ": P(100), "nonstructuralResistanceN": N(1e12), "overturningBaseXM": N(1e4), "overturningBaseYM": N(1e4), "periodRatio": P(1e3), "pierShearRdN": N(1e12),
    "axialForceN": S(1e12), "restrainedBarCount": N(1e3), "shearCrackingBeforeYield": R(0, 1), "shearForceDemandN": N(1e12), "qNominal": P(100), "kFoundation": N(1e16), "kSoil": N(1e16),
    "liqN160": N(1e3), "liqMagnitude": R(0, 10), "bearingAxialN": S(1e12), "bearingShearN": N(1e12), "bearingNMaxN": N(1e12), "pileHeadShearN": N(1e12), "pileHeadFixed": R(0, 1), "r": P(100), "n160": N(1e3),
    "scaleFactor": P(1e6), "backfillGamma": P(1e9), "importanceClass": R(1, 4), "logDecrement": N(10), "topographicAmplification": P(10),
}


def override(name, owner):
    return OVERRIDES.get(f"{owner}.{name}") or OVERRIDES.get(name)


def decide(unit, name, label, doc, node, owner=""):
    """{bound key: value} to add to the node; `Def.name` overrides win, then the unit class, then the bare-name overrides, then the dimensionless rules; None when no rule applies (the caller reports it)."""
    if node.get("type") == "integer":
        return decide_integer(node, name, label, doc)
    if f"{owner}.{name}" in OVERRIDES:
        return dict(OVERRIDES[f"{owner}.{name}"])
    cls = unit_class(unit, name, label)
    if cls is None and re.search(r"N/m³|N/m3|unit weight", f"{label} {doc}") and name != "gammaD":
        cls = "unit_weight"
    if cls and cls != "one":
        return dimensional(cls, name, label, doc)
    if name in OVERRIDES:
        return dict(OVERRIDES[name])
    return dimensionless(name, label, doc, unit)


def has_bounds(node):
    low = "minimum" in node or "exclusiveMinimum" in node
    high = "maximum" in node or "exclusiveMaximum" in node
    return low and high


def numeric_nodes(doc):
    """[(def key or '$root', property, property node, number node)] of every numeric member of a schema document."""
    out = []
    containers = [("$root", doc.get("properties", {}))] + [(key, node.get("properties", {})) for key, node in doc.get("$defs", {}).items()]
    for owner, props in containers:
        for name, prop in props.items():
            node = prop["items"] if prop.get("type") == "array" and isinstance(prop.get("items"), dict) else prop
            if node.get("type") in ("number", "integer"):
                out.append((owner, name, prop, node))
    return out


def unbounded_fields(snapshot):
    """['Def.property'] of the numeric members of a snapshot schema without both a lower and an upper bound: the rule `limits` of `evolve.py verify`."""
    return [f"{owner}.{name}" for owner, name, _, node in numeric_nodes(snapshot) if not has_bounds(node)]
