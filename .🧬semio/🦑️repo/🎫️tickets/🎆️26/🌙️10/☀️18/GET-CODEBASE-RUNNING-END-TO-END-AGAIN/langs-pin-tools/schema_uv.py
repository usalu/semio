import json
F = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains"
path = F + "/🧬️schema/🔣️.json"
schema = json.load(open(path, encoding="utf8"))
defs = schema["definitions"]
old = defs["Tool"]["properties"]["release"]["pattern"]
assert "v[0-9]+" in old
defs["Tool"]["properties"]["release"]["pattern"] = old.replace("/v[0-9]+", "/v?[0-9]+")
python = defs["Python"]
assert "tool" not in python["properties"]
properties = {}
for key, value in python["properties"].items():
    properties[key] = value
    if key == "uv": properties["tool"] = {"$ref": "#/definitions/Tool", "description": "The checksum-pinned uv archives; the version equals `uv`."}
python["properties"] = properties
python["required"] = [name for name in python["required"]]
python["required"].insert(python["required"].index("uv") + 1, "tool")
open(path, "w", encoding="utf8").write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
