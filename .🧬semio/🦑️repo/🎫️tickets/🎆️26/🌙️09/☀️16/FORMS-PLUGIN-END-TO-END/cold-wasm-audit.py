from pathlib import Path
import json
from jsonschema import Draft202012Validator

root = Path.cwd()
base = root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🕸️wasm-outputs"
fixture = json.loads((base / "🧪️tests/🔣️.json").read_text())
schema = json.loads((base / "🧬️schema/🔣️.json").read_text())
validator = Draft202012Validator({"$defs": schema["$defs"], "$ref": "#/$defs/Producer"})
def runtime(value):
    if isinstance(value, str):
        return value
    if isinstance(value, dict):
        for key, child in value.items():
            if key in ["import", "node", "default"]:
                result = runtime(child)
                if result:
                    return result
def contract(owner, row):
    package = "/".join(row["specifier"].split("/")[:2])
    key = "." + row["specifier"][len(package):]
    output = root / row["path"]
    own = root / owner["root"] / "package.json"
    directory = own.parent if own.exists() and json.loads(own.read_text()).get("name") == package else output.parent
    while directory.is_relative_to(root):
        manifest = directory / "package.json"
        if manifest.exists():
            data = json.loads(manifest.read_text())
            if data.get("name") == package:
                exports = data["exports"]
                selected = exports.get(key, exports if key == "." else None) if isinstance(exports, dict) else exports
                path = runtime(selected)
                assert path and (directory / path).resolve() == output.resolve(), row["specifier"]
                if "declarations" in row:
                    typed = selected.get("types") if isinstance(selected, dict) else None
                    typed = typed or str(Path(path).with_suffix(".d.ts"))
                    assert (directory / typed).resolve() == (root / row["declarations"]).resolve(), row["specifier"]
                return
        if directory == root:
            break
        directory = directory.parent
    raise AssertionError("No exact package export: " + row["specifier"])
count = 0
for owner in fixture["inventory"]:
    project = json.loads((root / owner["manifest"]).read_text())
    target = project["targets"]["wasm"]
    producer = target["metadata"]["semio"]["wasmPackageSourceV1"]
    validator.validate(producer)
    assert project["name"] == owner["name"]
    assert "{projectRoot}/.🧾️store/wasm.json" in target["outputs"]
    assert (root / producer["sourceOwner"] / "Cargo.toml").is_file()
    for row in producer["compileOutputs"]:
        contract(owner, row)
        assert len(str(root / row["path"])) <= 256
        count += 1
assert count == fixture["moduleCount"]
print(f"PASS: {len(fixture['inventory'])} canonical owners, {count} exact public module exports; jsonschema draft2020-12; no generated runtime bytes read")
