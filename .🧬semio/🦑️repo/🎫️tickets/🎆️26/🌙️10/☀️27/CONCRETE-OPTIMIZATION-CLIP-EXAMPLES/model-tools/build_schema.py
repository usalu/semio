import json
from pathlib import Path

target = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🧬️schema/🔣️.schema.json")


def strict(properties, required=None):
    return {"type": "object", "required": list(properties if required is None else required), "additionalProperties": False, "properties": properties}


def reference(name):
    return {"$ref": f"#/$defs/{name}"}


number = {"type": "number"}
positive = {"type": "number", "exclusiveMinimum": 0}
fraction = {"type": "number", "minimum": 0, "maximum": 1}
text = {"type": "string", "minLength": 1}
pair = lambda item=number: {"type": "array", "minItems": 2, "maxItems": 2, "items": item}
array = lambda item, low=1, high=None: {"type": "array", "minItems": low, **({} if high is None else {"maxItems": high}), "items": item}

sections = {
    "room": ["width", "depth", "height", "head"],
    "climate": ["heating_degree_hours", "cooling_degree_hours", "winter_irradiance_mean", "winter_irradiance_cos1", "winter_irradiance_cos2",
                "summer_irradiance_mean", "summer_irradiance_cos1", "summer_irradiance_cos2", "direct_share_winter", "direct_share_summer", "internal_gains_winter", "internal_gains_summer",
                "utilization_exponent_heating", "utilization_exponent_cooling"],
    "envelope": ["r_si", "r_se", "r_masonry", "conductivity_insulation", "u_window", "h_other", "g_value"],
    "daylight": ["transmittance", "sky_angle", "sky_angle_loss", "overhang_half_depth", "surface_area", "reflectance", "reference_factor", "max_autonomy", "orientation_gain"],
    "systems": ["cop", "eer"],
    "lighting": ["power_density", "hours"],
    "prices": ["fixed", "wall", "window", "insulation_per_cm", "overhang"],
    "economics": ["tariff", "tariff_hi", "service_life"],
    "carbon": ["wall", "window", "insulation_per_cm", "overhang"],
}
design_keys = ("orientation", "wwr", "overhang", "insulation")
axis = {"type": "number"}
factor = lambda: strict({"id": {"enum": ["wwr", "overhang", "insulation"]}, "levels": pair(), "unit": text, "title": reference("bilingual")})

schema = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "https://semio.tech/schemas/architecture/optimization-teaching.json",
    "$defs": {
        "bilingual": strict({"en": text, "de": text}),
        "quantity": strict({"value": number, "unit": text, "note": reference("bilingual")}),
        "design": strict({key: number for key in design_keys}),
        "designPartial": strict({key: number for key in design_keys[1:]}),
        "point": strict({"wwr": number, "overhang": number}),
        "range": strict({"min": number, "max": number, "unit": text, "title": reference("bilingual")}),
    },
    **strict({
        "schema": {"const": "semio.architecture.optimization/v2"},
        "languages": {"const": ["en", "de"]},
        "title": reference("bilingual"),
        "note": reference("bilingual"),
        "parameters": strict({name: strict({key: reference("quantity") for key in keys}) for name, keys in sections.items()}),
        "ranges": strict({key: reference("range") for key in design_keys}),
        "sun": strict({"latitude": number, "declination": number, "minimum_altitude": number, "hours": {"type": "integer", "minimum": 0}, "winter_altitude": number, "summer_altitude": number, "low_summer_altitude": number,
                       "winter_azimuth": pair(), "summer_azimuth": pair(), "note": reference("bilingual")}),
        "baseline": reference("design"),
        "orientation": strict({"note": reference("bilingual"), "range": pair({"type": "integer"}), "design": reference("designPartial"), "constraint": pair({"type": "integer"}),
                               "reference": {"type": "integer"}, "start": {"type": "integer"}, "neighbour_step": {"type": "integer", "minimum": 1}}),
        "surface": strict({"note": reference("bilingual"), "design": reference("design"), "wwr": pair(), "overhang": pair()}),
        "sampling": strict({
            "levels": strict({"wwr": array(number, 2), "overhang": array(number, 2), "orientation": array(number, 2), "insulation": array(number, 2)}),
            "random": array(pair(fraction)),
            "latin_permutation": {"type": "array", "minItems": 2, "uniqueItems": True, "items": {"type": "integer", "minimum": 0}},
        }),
        "factorial": strict({"note": reference("bilingual"), "design": reference("design"), "response": {"enum": ["heating", "cooling", "lighting", "electricity"]},
                             "factors": array(factor(), 2, 2), "third": factor()}),
        "weights": strict({name: strict({key: {"type": "number", "minimum": 0} for key in ("electricity", "cost", "daylight", "carbon")}) for name in ("balanced", "cost_heavy")}),
        "criteria": array(strict({"id": text, "key": text, "direction": {"enum": ["min", "max"]}, "unit": text, "title": reference("bilingual")}), 2),
        "variants": array(strict({"id": text, "extra": {"type": "boolean"}, "title": reference("bilingual"), "design": reference("design")}, ["id", "title", "design"]), 2),
        "pareto": strict({"note": reference("bilingual"), "design": reference("design"), "wwr": array(number, 2), "insulation": array(number, 2), "overhang": array(number, 2),
                          "weights": array(fraction), "dominated_example": strict({"winner": text, "loser": text}), "step": strict({"from": text, "to": text}),
                          "refinement": strict({"member": text})}),
        "algorithms": strict({
            "budget": {"type": "integer", "minimum": 1},
            "start": reference("point"),
            "gradient": strict({"difference": positive, "learning_rate": positive, "iterations": {"type": "integer", "minimum": 1}}),
            "pattern": strict({"step": reference("point"), "contraction": {"type": "number", "exclusiveMinimum": 0, "exclusiveMaximum": 1}, "tolerance": {"type": "number", "minimum": 0},
                               "iterations": {"type": "integer", "minimum": 1}}),
            "genetic": strict({"population": {"type": "integer", "minimum": 2}, "generations": {"type": "integer", "minimum": 1}, "elite": {"type": "integer", "minimum": 0},
                               "tournament": {"type": "integer", "minimum": 2},
                               "mutation": strict({"probability": fraction, "wwr": positive, "overhang": positive}),
                               "lcg": strict({"seed": {"type": "integer", "minimum": 1}, "multiplier": {"type": "integer", "minimum": 1}, "increment": {"type": "integer", "minimum": 0},
                                              "modulus": {"type": "integer", "minimum": 2}, "note": reference("bilingual")})}),
        }),
    }),
}
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
