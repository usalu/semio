import glob
import json
from pathlib import Path

n = Path(glob.glob("C:/git/semio/*/*/*nakagin")[0])


def sub(text, a, b, count=1):
    assert text.count(a) >= 1, a
    return text.replace(a, b) if count == 0 else text.replace(a, b, count)


j = n / "🔣️.json"
d = json.loads(j.read_text(encoding="utf-8"))
leaf = lambda v, unit, note: {"value": v, "unit": unit, "basis": "assumed", "note": note}
d["variants"] = [
    {"key": "window_upgrade",
     "name": {"en": "Window upgrade", "de": "Fensteraufwertung"},
     "description": {"en": "Triple or double low-e glazing in an insulated frame instead of the 2.33 W/(m²·K) window.", "de": "Dreifach- oder Zweifach-Low-E-Verglasung im gedämmten Rahmen statt des Fensters mit 2,33 W/(m²·K)."},
     "overrides": {"u_window": leaf(1.1, "W/(m²·K)", "teaching value for triple or double low-e glazing")}},
    {"key": "back_wall_exposed",
     "name": {"en": "Back wall treated as outside", "de": "Rückwand als Aussenwand behandelt"},
     "description": {"en": "The typical mistake: the core-side wall gets the outdoor temperature, factor b = 1.0 instead of 0.5.", "de": "Der typische Fehler: Die kernseitige Wand erhält die Aussentemperatur, Faktor b = 1,0 statt 0,5."},
     "overrides": {"attached_factor": leaf(1.0, "1", "no reduction towards the unheated core")}},
    {"key": "shared_roof_floor",
     "name": {"en": "Heated neighbours above and below", "de": "Beheizte Nachbarn oben und unten"},
     "description": {"en": "Roof and floor border heated capsules, so they lose no heat: U = 0.", "de": "Dach und Boden grenzen an beheizte Kapseln und verlieren keine Wärme: U = 0."},
     "overrides": {"u_roof": leaf(0.0, "W/(m²·K)", "no loss to a heated neighbour"), "u_floor": leaf(0.0, "W/(m²·K)", "no loss to a heated neighbour")}},
]
c = d["climate"]
c["source"] = "jma-tokyo-normals"
c["period"] = {"en": "JMA normals 1991-2020 (via the Wikipedia Tokyo climate table)", "de": "JMA-Normalwerte 1991-2020 (über die Wikipedia-Klimatabelle Tokio)"}
c["method"] = {"en": "HDD18: sum over the months of max(0, 18 °C minus monthly mean) times days", "de": "HDD18: Summe über die Monate von max(0, 18 °C minus Monatsmittel) mal Tage"}
c["heating_degree_days"] = {"value": 1585.7, "unit": "K·d", "basis": "derived", "source": "jma-tokyo-normals", "note": "monthly-mean method, equals the sum of the monthly degree days"}
j.write_text(json.dumps(d, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

s = n / "🧬️schema/🔣️.schema.json"
t = s.read_text(encoding="utf-8")
t = sub(t, '"energy", "climate"],', '"energy", "climate", "variants"],')
t = sub(t, '"required": ["station", "months"],', '"required": ["station", "source", "period", "method", "heating_degree_days", "months"],')
t = sub(t, '"station": {"$ref": "#/$defs/localized"},', '"station": {"$ref": "#/$defs/localized"},\n        "source": {"$ref": "#/$defs/id"},\n        "period": {"$ref": "#/$defs/localized"},\n        "method": {"$ref": "#/$defs/localized"},\n        "heating_degree_days": {"$ref": "#/$defs/positive"},')
variants = '''    "variants": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["key", "name", "description", "overrides"],
        "properties": {
          "key": {"type": "string", "pattern": "^[a-z]+(_[a-z]+)*$"},
          "name": {"$ref": "#/$defs/localized"},
          "description": {"$ref": "#/$defs/localized"},
          "overrides": {
            "type": "object",
            "minProperties": 1,
            "additionalProperties": false,
            "properties": {
              "u_roof": {"$ref": "#/$defs/nonnegative"},
              "u_floor": {"$ref": "#/$defs/nonnegative"},
              "u_wall": {"$ref": "#/$defs/nonnegative"},
              "u_window": {"$ref": "#/$defs/nonnegative"},
              "attached_factor": {"$ref": "#/$defs/nonnegative"},
              "air_change": {"$ref": "#/$defs/nonnegative"}
            }
          }
        }
      }
    },
    "climate": {'''
t = sub(t, '    "climate": {', variants)
t = sub(t, '    "angle":', '    "nonnegative": {"allOf": [{"$ref": "#/$defs/quantity"}, {"type": "object", "properties": {"value": {"type": "number", "minimum": 0}}}]},\n    "angle":')
s.write_text(t, encoding="utf-8")

p = n / "🐍️.py"
t = p.read_text(encoding="utf-8")
t = sub(t, '''def envelope(t) -> dict:
    """🧥 Envelope of a type: five exposed faces plus the back face attached to the core, window cut from the front."""
    t = capsule_type(t) if isinstance(t, str) else t
    e = _data()["energy"]
''', '''def variants() -> tuple:
    """🧪 Keys of the named design variants."""
    return tuple(v["key"] for v in _data()["variants"])


def variant(name) -> dict:
    """🧪 A named design variant: key, bilingual name and description, and the overriding Values by energy key."""
    raw = next((v for v in _data()["variants"] if v["key"] == name), None)
    if raw is None:
        raise KeyError(f"Unknown variant: {name}")
    return {"key": raw["key"], "name": raw["name"], "description": raw["description"],
            "overrides": {k: Value(o["value"], o["unit"], o["basis"], o.get("source")) for k, o in raw["overrides"].items()}}


def _energy(variant_names=None) -> dict:
    e = dict(_data()["energy"])
    names = (variant_names,) if isinstance(variant_names, str) else tuple(variant_names or ())
    for name in names:
        for key, o in variant(name)["overrides"].items():
            e[key] = {**e[key], "value": o.value, "unit": o.unit, "basis": o.basis}
    return e


def envelope(t, variant=None) -> dict:
    """🧥 Envelope of a type, optionally under a design variant name or tuple of names: five exposed faces plus the back face attached to the core, window cut from the front."""
    t = capsule_type(t) if isinstance(t, str) else t
    e = _energy(variant)
''')
t = sub(t, '''    h_v = e["air_heat_capacity"]["value"] * e["air_change"]["value"] * pr["volume"]
    return {"key": t.key,''', '''    h_v = e["air_heat_capacity"]["value"] * e["air_change"]["value"] * pr["volume"]
    return {"key": t.key, "variant": variant,''')
t = sub(t, '''def heating(t) -> dict:
    """🔥 Heating of one capsule: design load in W, annual demand in kWh/a and in kWh/(m²·a) of net floor area."""
    t = capsule_type(t) if isinstance(t, str) else t
    c, env, pr = climate(), envelope(t), program(t)''', '''def heating(t, variant=None) -> dict:
    """🔥 Heating of one capsule, optionally under a design variant (name or tuple of names): design load in W, annual demand in kWh/a and in kWh/(m²·a) of net floor area."""
    t = capsule_type(t) if isinstance(t, str) else t
    c, env, pr = climate(), envelope(t, variant), program(t)''')
t = sub(t, '''"annual_per_area": annual / pr["net_floor_area"], "net_floor_area"''', '''"annual_per_area": annual / pr["net_floor_area"], "variant": variant, "net_floor_area"''')
t = sub(t, '''"months": months, "hdd": sum(m["hdd"] for m in months),
            "sources": [d["sources"]["jma-tokyo-normals"]]}''', '''"months": months, "hdd": sum(m["hdd"] for m in months),
            "source": d["climate"]["source"], "period": d["climate"]["period"], "method": d["climate"]["method"],
            "hdd_value": value("climate.heating_degree_days"), "sources": [d["sources"][d["climate"]["source"]]]}''')
p.write_text(t, encoding="utf-8")
print("done")
