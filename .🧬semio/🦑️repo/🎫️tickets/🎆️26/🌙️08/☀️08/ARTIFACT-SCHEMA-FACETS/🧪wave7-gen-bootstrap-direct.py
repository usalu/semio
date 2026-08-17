#!/usr/bin/env python3
import re
import subprocess

PLUGIN_CRATE = {
    "🎞️animate": "semio_s_plugin_animate",
    "🏛️architect": "semio_s_plugin_architect",
    "🧱️block": "semio_s_plugin_block",
    "📐️cad": "semio_s_plugin_cad",
    "🕸️dag": "semio_s_plugin_dag",
    "🎪️demonstrator": "semio_s_plugin_demonstrator",
    "🖍️draw": "semio_s_plugin_draw",
    "🔋️energy": "semio_s_plugin_energy",
    "🏗️fem": "semio_s_plugin_fem",
    "🌊️flow": "semio_s_plugin_flow",
    "📋️forms": "semio_s_plugin_forms",
    "🌍️gis": "semio_s_plugin_gis",
    "📜️imperative": "semio_s_plugin_imperative",
    "📏️layout": "semio_s_plugin_layout",
    "💠️lowpoly": "semio_s_plugin_lowpoly",
    "➗️mathematical": "semio_s_plugin_mathematical",
    "📕️norm": "semio_s_plugin_norm",
    "🗒️note": "semio_s_plugin_note",
    "📖️playbook": "semio_s_plugin_playbook",
    "🌀️procedural": "semio_s_plugin_procedural",
    "🏭️process": "semio_s_plugin_process",
    "🧩️puzzle": "semio_s_plugin_puzzle",
    "🖨️raster": "semio_s_plugin_raster",
    "💡️reasoning": "semio_s_plugin_reasoning_mindmap",
    "📸️remodel": "semio_s_plugin_remodel",
    "🎬️sequence": "semio_s_plugin_sequence",
    "🎥️shooting": "semio_s_plugin_shooting",
    "🪵️sourcing": "semio_s_plugin_sourcing",
    "🪐️space": "semio_s_plugin_space",
    "🔱️trinity": "semio_s_plugin_trinity",
    "🌿️vcs": "semio_s_plugin_vcs",
    "✒️writer": "semio_s_plugin_writer",
}

MOD_OVERRIDES = {"energy_model": "model"}

lines = subprocess.check_output(
    [
        "rg",
        "pub fn [a-z0-9_]*_artifact_schema_descriptor",
        "/Users/ueli/Documents/semio/✏️s/🔌️plugins",
        "--glob",
        "**/🧬️schema/🦀️component.rs",
    ],
    text=True,
).strip().splitlines()

calls = []
for line in lines:
    path, rest = line.split(":pub fn ", 1)
    fn = rest.split("(", 1)[0]
    m = re.search(r"/🔌️plugins/([^/]+)/", path)
    plugin_dir = m.group(1)
    crate = PLUGIN_CRATE[plugin_dir]
    stem = fn[: -len("_artifact_schema_descriptor")]
    mod = MOD_OVERRIDES.get(stem, stem)
    calls.append(
        f"    register_artifact_schema_descriptor({crate}::artifacts::{mod}::schema::{fn}());"
    )

print("fn register_all_plugin_artifact_schema_descriptors() {")
for c in sorted(calls):
    print(c)
print("}")
