"""📐️ S2-NORM WP-6: completes norm snapshot schemas whose collections were typed `{type: object}` or restated their record
inline although the Rust record, the GraphQL type and the protobuf message already exist. Each collection's `items` becomes a `$ref` to its record, and a
record the schema lacks is added as a closed `$defs` entry whose members follow the Rust struct field order (the wire
order) and Rust types (`String` → string, `f64` → number). The wire is unchanged; the schema stops admitting anything.

  python3 🧪️s2-norm-schema-records.py [--check]
"""

import json
import sys

ROOT = "/Users/ueli/Documents/semio"
SNAPSHOT = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/{}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json"
#: 🧾️ Formats a record's sibling twins implement, as the artifact's existing records declare them.
FORMATS = ["🔣️jsonschema", "🔗️graphql", "🛰️protobuf"]
PLAN = {
    "🌍️en1997": {
        "collections": {"layers": "SoilLayer", "footings": "SpreadFoundation", "piles": "Pile", "retainingWalls": "RetainingWall", "slopes": "Slope", "upliftCases": "UpliftCase"},
        "records": {
            "RetainingWall": [("id", "string"), ("height", "number"), ("embedment", "number"), ("baseWidth", "number"), ("stemThickness", "number"), ("backfillPhiDeg", "number"), ("backfillGamma", "number"), ("wallFrictionDeg", "number"), ("earthPressureMode", "string"), ("wallMovement", "string"), ("ocr", "number"), ("concreteGamma", "number"), ("surcharge", "number"), ("verticalPermanent", "number"), ("horizontalPermanent", "number")],
            "Slope": [("id", "string"), ("angleDeg", "number"), ("height", "number"), ("length", "number"), ("governingLayerId", "string")],
            "UpliftCase": [("id", "string"), ("permanentStabilizing", "number"), ("permanentDestabilizing", "number"), ("variableDestabilizing", "number"), ("porePressure", "number"), ("totalStress", "number")],
        },
    },
    "🪶️en1999": {
        "collections": {"materials": "AluminiumMaterial", "sections": "AluminiumSection", "members": "AluminiumMember", "connections": "AluminiumConnection", "fireScenarios": "FireScenario", "fatigueDetails": "FatigueDetail", "coldFormed": "ColdFormedSheet", "shells": "AluminiumShell"},
        "records": {},
    },
}


def complete(schema, plan):
    defs = schema.setdefault("$defs", {})
    for name, fields in plan["records"].items():
        if name not in defs:
            defs[name] = {"title": name, "type": "object", "additionalProperties": False, "required": [field for field, _ in fields], "properties": {field: {"type": kind} for field, kind in fields}, "x-semio-formats": FORMATS}
    for collection, record in plan["collections"].items():
        assert record in defs, record
        schema["properties"][collection]["items"] = {"$ref": f"#/$defs/{record}"}
    return schema


def main(check):
    pending = 0
    for artifact, plan in PLAN.items():
        path = f"{ROOT}/{SNAPSHOT.format(artifact)}"
        text = open(path, encoding="utf-8").read()
        updated = json.dumps(complete(json.loads(text), plan), ensure_ascii=False, indent=2) + "\n"
        if updated != text:
            pending += 1
            if not check:
                open(path, "w", encoding="utf-8").write(updated)
            print(f"{'stale' if check else 'wrote'} {artifact}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main("--check" in sys.argv))
