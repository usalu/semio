import importlib.util, json, os, sys
import jsonschema

ROOT = "/Users/ueli/Documents/semio"
MUT = os.path.join(ROOT, "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
GEN = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/🐍️generate-mutation-leaves.py")
os.chdir(ROOT)
spec = importlib.util.spec_from_file_location("gen", GEN)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
mine = [k for k in m.KINDS if 500 <= k.number <= 699]
print("jsonschema", jsonschema.__version__, "| G3 kinds", len(mine))
bad = 0
aggregate = json.load(open(os.path.join(MUT, "🔣️.json"), encoding="utf-8"))
for k in mine:
    doc = json.load(open(os.path.join(MUT, k.dir, "🧬️.schema.json"), encoding="utf-8"))
    try:
        jsonschema.Draft7Validator.check_schema(doc)
    except Exception as error:
        bad += 1
        print("INVALID SCHEMA", k.slug, error)
    if k.variant not in aggregate.get("$defs", {}):
        bad += 1
        print("MISSING FROM AGGREGATE", k.variant)
populated = 0
for k in mine:
    for emoji, name, _d, _s in k.cases:
        payload_path = os.path.join(MUT, k.dir, "🧪️tests", f"{emoji}{name}", "🦠️mutation/🔣️.json")
        payload = json.load(open(payload_path, encoding="utf-8"))
        if payload == {}:
            continue
        populated += 1
        body = {key: value for key, value in payload.items() if key != "mutation"}
        schema = json.load(open(os.path.join(MUT, k.dir, "🧬️.schema.json"), encoding="utf-8"))
        errors = sorted(jsonschema.Draft7Validator(schema).iter_errors(body), key=str)
        for error in errors:
            bad += 1
            print("PAYLOAD FAILS SCHEMA", k.slug, name, error.message)
print(f"result: {len(mine)} schemas meta-validated, {populated} materialized fixture payloads validated, {bad} problems")
