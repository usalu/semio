"""🧷️ Declares wasm-bindgen package binding roots that sources import relatively as owned generator contracts."""
import json, sys
p = sys.argv[1]
contracts = {
    "spatial-kernel-session-bindings": ("✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session", "@semio-tech/s-spatial-kernel-semio-session-rs", "The session wasm route runs wasm-bindgen into the package bindings that the browser session law imports"),
    "cad-geometry-bindings": ("✏️s/🧑‍💻dev/📐️cad", "@semio-tech/s-cad-composition-rs", "The CAD wasm route runs wasm-bindgen into the package bindings that the STEP composition imports"),
}
t = json.loads(open(p, encoding="utf-8").read())
for key, (r, project, reason) in contracts.items():
    o = r + "/📦️packages/🦀️rust"
    t["generatorContracts"][key] = {
        "ownership": "owned",
        "ownerPath": o,
        "target": project + ":wasm",
        "inputPatterns": ["✏️s/Cargo.lock", "✏️s/Cargo.toml", o + "/Cargo.toml", o + "/📋️project.json", o + "/📜️script.ts", r + "/**/*.rs"],
        "outputRoots": [{"path": o + "/🕸️bindings", "inclusion": "ignored"}],
        "reason": reason,
    }
t["generatorContracts"] = dict(sorted(t["generatorContracts"].items()))
open(p, "w", encoding="utf-8").write(json.dumps(t, ensure_ascii=False, indent=2) + "\n")
