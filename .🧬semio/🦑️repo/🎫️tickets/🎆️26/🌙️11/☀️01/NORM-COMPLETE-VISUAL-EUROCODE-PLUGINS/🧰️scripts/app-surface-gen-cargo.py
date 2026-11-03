#!/usr/bin/env python3
"""Writes the Cargo.toml of every split app-surface crate (and the facade) from one table."""
import os
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
AS = "📇️registry/🧬️contract/🖥️app-surface"
PREFIX = "semio-s-plugin-norm-app-surface"
CRATES = {
  "foundation": (f"{AS}/🧱️foundation", "Norm app surface foundation: ids, value paths, field metadata, report-tree helpers, document editors and store ownership macros", []),
  "viewport3d": (f"{AS}/🪟️visual-windows/🧊️viewport3d", "Norm app surface 3D viewport: scene lowering, camera, picking, playback and accessible descriptions", []),
  "visual-chrome": (f"{AS}/🖼️visual-chrome", "Norm app surface visual chrome: 2D diagrams, accessibility, themes, device profiles, manifest and interaction domain", ["foundation", "viewport3d"]),
  "live-core": (f"{AS}/🧲️live-core", "Norm app surface live core: lane payloads and runtime, direct manipulation, edit rules and live command dispatch", ["foundation"]),
  "panels": (f"{AS}/🧾️panels", "Norm app surface panels: checks, properties, outliner, selection mirror, catalogue, viewport toolbar and lane overlays", ["foundation", "viewport3d", "visual-chrome", "live-core"]),
  "live-editor": (f"{AS}/🎛️live-editor", "Norm app surface live editor: retained commands, tool jobs, live editor wiring and the disposition oracle", ["foundation", "viewport3d", "visual-chrome", "live-core", "panels"]),
}
FACADE = f"{AS}"
def cargo(dirpath, name, desc, lowers, facade=False):
    cdir = os.path.join(NORM, dirpath, "📦️packages/🦀️rust")
    os.makedirs(cdir, exist_ok=True)
    depth = len(os.path.relpath(NORM, cdir).split(os.sep))
    root = "../" * depth
    root = root[:-1]
    fw = "../" * (depth + 3) + "🧰️framework/🔨️modules/🎯️action-bus/📦️packages/🦀️rust"
    lines = [
      "[package]", f'workspace = "{root}"', f'name = "{name}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", f'description = "{desc}"', "",
      "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "",
    ]
    if name == PREFIX or name.endswith("live-editor"):
        lines += ["[features]", f'compliance-testing = [{", ".join(chr(34)+x+chr(34) for x in (["dep:serde", "dep:serde_json", "dep:semio-framework-artifact-reference"] if not facade else [FEATURE_FORWARD]))}]', ""]
    lines += ["[dependencies]", "pack = { workspace = true }", "semio-framework = { workspace = true }", f'semio-framework-action-bus = {{ path = "{fw}" }}']
    lines += ["semio-framework-job = { workspace = true }", "semio-framework-os-kernel = { workspace = true }", "semio-framework-pack-json = { workspace = true }", "semio-framework-plugin = { workspace = true }", "semio-framework-schema = { workspace = true }", "semio-framework-schema-registry = { workspace = true }", "semio-framework-ui-locale = { workspace = true }", "semio-framework-value = { workspace = true }", "semio-framework-value-derive = { workspace = true }", "semio-s-plugin-norm-compliance = { workspace = true }", "semio-s-plugin-norm-results-config = { workspace = true }"]
    scene = os.path.relpath(os.path.join(NORM, "🎬️structural-scene/📦️packages/🦀️rust"), cdir)
    lines += [f'semio-s-plugin-norm-structural-scene = {{ path = "{scene}" }}', "semio-framework-ui-contract = { workspace = true }"]
    if name.endswith("live-editor"):
        lines += ["semio-framework-artifact-reference = { workspace = true, optional = true }", "serde = { workspace = true, optional = true }", "serde_json = { workspace = true, optional = true }"]
    for low in lowers:
        d = os.path.relpath(os.path.join(NORM, CRATES[low][0], "📦️packages/🦀️rust"), cdir)
        lines.append(f'{PREFIX}-{low} = {{ path = "{d}" }}')
    lines += ["", "[dev-dependencies]", "semio-framework-async-macros = { workspace = true }", 'semio-framework-os-kernel = { workspace = true, features = ["protocol-laws", "mutation-testing"] }', 'semio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }', 'semio-s-plugin-norm-compliance = { workspace = true, features = ["compliance-testing"] }', "serde_json = { workspace = true }", ""]
    open(os.path.join(cdir, "Cargo.toml"), "w", encoding="utf-8").write("\n".join(lines))
FEATURE_FORWARD = f"{PREFIX}-live-editor/compliance-testing"
for k, (d, desc, lowers) in CRATES.items():
    cargo(d, f"{PREFIX}-{k}", desc, lowers)
cargo(FACADE, PREFIX, "Norm app surface facade: panels, windows, field meta, tool jobs and the retained editor, re-exported from the split app-surface crates", list(CRATES), facade=True)
