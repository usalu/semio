#!/usr/bin/env python3
"""T1.3 GuestSlim step: wires infinite_canvas::host_asset::register_asset_reader into each
standalone plugin manifest's `setup:` function (semio_plugin! macro hook), and adds the direct
infinite_canvas dependency needed to name it. Ticket-scoped, throwaway."""
ROOT = "/Users/ueli/Documents/semio/"

CANVAS_DEP_LINE = 'infinite_canvas = { path = "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🖼️canvas/⚡️implementation/🦀️rust", package = "semio-framework-os-kernel-infinite-canvas", default-features = false }\n'

TARGETS = {
    "✏️s/🔌️plugin/🌊️flow/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_flow_exports",
    "✏️s/🔌️plugin/🕸️dag/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_dag_exports",
    "✏️s/🔌️plugin/🎬️sequence/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_sequence_exports",
    "✏️s/🔌️plugin/🔱️trinity/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_trinity_exports",
    "✏️s/🔌️plugin/💡️reasoning/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_reasoning_mindmap_exports",
    "✏️s/🔌️plugin/📏️layout/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_layout_exports",
    "✏️s/🔌️plugin/🧩️puzzle/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_puzzle_exports",
    "✏️s/🔌️plugin/🌀️procedural/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust": "register_procedural_exports",
}

REG_CALL = '    infinite_canvas::host_asset::register_asset_reader(semio_framework_plugin::host_read_asset);\n'
REG_COMMENT = (
    '    // \U0001fac1️ GUESTSLIM: wires infinite_canvas\'s host-fetched typst font path (this crate builds\n'
    '    // with `render` off) to the component `read-asset` import.\n'
)

for rel, setup_fn in TARGETS.items():
    cargo_path = ROOT + rel + "/Cargo.toml"
    lib_path = ROOT + rel + "/📦️lib.rs"

    with open(cargo_path, "r", encoding="utf-8") as f:
        cargo = f.read()
    if "infinite_canvas" not in cargo:
        cargo = cargo.replace("[dependencies]\n", "[dependencies]\n" + CANVAS_DEP_LINE, 1)
        with open(cargo_path, "w", encoding="utf-8") as f:
            f.write(cargo)
        print(f"  + infinite_canvas dep: {rel}")
    else:
        print(f"  SKIP (already has infinite_canvas): {rel}/Cargo.toml")

    with open(lib_path, "r", encoding="utf-8") as f:
        lib = f.read()
    marker = f"fn {setup_fn}() {{\n"
    if marker not in lib:
        print(f"  WARNING: setup fn signature not found verbatim in {rel}/📦️lib.rs (skipped, check manually)")
        continue
    if "register_asset_reader" in lib:
        print(f"  SKIP (already wired): {rel}/📦️lib.rs")
        continue
    lib = lib.replace(marker, marker + REG_COMMENT + REG_CALL, 1)
    with open(lib_path, "w", encoding="utf-8") as f:
        f.write(lib)
    print(f"  + registration call in {setup_fn}(): {rel}")
