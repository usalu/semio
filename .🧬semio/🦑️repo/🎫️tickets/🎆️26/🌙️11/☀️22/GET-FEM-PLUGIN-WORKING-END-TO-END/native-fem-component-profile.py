import json, sys
from pathlib import Path

root = Path(sys.argv[1])
profile = root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile"
schema = json.loads((profile / "🧬️schema/🔣️.json").read_text(encoding="utf-8"))
constraints = json.loads((profile / "📋️constraints/🔣️.json").read_text(encoding="utf-8"))

def find(node, key):
    if isinstance(node, dict):
        if key in node:
            return node[key]
        for value in node.values():
            found = find(value, key)
            if found is not None:
                return found
    if isinstance(node, list):
        for value in node:
            found = find(value, key)
            if found is not None:
                return found
    return None

target, host = find(schema, "prototype-component"), find(schema, "prototype-host")
group = constraints["groups"][constraints["owners"].get("🌎️hub/Cargo.toml", "repository")]
level = lambda value: '"s"' if value == "s" else str(int(value))
lines = ["[profile.prototype]", 'inherits = "dev"', f"opt-level = {level(target['optLevel'])}", f"debug = {target['debug']}",
         f"codegen-units = {target['codegenUnits']}", f"incremental = {str(target['incremental']).lower()}", f'lto = "{target["lto"]}"',
         "debug-assertions = true", "overflow-checks = true", 'strip = "symbols"', 'trim-paths = "object"', "",
         "[profile.prototype.build-override]", f"opt-level = {int(host['optLevel'])}", f"debug = {host['debug']}",
         f"codegen-units = {host['codegenUnits']}", f"incremental = {str(host['incremental']).lower()}"]
for row in sorted(group["native"] + group["wasm"], key=lambda row: row["package"]):
    lines += ["", f'[profile.prototype.package."{row["package"]}"]', f"opt-level = {int(row['optLevel'])}"]
Path(sys.argv[2]).write_text("\n".join(lines) + "\n", encoding="utf-8")
print(len(group["native"] + group["wasm"]), "package overrides")
