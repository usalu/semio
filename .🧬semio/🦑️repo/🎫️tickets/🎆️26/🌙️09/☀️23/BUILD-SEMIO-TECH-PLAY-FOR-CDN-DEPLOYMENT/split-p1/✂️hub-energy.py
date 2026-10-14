#!/usr/bin/env python3
"""✂️ Points the energy hub composition at the owning energy crates (lane P1): the package root loses its glob/shim
re-exports, the plugin root and its surface law name the editor, viewer and aggregator crates directly, and the hub
workspace and manifest declare exactly those crates.
"""
import os, re
HUB = "/Users/ueli/Documents/semio/🌎️hub"; C = f"{HUB}/🧩️compositions/🔋️energy"
def rd(p): return open(p, encoding="utf-8").read()
def wr(p, t): open(p, "w", encoding="utf-8").write(t)
pkg = rd(f"{C}/📦️packages/🦀️rust/🦀️.rs")
pkg = re.sub(r"use semio_s_artifact_energy_model::\*;\nmod editor \{\n    pub use semio_s_artifact_energy_model::editor::\*;\n\}\nmod viewer \{\n    pub use semio_s_artifact_energy_model::viewer::\*;\n\}\n", "", pkg)
wr(f"{C}/📦️packages/🦀️rust/🦀️.rs", pkg)
RULES = [(r"\bcrate::editor::model::", "semio_s_artifact_energy_model_editor::"), (r"\bcrate::viewer::model::", "semio_s_artifact_energy_model_viewer::"),
         (r"\bcrate::declaration\(\)", "semio_s_artifact_energy_model::declaration()"), (r"\bcrate::artifact_kind\(\)", "semio_s_artifact_energy_model::artifact_kind()")]
for p in (f"{C}/🦀️.rs", f"{C}/🧪️tests/🔬️surface/🦀️.rs"):
    t = rd(p)
    for a, b in RULES: t = re.sub(a, b, t)
    wr(p, t)
man = rd(f"{C}/📦️packages/🦀️rust/Cargo.toml")
man = man.replace("semio-s-artifact-energy-model = { workspace = true }\n", "semio-s-artifact-energy-model = { workspace = true }\nsemio-s-artifact-energy-model-editor = { workspace = true }\nsemio-s-artifact-energy-model-viewer = { workspace = true }\n", 1)
wr(f"{C}/📦️packages/🦀️rust/Cargo.toml", man)
ws = rd(f"{HUB}/Cargo.toml")
base = "../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
row = 'semio-s-artifact-energy-model = { path = "../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust" }\n'
if "semio-s-artifact-energy-model-editor =" not in ws:
    ws = ws.replace(row, row + f'semio-s-artifact-energy-model-editor = {{ path = "{base}/✏️editor/📦️packages/🦀️rust" }}\nsemio-s-artifact-energy-model-viewer = {{ path = "{base}/👁️viewer/📦️packages/🦀️rust" }}\n', 1)
wr(f"{HUB}/Cargo.toml", ws)
print("ok")
