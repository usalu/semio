import json
import sys
from pathlib import Path

S = Path("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema")


def edit_ts():
    p = S / "🟦️.ts"
    t = open(p, encoding="utf-8", newline="").read()
    nl = "\r\n"
    assert " * - `fixed`: a prescribed surface temperature (only for reference fixtures). */" in t
    t = t.replace(
        " * - `fixed`: a prescribed surface temperature (only for reference fixtures). */",
        " * - `hydronic`: a surface heated by the water of a heating circuit: the inside temperature is the circuit water at the region's position along the flow, from the supply temperature falling to the return temperature, at the circuit's operating level; heat flows from it through the layers and the water-side film.\r\n"
        " * - `coupled`: a surface whose inside is the water of another region of the asset (a jug or pot wall): below the fill level the wall follows that region's temperature through the layers and the water-side film, and the heat it draws is taken from that region.\r\n"
        " * - `fixed`: a prescribed surface temperature (only for reference fixtures). */",
    )
    t = t.replace(
        'export const THERMAL_MODELS = ["body", "leaf", "passive", "operating", "conditioned", "fixed"] as const;',
        'export const THERMAL_MODELS = ["body", "leaf", "passive", "operating", "conditioned", "hydronic", "coupled", "fixed"] as const;',
    )
    anchor = "/** 🔥️ A thermal region: one surface patch"
    assert anchor in t
    new_types = (
        "/** 💧️ One row of a heating-circuit table: the supply and return water temperatures at one operating level `v`. */" + nl
        + "export type CircuitRow = { readonly v: number; readonly supplyTempC: number; readonly returnTempC: number };" + nl + nl
        + "/** 🚰️ The inside of a `hydronic` region: the water of the circuit driven by the operating level `key`. `rows` (ascending in `v`, at least one) give the supply and return temperatures at named levels and are interpolated linearly in between;"
        + " below the first row the water falls linearly to the air temperature at level 0, above the last row it holds. `flowPosition` (0 at the inlet, 1 at the outlet) places the region along the flow:"
        + " its water is `supply − (supply − return) · flowPosition`. `filmM2KW` is the resistance between that water and the layers (water-side film, fins), referred to the region's outer area. */" + nl
        + "export type HydronicCircuit = { readonly key: Slug; readonly rows: readonly CircuitRow[]; readonly flowPosition: number; readonly filmM2KW: number };" + nl + nl
        + "/** 🫗️ The inside of a `coupled` region: the water of the region `region` of the same asset. `wetFraction` is the share of the outer area behind which the water stands (fill level over wall height); only that share conducts, `filmM2KW` is the water-side resistance referred to the wetted area. */" + nl
        + "export type WaterCoupling = { readonly region: Slug; readonly wetFraction: number; readonly filmM2KW: number };" + nl + nl
    )
    t = t.replace(anchor, new_types + anchor, 1)
    old = "  readonly conditioned?: { readonly interiorTempC: number; readonly interiorFilmM2KW: number };"
    assert old in t
    t = t.replace(old, old + nl + "  readonly hydronic?: HydronicCircuit;" + nl + "  readonly coupled?: WaterCoupling;", 1)
    helper_anchor = "/** 🫀️ The whole-body thermal physiology"
    assert helper_anchor in t
    helper = (
        "/** 🔑️ The operating key that drives a region: the key of its `operating` heat or of its `hydronic` circuit, else `null`. */" + nl
        + "export const operatingKeyOf = (region: ThermalRegion): Slug | null => region.operating?.key ?? region.hydronic?.key ?? null;" + nl + nl
    )
    t = t.replace(helper_anchor, helper + helper_anchor, 1)
    open(p, "w", encoding="utf-8", newline="").write(t)


def edit_json():
    p = S / "🔣️.json"
    t = open(p, encoding="utf-8", newline="").read()
    nl = "\r\n"
    old = '        "conditioned",' + nl + '        "fixed"' + nl + "      ]"
    assert old in t
    t = t.replace(old, '        "conditioned",' + nl + '        "hydronic",' + nl + '        "coupled",' + nl + '        "fixed"' + nl + "      ]", 1)
    d = json.loads(t)
    assert "Slug" in d["$defs"]
    marker = '        "fixed": {' + nl + '          "type": "object",' + nl + '          "additionalProperties": false,' + nl + '          "required": [' + nl + '            "surfaceTempC"'
    assert marker in t
    props = {
        "hydronic": {
            "type": "object",
            "additionalProperties": False,
            "required": ["key", "rows", "flowPosition", "filmM2KW"],
            "properties": {
                "key": {"$ref": "#/$defs/Slug"},
                "rows": {
                    "type": "array",
                    "minItems": 1,
                    "items": {
                        "type": "object",
                        "additionalProperties": False,
                        "required": ["v", "supplyTempC", "returnTempC"],
                        "properties": {"v": {"type": "number", "minimum": 0}, "supplyTempC": {"type": "number"}, "returnTempC": {"type": "number"}},
                    },
                },
                "flowPosition": {"type": "number", "minimum": 0, "maximum": 1},
                "filmM2KW": {"type": "number", "minimum": 0},
            },
        },
        "coupled": {
            "type": "object",
            "additionalProperties": False,
            "required": ["region", "wetFraction", "filmM2KW"],
            "properties": {
                "region": {"$ref": "#/$defs/Slug"},
                "wetFraction": {"type": "number", "exclusiveMinimum": 0, "maximum": 1},
                "filmM2KW": {"type": "number", "minimum": 0},
            },
        },
    }
    block = ""
    for name, schema in props.items():
        text = json.dumps(schema, indent=2, ensure_ascii=False)
        lines = text.split("\n")
        block += '        "' + name + '": ' + (nl + "        ").join(lines) + "," + nl
    t = t.replace(marker, block + marker, 1)
    json.loads(t)
    open(p, "w", encoding="utf-8", newline="").write(t)


edit_ts()
edit_json()
print("ok")
