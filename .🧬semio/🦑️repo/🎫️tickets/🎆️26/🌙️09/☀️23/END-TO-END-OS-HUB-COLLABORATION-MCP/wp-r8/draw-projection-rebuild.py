"""Rebuilds the authored Draw source scenario and the Draw projection of 📐️cad-draw-path-projection on the current Draw
command bundle (ticket input, R8 round 3): artifact `🖍️drawing`, the nine-file `draw-editor-command-bundle-v1` contract (package
manifests point their library entry at the owner's `🦀️.rs`, no package glue, no configurable entry), and the dependency registry
consumer at `🔒️dependencies.json`."""
import hashlib, json, os

REPO = "/Users/ueli/Documents/semio"
LIBRARY = f"{REPO}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SCENARIO = f"{LIBRARY}/🧫️fixtures/🖍️draw-source-scenario/🔣️.json"
SCHEMA = f"{LIBRARY}/🧬️schema/🖍️draw-source-scenario/🔣️.json"
GOLDEN = f"{LIBRARY}/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json"
GLUE = ["🔄️fsm/✨️macros/📦️packages/🦀️rust/🦀️.rs", "🔄️fsm/📦️packages/🦀️rust/🦀️.rs"]
REGISTRY_CORRUPT, REGISTRY = "../🖍️draw-source-scenario/🔣️.json", "🔒️dependencies.json"
SCHEMA_REGISTRY_CORRUPT = "../../🖍️draw-source-scenario/🛂️schema/🔣️.json"

scenario = json.load(open(SCENARIO, encoding="utf-8"))
scenario["owner"]["artifactId"] = "🖍️drawing"
members = [m for m in scenario["members"] if m["path"] not in GLUE]
for member in members:
    if member["path"] == "🔄️fsm/✨️macros/🦀️.rs":
        member["content"] = "extern crate proc_macro;\n#[proc_macro]\npub fn fixture(input: proc_macro::TokenStream) -> proc_macro::TokenStream { input }\n"
    if member["path"].endswith("Cargo.toml"):
        member["content"] = member["content"].replace('path = "🦀️.rs"', 'path = "../../🦀️.rs"')
scenario["members"] = members
for consumer in scenario["consumers"]:
    if consumer["path"] == REGISTRY_CORRUPT:
        consumer["path"] = REGISTRY
oracle = scenario["oracle"]
oracle.pop("configuration")
oracle["destinationDirectoryCount"], oracle["destinationNodeCount"] = 7, 16
open(SCENARIO, "w", encoding="utf-8").write(json.dumps(scenario, ensure_ascii=False, indent=2) + "\n")

schema_text = open(SCHEMA, encoding="utf-8").read()
schema = json.loads(schema_text)
properties = schema["properties"]
properties["owner"]["properties"]["artifactId"]["const"] = "🖍️drawing"
properties["members"]["minItems"] = properties["members"]["maxItems"] = len(members)
properties["members"]["allOf"] = [rule for rule in properties["members"]["allOf"] if rule["contains"]["properties"]["path"]["const"] not in GLUE]
for rule in properties["consumers"]["allOf"]:
    if rule["contains"]["properties"]["path"]["const"] == SCHEMA_REGISTRY_CORRUPT:
        rule["contains"]["properties"]["path"]["const"] = REGISTRY
oracle_schema = properties["oracle"]
oracle_schema["properties"].pop("configuration")
oracle_schema["required"] = [key for key in oracle_schema["required"] if key != "configuration"]
oracle_schema["properties"]["destinationDirectoryCount"]["const"] = 7
oracle_schema["properties"]["destinationNodeCount"]["const"] = 16
open(SCHEMA, "w", encoding="utf-8").write(json.dumps(schema, ensure_ascii=False, indent=2) + "\n")

golden = json.load(open(GOLDEN, encoding="utf-8"))
golden.pop("referencePreimageHashAlgorithm")
golden.pop("referenceConsumers")
projection = next(p for p in golden["projections"] if p["contractId"] == "artifact-editor-command-bundle-v1")
projection["artifactId"] = "🖍️drawing"
projection["sourceRoot"] = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down"
projection["destinationRoot"] = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down"
paths = sorted((m["path"] for m in members), key=lambda path: f"{projection['sourceRoot']}/{path}".encode())
projection["mappings"] = [{"sourcePath": f"{projection['sourceRoot']}/{path}", "destinationPath": f"{projection['destinationRoot']}/{path}"} for path in paths]
directories = set()
for mapping in projection["mappings"]:
    path = os.path.dirname(mapping["destinationPath"])
    while True:
        directories.add(path)
        if path == projection["destinationRoot"]:
            break
        path = os.path.dirname(path)
projection["sourceFileCount"] = len(paths)
projection["destinationDirectoryCount"] = len(directories)
projection["destinationNodeCount"] = len(directories) + len(paths)
projection["maxPathBytes"] = max(len(m["destinationPath"].encode()) for m in projection["mappings"])
projection["mappingDigest"] = hashlib.sha256("\n".join(f"{m['sourcePath']}\0{m['destinationPath']}" for m in projection["mappings"]).encode()).hexdigest()
projection["fileContracts"] = 4
projection.pop("referenceEdits")
open(GOLDEN, "w", encoding="utf-8").write(json.dumps(golden, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({k: projection[k] for k in ("sourceFileCount", "destinationDirectoryCount", "destinationNodeCount", "maxPathBytes", "mappingDigest")}, ensure_ascii=False))
live = sorted(os.path.relpath(os.path.join(d, f), f"{REPO}/{projection['destinationRoot']}") for d, _, fs in os.walk(f"{REPO}/{projection['destinationRoot']}") for f in fs if "🧪️tests" not in d)
print("live destination bundle equals the projection:", live == sorted(paths), live)
