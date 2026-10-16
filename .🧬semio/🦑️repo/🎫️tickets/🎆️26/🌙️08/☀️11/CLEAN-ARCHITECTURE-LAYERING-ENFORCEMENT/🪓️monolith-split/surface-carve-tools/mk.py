import os, sys, json
R = "/Users/ueli/Documents/semio"
FW = R + "/🧰️framework"
S = FW + "/🔨️modules/🗺️surface"
M = FW + "/🔨️modules"
# crate table: key -> (name, id, node dir, package-dir parent node)
CR = {
 "terrain":   ("semio-framework-surface-terrain",   "surface-terrain",   S + "/🏔️terrain"),
 "paint":     ("semio-framework-surface-paint",     "surface-paint",     S + "/🎨️paint"),
 "node-graph":("semio-framework-surface-node-graph","surface-node-graph",S + "/🕸️node-graph"),
 "mvt":       ("semio-framework-surface-tiled-map-mvt","surface-tiled-map-mvt", S + "/🗺️tiled-map/🧱️vector-tiles/📦️mvt-decoder"),
 "model":     ("semio-framework-surface-tiled-map-model","surface-tiled-map-model", S + "/🗺️tiled-map/🧮️model"),
 "gis-policy":("semio-framework-surface-tiled-map-gis-policy","surface-tiled-map-gis-policy", S + "/🗺️tiled-map/🌍️gis-policy"),
 "host":      ("semio-framework-surface-tiled-map","surface-tiled-map", S + "/🗺️tiled-map"),
}
def pkg(key): return CR[key][2] + "/📦️packages/🦀️rust"
def rel(frm, to): return os.path.relpath(to, frm)
# external first-party dependency specs: dep key -> (target pkg dir, extra)
EXT = {
 "semio-framework-pack": (M + "/🎒️pack/📦️packages/🦀️rust", ""),
 "semio-framework-io-base64": (M + "/🚪️io/🔤️base64/📦️packages/🦀️rust", ""),
 "semio-framework-pack-json": (M + "/🎒️pack/🔤️json/📦️packages/🦀️rust", ""),
 "semio-framework-value": (M + "/🌱️value/📦️packages/🦀️rust", ""),
 "semio-framework-pack-error": (M + "/🎒️pack/⚠️error/📦️packages/🦀️rust", ""),
 "semio-framework-replication": (M + "/📡️replication/📦️packages/🦀️rust", ""),
 "semio-framework-ui-viewport": (M + "/🖱️ui/🪟️viewport/📦️packages/🦀️rust", ""),
 "ui_styling": (M + "/🖱️ui/🎨️styling/📦️packages/🦀️rust", ', package = "semio-framework-ui-styling"'),
 "canvas": (M + "/🖼️canvas/📦️packages/🦀️rust", ', package = "semio-framework-canvas"'),
 "semio-framework-pixels": (M + "/🔲️pixels/📦️packages/🦀️rust", ', package = "semio-framework-pixels"'),
 "semio-framework-trace": (M + "/⏱️trace/📦️packages/🦀️rust", ""),
}
WS = {"semio-framework-async": "{ workspace = true }", "semio-framework-job": "{ workspace = true }"}
INTERNAL = {"mvt": "tiled_map_mvt", "model": "tiled_map_model", "gis-policy": "tiled_map_gis_policy"}
def dep_line(frm_key, dep):
    f = pkg(frm_key)
    if dep in EXT:
        p, extra = EXT[dep]
        return f'{dep} = {{ path = "{rel(f, p)}"{extra} }}'
    if dep in WS: return f'{dep} = {WS[dep]}'
    if dep == "serde": return 'serde = { version = "1.0.219", features = ["derive"] }'
    if dep == "serde_json": return 'serde_json = "1.0.140"'
    if dep == "image": return 'image = { version = "0.25", default-features = false, features = ["png"] }'
    if dep in CR:
        alias = INTERNAL[dep]
        return f'{alias} = {{ path = "{rel(f, pkg(dep))}", package = "{CR[dep][0]}" }}'
    raise KeyError(dep)
WASM_TARGET = "[target.'cfg(all(target_arch = \"wasm32\", not(target_env = \"p2\")))'.dependencies]"
WASM_LINES = ['js-sys = "0.3.83"', 'wasm-bindgen = "0.2.106"', 'web-sys = { version = "0.3.98", features = ["HtmlCanvasElement", "Window"] }']
def manifest(key, desc, deps, devdeps=(), wasm=False, wasm_extra=(), tests=(), features=None):
    name, ident, node = CR[key]
    depth = len(os.path.relpath(pkg(key), FW).split("/"))
    ws = "/".join([".."] * depth)
    out = [f'[package]', f'workspace = "{ws}"', f'name = "{name}"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true', f'description = "{desc}"', '',
           '[package.metadata.semio]', 'role = "framework"', f'id = "{ident}"', '',
           '[package.metadata.semio.rust-source-boundary]', 'maxSourceBytes = 262144', 'maxSourceFiles = 32', 'maxActiveLines = 4000', '',
           '[lints]', 'workspace = true', '', '[lib]', 'path = "../../🦀️.rs"', '']
    if features: out += ['[features]'] + features + ['']
    out += ['[dependencies]'] + [dep_line(key, d) for d in deps] + ['']
    if wasm:
        out += [WASM_TARGET] + WASM_LINES + list(wasm_extra) + ['']
    for t in tests:
        out += ['[[test]]'] + t + ['']
    out += ['[dev-dependencies]'] + [dep_line(key, d) for d in devdeps] + ['']
    return "\n".join(out)
def project_json(ident):
    return json.dumps({"name": f"@semio-tech/framework-{ident}-rs", "tags": ["scope:framework", "language:rust"]}, ensure_ascii=False, indent=2) + "\n"
def write(key, mf):
    os.makedirs(pkg(key), exist_ok=True)
    p = pkg(key) + "/Cargo.toml"
    open(p, "w", encoding="utf-8").write(mf)
    open(pkg(key) + "/📋️project.json", "w", encoding="utf-8").write(project_json(CR[key][1]))
    print("wrote", p, len(os.path.relpath(p, R).encode()))
