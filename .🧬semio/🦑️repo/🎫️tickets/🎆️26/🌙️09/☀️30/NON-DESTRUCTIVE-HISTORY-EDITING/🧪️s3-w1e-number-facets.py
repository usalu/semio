"""🎛️ S3-W1E: writes the language-agnostic `ActionArgDef::number_facets` corpus (`🛂️manifest/🧫️fixtures/🧫️number-facets`).

An independent Python implementation of the facet mapping (key range, step, look, axis, unit symbols, display factor,
precision, admitted detents, localized hard-limit refusals in display units) computes every expected row; the Rust
(`🧪️tests/🧪️number-facets/🦀️.rs`) and TypeScript (`🟦️.ts`) twins are held to it.
"""
import json
import math
from decimal import ROUND_HALF_UP, Decimal
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🧫️number-facets")
PI = math.pi
RAD_TO_DEG = 180 / math.pi
PRECISION_MAX = 15
REFUSALS = {
    "en": {(True, False): "Must be at least {bound}", (True, True): "Must be greater than {bound}", (False, False): "Must be at most {bound}", (False, True): "Must be less than {bound}"},
    "de": {(True, False): "Muss mindestens {bound} sein", (True, True): "Muss größer als {bound} sein", (False, False): "Darf höchstens {bound} sein", (False, True): "Muss kleiner als {bound} sein"},
}
SYMBOLS = {"deg": "°", "degree": "°", "degrees": "°", "percent": "%"}


def label(en, de):
    return {"native": {"en": en, "de": de}, "reuse": {"en": en, "de": de}}


def twelve(value):
    """🔢️ The contract's twelve-significant-digit text: exact binary value, half up, trailing zeros trimmed."""
    if value == 0:
        return "0"
    exact = Decimal(value)
    rounded = exact.quantize(Decimal(1).scaleb(exact.adjusted() - 11), rounding=ROUND_HALF_UP).normalize()
    text = format(rounded, "f")
    return text.rstrip("0").rstrip(".") if "." in text else text


def control(definition):
    schema, presentation = definition["schema"], (definition.get("presentation") or {}).get("kind")
    if schema["kind"] == "vector":
        return None if presentation == "color" else "vector"
    if schema["kind"] != "number":
        return None
    if presentation in ("slider", "dial", "stepper"):
        return presentation
    if schema.get("integer"):
        return "stepper"
    return "slider" if "min" in schema and "max" in schema else "number"


def facets(definition, locale):
    kind = control(definition)
    if kind is None:
        return None
    schema = definition["schema"]
    low_excluded, high_excluded = schema.get("minExclusive", False), schema.get("maxExclusive", False)
    if kind in ("slider", "dial"):
        low, high = schema.get("softMin", schema.get("min", 0.0)), schema.get("softMax", schema.get("max", 0.0))
    elif kind == "vector":
        low, high = schema.get("min"), schema.get("max")
    else:
        low = None if low_excluded else schema.get("min")
        high = None if high_excluded else schema.get("max")
    out = {"appearance": "dial" if kind == "dial" else "track", "scale": "log" if kind == "slider" and schema.get("scale") == "log" else "linear"}
    if low is not None:
        out["min"] = low
    if high is not None:
        out["max"] = high
    step = schema.get("step")
    if step is not None and math.isfinite(step) and step > 0:
        out["step"] = step
    for key in ("unit", "displayUnit"):
        if key in schema:
            out[key] = SYMBOLS.get(schema[key], schema[key])
    if "displayFactor" in schema:
        out["displayFactor"] = schema["displayFactor"]
    if "precision" in schema:
        out["precision"] = min(schema["precision"], PRECISION_MAX)
    shown_unit = out.get("displayUnit", out.get("unit"))
    factor = schema.get("displayFactor", 1)

    def bound(value, exclusive, below):
        text = twelve(value * factor)
        text = f"{text} {shown_unit}" if shown_unit else text
        entry = {"value": value}
        if exclusive:
            entry["exclusive"] = True
        entry["refusal"] = REFUSALS[locale][(below, exclusive)].replace("{bound}", text)
        return entry

    limits = {}
    if "min" in schema:
        limits["min"] = bound(schema["min"], low_excluded, True)
    if "max" in schema:
        limits["max"] = bound(schema["max"], high_excluded, False)

    def admitted(snap):
        lower, upper = limits.get("min"), limits.get("max")
        if lower and (snap <= lower["value"] if lower.get("exclusive") else snap < lower["value"]):
            return False
        return not (upper and (snap >= upper["value"] if upper.get("exclusive") else snap > upper["value"]))

    kept, previous = [], -math.inf
    for snap in [] if kind == "number" else schema.get("snaps", []):
        if math.isfinite(snap) and snap > previous and (low is None or snap >= low) and (high is None or snap <= high) and admitted(snap):
            kept.append(snap)
            previous = snap
    out["snaps"] = kept
    out["limits"] = limits
    return out


CASES = [
    ("dial-radians-shown-in-degrees", "en", {"id": "/angle", "label": label("Angle", "Winkel"), "schema": {"kind": "number", "step": PI / 180, "unit": "rad", "snaps": [-PI, -PI / 2, 0, PI / 2, PI], "softMin": -PI, "softMax": PI, "displayUnit": "deg", "displayFactor": RAD_TO_DEG}, "presentation": {"kind": "dial"}, "required": True}),
    ("log-slider-soft-travel-inside-an-exclusive-floor", "en", {"id": "/factor", "label": label("Factor", "Faktor"), "schema": {"kind": "number", "min": 0, "minExclusive": True, "max": 100, "step": 0.01, "snaps": [0.25, 0.5, 1, 2, 4, 50], "softMin": 0.1, "softMax": 10, "precision": 2, "scale": "log"}, "presentation": {"kind": "slider"}, "required": True}),
    ("stepper-excluded-floor-is-no-key-end", "de", {"id": "/width", "label": label("Width", "Breite"), "schema": {"kind": "number", "min": 0, "minExclusive": True, "max": 10, "integer": True, "unit": "mm", "snaps": [0, 2, 5, 5, 12]}, "required": True}),
    ("stepper-angle-bounds-in-display-units", "en", {"id": "/turn", "label": label("Turn", "Drehung"), "schema": {"kind": "number", "min": -PI, "max": PI, "step": PI / 180, "unit": "rad", "precision": 0, "displayUnit": "deg", "displayFactor": RAD_TO_DEG}, "presentation": {"kind": "stepper"}, "required": True}),
    ("number-field-display-factor", "de", {"id": "/length", "label": label("Length", "Länge"), "schema": {"kind": "number", "min": 0, "step": 0.001, "unit": "m", "precision": 1, "displayUnit": "cm", "displayFactor": 100}, "required": True}),
    ("vector-axes-share-the-facets", "en", {"id": "/offset", "label": label("Offset", "Versatz"), "schema": {"kind": "vector", "dims": 3, "min": -1, "max": 1, "unit": "percent", "step": 0.1, "snaps": [-1, 0, 1, 2], "precision": 2}, "required": True}),
    ("slider-without-step-and-clamped-precision", "en", {"id": "/mix", "label": label("Mix", "Mischung"), "schema": {"kind": "number", "min": 0, "max": 1, "step": 0, "precision": 40}, "required": True}),
    ("text-has-no-facets", "en", {"id": "/name", "label": label("Name", "Name"), "schema": {"kind": "string"}, "required": True}),
    ("colour-vector-has-no-facets", "en", {"id": "/color", "label": label("Color", "Farbe"), "schema": {"kind": "vector", "dims": 4, "min": 0, "max": 1}, "presentation": {"kind": "color"}, "required": True}),
]

fixture = {
    "$schema": "./🧬️schema/🔣️.json",
    "note": "🎛️ `ActionArgDef::number_facets` / `actionArgNumberFacets`: the UI-contract facets every renderer of an input derives from its descriptor — key range (a slider's or dial's travel; a stepper's, number field's or vector axis' hard bounds, an excluded bound none), step (a non-positive one none), look, axis, unit symbols (deg reads °, percent reads %), display factor, precision (at most 15), the detents the detent law and the limits admit (ascending, inside the key range), and the schema's hard limits with refusals in the case's locale naming the bound in display units (twelve significant digits) beside the shown unit. `facets: null` for an input that is no number. Written by the ticket script `🧪️s3-w1e-number-facets.py` (an independent Python implementation).",
    "cases": [{"name": name, "locale": locale, "def": definition, "facets": facets(definition, locale)} for name, locale, definition in CASES],
}

schema = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "https://json.schemas.assets.semio-tech.com/framework/manifest/number-facets.fixture.json",
    "title": "NumberFacetsFixture",
    "description": "Rows of an action argument descriptor, a locale, and the UI-contract number facets every renderer derives from it (null when the input is no number).",
    "type": "object",
    "additionalProperties": False,
    "required": ["$schema", "note", "cases"],
    "$defs": {
        "bound": {"type": "object", "additionalProperties": False, "required": ["value", "refusal"], "properties": {"value": {"type": "number"}, "exclusive": {"const": True}, "refusal": {"type": "string", "minLength": 1}}},
        "facets": {
            "type": "object",
            "additionalProperties": False,
            "required": ["appearance", "scale", "snaps", "limits"],
            "properties": {
                "min": {"type": "number"},
                "max": {"type": "number"},
                "step": {"type": "number", "exclusiveMinimum": 0},
                "appearance": {"enum": ["track", "dial"]},
                "scale": {"enum": ["linear", "log"]},
                "unit": {"type": "string", "minLength": 1},
                "displayUnit": {"type": "string", "minLength": 1},
                "displayFactor": {"type": "number", "exclusiveMinimum": 0},
                "precision": {"type": "integer", "minimum": 0, "maximum": PRECISION_MAX},
                "snaps": {"type": "array", "items": {"type": "number"}},
                "limits": {"type": "object", "additionalProperties": False, "properties": {"min": {"$ref": "#/$defs/bound"}, "max": {"$ref": "#/$defs/bound"}}},
            },
        },
        "case": {
            "type": "object",
            "additionalProperties": False,
            "required": ["name", "locale", "def", "facets"],
            "properties": {
                "name": {"type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"},
                "locale": {"enum": ["en", "de"]},
                "def": {"type": "object", "required": ["id", "label", "schema"]},
                "facets": {"oneOf": [{"type": "null"}, {"$ref": "#/$defs/facets"}]},
            },
        },
    },
    "properties": {"$schema": {"type": "string"}, "note": {"type": "string"}, "cases": {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/case"}}},
}

(ROOT / "🧬️schema").mkdir(parents=True, exist_ok=True)
(ROOT / "🔣️.json").write_text(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
(ROOT / "🧬️schema" / "🔣️.json").write_text(json.dumps(schema, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(f"[number-facets] {len(CASES)} cases")
