"""🧪️ Session-2 W1-E (G6): writes the axis/display/typed/limits rows and the new pointer/key/value-text/document rows of
`🧫️number-controls`, computed by an independent Python implementation of the number-control laws (IEEE doubles,
`decimal` for the exact-binary half-up rounding), and the matching fixture schema.

Run from the repo root: `python3 .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-w1e-number-controls.py`.
Idempotent: rows are keyed by `case` and replaced.
"""
import json
import math
from decimal import ROUND_HALF_UP, Decimal

ROOT = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls"
DEG = 57.29577951308232
STEP_DEG = 0.017453292519943295
PI = math.pi
DIAL_SNAPS = [-PI, -PI / 2, 0, PI / 2, PI]
LOG_SNAPS = [0.25, 0.5, 1, 2, 4]


def round_fixed(value, digits):
    return float(Decimal(value).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP))


def fixed_text(value, digits):
    text = format(Decimal(value).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP), "f")
    return text[1:] if text.startswith("-") and set(text[1:]) <= set("0.") else text


def twelve(value):
    if value == 0:
        return "0"
    rounded = float(format(Decimal(value).normalize(), ".12g") if False else Decimal(value).quantize(Decimal(1).scaleb(math.floor(math.log10(abs(value))) - 11), rounding=ROUND_HALF_UP))
    text = repr(rounded)
    if "e" in text or "E" in text:
        return text
    return text[:-2] if text.endswith(".0") else text


def decimals(value):
    text = twelve(value)
    mantissa, _, exponent = text.partition("e")
    fraction = len(mantissa.split(".")[1]) if "." in mantissa else 0
    return max(0, fraction - int(exponent or 0))


def axis_position(value, low, high, scale):
    if scale == "log":
        if not (low > 0 and high > 0 and value > 0):
            return 1.0 if low > 0 and value >= high else 0.0
        low, high, value = math.log(low), math.log(high), math.log(value)
    span = high - low
    return min(1.0, max(0.0, (value - low) / span)) if span > 0 else 0.0


def dial_angle(position):
    return (min(1.0, max(0.0, position)) - 0.5) * 2 * math.pi


def dial_position(angle):
    turns = angle / (2 * math.pi) + 0.5
    return turns - math.floor(turns)


def pointer(value, low, high, step, snaps, scale="linear"):
    clamped = min(max(value, low), max(high, low))
    digits = min(12, max(decimals(low), decimals(step)))
    stepped = min(max(round_fixed(low + round_half_away((clamped - low) / step) * step, digits), low), max(high, low)) if step > 0 else clamped
    axis = (lambda at: math.log(at)) if scale == "log" and low > 0 and high > 0 else (lambda at: at)
    radius = abs(axis(high) - axis(low)) * 0.03
    best = None
    for snap in snaps:
        gap = abs(axis(snap) - axis(clamped))
        if gap <= radius and (best is None or gap < best[1]):
            best = (snap, gap)
    return best[0] if best else stepped


def round_half_away(value):
    return math.floor(value + 0.5) if value >= 0 else -math.floor(-value + 0.5)


def key(current, low, high, step, snaps, name, large):
    step = step if step > 0 else 1.0
    origin = low if low is not None else 0.0
    digits = min(12, max(decimals(origin), decimals(step)))

    def clamp(value):
        if low is not None:
            value = max(value, low)
        if high is not None:
            value = min(value, high)
        return value

    def walk(rungs, forward):
        position = (current - origin) / step
        nearest = round_half_away(position)
        base = nearest if abs(position - nearest) <= 1e-9 * max(1, abs(nearest)) else (math.floor(position) if forward else math.ceil(position))
        return clamp(round_fixed(origin + ((base + rungs) if forward else (base - rungs)) * step, digits))

    def settle(value):
        for snap in snaps:
            if abs(value - snap) <= 1e-9 * step * max(1, abs((snap - origin) / step)):
                return snap
        return value

    def page(forward):
        target = walk(10, forward)
        reached = [snap for snap in snaps if (current < snap <= target if forward else target <= snap < current)]
        return (min(reached) if forward else max(reached)) if reached else settle(target)

    return {"increment": lambda: settle(walk(10 if large else 1, True)), "decrement": lambda: settle(walk(10 if large else 1, False)), "pageUp": lambda: page(True), "pageDown": lambda: page(False)}[name]()


def display_text(stored, factor, precision):
    shown = stored * factor if factor is not None else stored
    return twelve(shown) if precision is None else fixed_text(shown, precision)


def typed(value, factor, precision, candidates):
    shown = twelve(value) if precision is None else fixed_text(value, precision)
    for candidate in candidates:
        if display_text(candidate, factor, precision) == shown:
            return candidate
    rounded = value if precision is None else round_fixed(value, precision)
    return rounded / factor if factor is not None else rounded


def crossed(value, low, high, limits):
    if limits is None:
        limits = {"min": None if low is None else {"value": low}, "max": None if high is None else {"value": high}}
    lower, upper = limits.get("min"), limits.get("max")
    if lower is not None and (value <= lower["value"] if lower.get("exclusive") else value < lower["value"]):
        return "min"
    if upper is not None and (value >= upper["value"] if upper.get("exclusive") else value > upper["value"]):
        return "max"
    return None


DIAL = {"type": "slider", "value": PI / 2, "min": -PI, "max": PI, "step": STEP_DEG, "unit": "rad", "snaps": DIAL_SNAPS, "appearance": "dial", "displayUnit": "°", "displayFactor": DEG, "limits": {}}
LOG = {"type": "slider", "value": 1, "min": 0.1, "max": 10, "step": 0.01, "snaps": LOG_SNAPS, "scale": "log", "precision": 2, "limits": {"min": {"value": 0, "exclusive": True, "refusal": "Must be greater than 0"}}}
STEPPER = {"type": "numberStepper", "value": 2, "step": 1, "uniform": True, "snaps": [0, 5, 10], "unit": "mm", "precision": 1}


def rows():
    rung = lambda degrees: round_fixed(-PI + (degrees + 180) * STEP_DEG, 12)
    axis = [
        ("linear-midpoint", 5, 0, 10, "linear"),
        ("log-unit-factor-is-the-middle", 1, 0.1, 10, "log"),
        ("log-quarter-tick", 0.25, 0.1, 10, "log"),
        ("log-half-tick", 0.5, 0.1, 10, "log"),
        ("log-double-tick", 2, 0.1, 10, "log"),
        ("log-quadruple-tick", 4, 0.1, 10, "log"),
        ("log-at-min", 0.1, 0.1, 10, "log"),
        ("log-at-max", 10, 0.1, 10, "log"),
        ("log-clamped-below", 0.05, 0.1, 10, "log"),
        ("dial-right-angle", PI / 2, -PI, PI, "linear"),
    ]
    pointer_rows = [
        ("log-pulled-onto-detent", 0.52, 0.1, 10, 0.01, LOG_SNAPS, "log"),
        ("log-outside-radius-keeps-step", 0.7, 0.1, 10, 0.01, LOG_SNAPS, "log"),
        ("log-radius-is-axis-relative", 3.6, 0.1, 10, 0.01, LOG_SNAPS, "log"),
        ("linear-radius-on-the-same-row", 3.6, 0.1, 10, 0.01, LOG_SNAPS, "linear"),
        ("log-step-cleaned-to-decimals", 0.2549, 0.1, 10, 0.01, [], "log"),
        ("dial-pulled-onto-right-angle", 1.55, -PI, PI, STEP_DEG, DIAL_SNAPS, "linear"),
    ]
    key_rows = [
        ("dial-arrow-steps-one-degree", 0, -PI, PI, STEP_DEG, DIAL_SNAPS, "increment", False),
        ("dial-arrow-settles-onto-a-detent", rung(89), -PI, PI, STEP_DEG, DIAL_SNAPS, "increment", False),
        ("dial-arrow-leaves-a-detent", PI / 2, -PI, PI, STEP_DEG, DIAL_SNAPS, "increment", False),
        ("dial-page-walks-ten-rungs-before-a-far-detent", 0, -PI, PI, STEP_DEG, DIAL_SNAPS, "pageUp", False),
        ("dial-page-stops-on-the-detent-it-reaches", rung(85), -PI, PI, STEP_DEG, DIAL_SNAPS, "pageUp", False),
        ("log-page-stops-on-a-detent", 0.45, 0.1, 10, 0.01, LOG_SNAPS, "pageUp", False),
        ("stepper-page-stops-on-a-detent", 2, None, None, 1, [0, 5, 10], "pageUp", False),
        ("stepper-page-down-stops-on-a-detent", 7, None, None, 1, [0, 5, 10], "pageDown", False),
    ]
    dial_rows = [
        ("centre-points-at-three-oclock", 0.5),
        ("right-angle-points-up", 0.75),
        ("negative-right-angle-points-down", 0.25),
        ("travel-start-on-the-seam", 0.0),
        ("travel-end-on-the-seam", 1.0),
        ("one-degree", 0.5 + 1 / 360),
    ]
    display_rows = [
        ("radians-as-degrees", PI / 2, DEG, None),
        ("negative-half-turn", -PI, DEG, None),
        ("quarter-right-angle", PI / 4, DEG, None),
        ("degrees-at-precision", 0.1, DEG, 1),
        ("without-factor-at-precision", 2.5, None, 2),
        ("factor-only", 0.25, 100, None),
    ]
    typed_rows = [
        ("typed-degrees-keep-the-exact-detent", 90, DEG, None, DIAL_SNAPS),
        ("typed-degrees-divided-back", 45, DEG, None, []),
        ("typed-thirty-degrees", 30, DEG, None, [0]),
        ("typed-value-rounded-to-precision", 2.346, None, 2, []),
        ("retyped-display-keeps-the-current-value", 1, None, 2, [1.004]),
        ("typed-percent", 37.5, 100, None, []),
    ]
    limit_rows = [
        ("negative-factor-crosses-an-excluded-zero", -5, None, None, {"min": {"value": 0, "exclusive": True}}),
        ("zero-crosses-its-excluded-bound", 0, None, None, {"min": {"value": 0, "exclusive": True}}),
        ("beyond-the-soft-travel-is-admitted", 20, 0.1, 10, {"min": {"value": 0, "exclusive": True}}),
        ("an-included-bound-admits-itself", 0, None, None, {"min": {"value": 0}}),
        ("above-the-upper-limit", 11, None, None, {"min": {"value": 0}, "max": {"value": 10}}),
        ("the-travel-is-the-limit-without-limits", 12, 0, 10, None),
        ("open-limits-admit-anything", 1e6, -PI, PI, {}),
    ]
    value_texts = [
        ("dial-in-degrees", dict(DIAL), "90 °", 90, -180, 180),
        ("log-slider-at-precision", dict(LOG), "1.00", 1, 0.1, 10),
        ("stepper-with-unit", dict(STEPPER), "2.0 mm", 2, None, None),
        ("stepper-with-display-factor", {"type": "numberStepper", "value": 0.5, "step": 0.01, "uniform": True, "displayFactor": 100, "displayUnit": "%"}, "50 %", 50, None, None),
    ]
    documents = [
        ("dial-with-snaps", dict(DIAL), None),
        ("log-slider", dict(LOG), None),
        ("stepper-detents", dict(STEPPER), None),
        ("stepper-detent-outside-bounds", {"type": "numberStepper", "value": 2, "step": 1, "uniform": True, "min": 0, "max": 4, "snaps": [5]}, "invalidSnaps"),
        ("log-slider-non-positive-travel", {**LOG, "min": 0, "limits": None}, "invalidNumberRange"),
        ("zero-display-factor", {"type": "slider", "value": 1, "min": 0, "max": 2, "step": 1, "displayFactor": 0}, "invalidNumberRange"),
        ("limits-exclude-the-travel", {"type": "slider", "value": 1, "min": 0, "max": 2, "step": 1, "limits": {"min": {"value": 0, "exclusive": True}}}, "invalidNumberRange"),
        ("inverted-limits", {"type": "input", "kind": "number", "value": "1", "limits": {"min": {"value": 5}, "max": {"value": 1}}}, "invalidNumberRange"),
        ("number-input-with-display-factor", {"type": "input", "kind": "number", "value": "0.5", "displayFactor": 100, "precision": 1}, None),
    ]
    out = {
        "axis": [{"case": c, "value": v, "min": lo, "max": hi, "scale": sc, "position": axis_position(v, lo, hi, sc)} for c, v, lo, hi, sc in axis],
        "pointer": [{"case": c, "value": v, "min": lo, "max": hi, "step": st, "snaps": sn, "scale": sc, "expected": pointer(v, lo, hi, st, sn, sc)} for c, v, lo, hi, st, sn, sc in pointer_rows],
        "keys": [{"case": c, "current": cur, "min": lo, "max": hi, "step": st, "snaps": sn, "key": k, "large": lg, "expected": key(cur, lo, hi, st, sn, k, lg)} for c, cur, lo, hi, st, sn, k, lg in key_rows],
        "dial": [{"case": c, "position": p, "angle": dial_angle(p), "back": dial_position(dial_angle(p))} for c, p in dial_rows],
        "display": [{"case": c, "stored": v, "factor": f, "precision": p, "text": display_text(v, f, p)} for c, v, f, p in display_rows],
        "typed": [{"case": c, "typed": v, "factor": f, "precision": p, "candidates": cand, "expected": typed(v, f, p, cand)} for c, v, f, p, cand in typed_rows],
        "limits": [{"case": c, "value": v, "min": lo, "max": hi, "limits": li, "crossed": crossed(v, lo, hi, li)} for c, v, lo, hi, li in limit_rows],
        "valueTexts": [{"case": c, "component": comp, "valueText": text, "valueNow": now, "valueMin": lo, "valueMax": hi} for c, comp, text, now, lo, hi in value_texts],
        "documents": [{"case": c, "component": {k: v for k, v in comp.items() if v is not None}, "violation": vio} for c, comp, vio in documents],
    }
    return out


def literal(value):
    if isinstance(value, dict):
        return "{ " + ", ".join(f"{json.dumps(k, ensure_ascii=False)}: {literal(v)}" for k, v in value.items()) + " }" if value else "{}"
    if isinstance(value, list):
        return "[" + ", ".join(literal(v) for v in value) + "]"
    if isinstance(value, float) and value.is_integer() and abs(value) < 1e15:
        return str(int(value))
    return json.dumps(value, ensure_ascii=False)


def main():
    path = f"{ROOT}/🔣️.json"
    with open(path, encoding="utf-8") as handle:
        fixture = json.load(handle)
    for section, generated in rows().items():
        existing = fixture.setdefault(section, [])
        names = {row["case"] for row in generated}
        fixture[section] = [row for row in existing if row["case"] not in names] + generated
    fixture["note"] = (
        "🎚️ The number-control laws every renderer shares: the slider detent law (`snaps` finite, strictly ascending, inside min..=max), "
        "the axis law (a value's position on a linear or log axis and back, `min`/`max` exactly at the ends), the dial law (a position's needle angle counter-clockwise from three o'clock, the travel's centre at 0, and back, the seam at nine o'clock), the pointer law (clamp, quantize onto the step ladder from min "
        "cleaned to its decimals, then pull onto the nearest snap within 3 % of the axis — the log axis for `log` —, first snap wins a tie), the keyboard law of sliders, steppers "
        "and number fields (arrows walk one rung of the step ladder from min — from 0 without one —, a large arrow ten rungs, and never stop on a detent off their path; "
        "from an off-ladder detent the first rung beyond it is one rung; page keys walk ten rungs and stop on the first detent they reach; a key landing within ladder "
        "tolerance of a detent lands on it exactly; Home and End reach the bounds and keep the value without one; an invalid step walks rungs of one; results clamp to "
        "the bounds and clean to the decimals of min and step), the display law (shown = stored × displayFactor at the precision or in the twelve-digit format; a typed "
        "number whose display text equals a candidate's keeps that exact stored value, any other is rounded to the precision and divided back), the hard-bound law "
        "(the limit a typed value crosses, the lower first, an excluded limit refusing itself, the inclusive min/max without declared limits), the fixed-precision law "
        "(exactly `precision` fraction digits, ties away from zero as JavaScript's toFixed at every magnitude, unsigned zero, precision capped at 15, magnitudes from "
        "1e21 fall back to the twelve-digit format), the spoken value of a range control (display numbers and text with its unit) and the validator's verdict on "
        "documents carrying them (`invalidSnaps`, `invalidNumberRange`: a non-positive display factor, a log travel that is not strictly positive, limits that are "
        "inverted or do not admit the key range and every detent). Answered by `snaps_are_valid`/`slider_axis_position`/`slider_axis_value`/`dial_angle`/`dial_position`/`slider_pointer_value`/"
        "`slider_adjacent_snap`/`ui_number_key_value`/`ui_number_display_text`/`ui_number_typed_value`/`ui_number_crossed_bound`/`format_ui_number_fixed`/"
        "`round_ui_number`/`accessibility_value`/`validate_snapshot` in Rust and their camelCase twins in TypeScript; decimal.js is the third-party rounding oracle, "
        "d3-scale (`scaleLog`, `scaleLinear`) the third-party axis and display oracle, and `🧪️s2-w1e-number-controls.py` in the authoring ticket an independent "
        "Python implementation that wrote the expectations."
    )
    order = ["$schema", "note", "validity", "axis", "dial", "pointer", "adjacent", "keys", "display", "typed", "limits", "fixed", "valueTexts", "documents"]
    lines = ["{", f'  "$schema": {json.dumps(fixture["$schema"], ensure_ascii=False)},', f'  "note": {json.dumps(fixture["note"], ensure_ascii=False)},']
    sections = [name for name in order[2:]]
    for index, name in enumerate(sections):
        body = ",\n".join(f"    {literal(row)}" for row in fixture[name])
        lines.append(f'  "{name}": [\n{body}\n  ]' + ("," if index + 1 < len(sections) else ""))
    lines.append("}")
    with open(path, "w", encoding="utf-8") as handle:
        handle.write("\n".join(lines) + "\n")
    schema_path = f"{ROOT}/🧬️schema/🔣️.json"
    with open(schema_path, encoding="utf-8") as handle:
        schema = json.load(handle)
    schema["description"] = "The shared number-control laws: slider detent validity, the linear and log axis, pointer resolution, page-key detent stops, the keyboard law, the display factor and its exact read-back, hard bounds, fixed-precision formatting and rounding, spoken values, and the validator's verdict on documents carrying them."
    schema["required"] = order
    defs = schema["$defs"]
    defs["scale"] = {"enum": ["linear", "log"]}
    defs["bound"] = {"type": "object", "additionalProperties": False, "required": ["value"], "properties": {"value": {"type": "number"}, "exclusive": {"type": "boolean"}, "refusal": {"type": "string"}}}
    defs["limits"] = {"type": "object", "additionalProperties": False, "properties": {"min": {"$ref": "#/$defs/bound"}, "max": {"$ref": "#/$defs/bound"}}}
    props = schema["properties"]
    case = {"$ref": "#/$defs/case"}
    nullable = {"type": ["number", "null"]}
    props["axis"] = {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["case", "value", "min", "max", "scale", "position"], "properties": {"case": case, "value": {"type": "number"}, "min": {"type": "number"}, "max": {"type": "number"}, "scale": {"$ref": "#/$defs/scale"}, "position": {"type": "number", "minimum": 0, "maximum": 1}}}}
    props["dial"] = {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["case", "position", "angle", "back"], "properties": {"case": case, "position": {"type": "number", "minimum": 0, "maximum": 1}, "angle": {"type": "number"}, "back": {"type": "number", "minimum": 0, "maximum": 1}}}}
    props["pointer"]["items"]["properties"]["scale"] = {"$ref": "#/$defs/scale"}
    props["display"] = {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["case", "stored", "factor", "precision", "text"], "properties": {"case": case, "stored": {"type": "number"}, "factor": nullable, "precision": {"type": ["integer", "null"], "minimum": 0, "maximum": 15}, "text": {"type": "string"}}}}
    props["typed"] = {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["case", "typed", "factor", "precision", "candidates", "expected"], "properties": {"case": case, "typed": {"type": "number"}, "factor": nullable, "precision": {"type": ["integer", "null"], "minimum": 0, "maximum": 15}, "candidates": {"$ref": "#/$defs/snaps"}, "expected": {"type": "number"}}}}
    props["limits"] = {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["case", "value", "min", "max", "limits", "crossed"], "properties": {"case": case, "value": {"type": "number"}, "min": nullable, "max": nullable, "limits": {"oneOf": [{"type": "null"}, {"$ref": "#/$defs/limits"}]}, "crossed": {"enum": [None, "min", "max"]}}}}
    value_text_props = props["valueTexts"]["items"]["properties"]
    value_text_props["component"]["properties"]["type"] = {"enum": ["input", "numberStepper", "slider"]}
    value_text_props["valueNow"] = nullable
    value_text_props["valueMin"] = nullable
    value_text_props["valueMax"] = nullable
    props["documents"]["items"]["properties"]["violation"] = {"enum": [None, "invalidSnaps", "invalidNumberRange"]}
    schema["properties"] = {name: props[name] for name in order}
    with open(schema_path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")


main()
