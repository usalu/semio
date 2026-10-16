#!/usr/bin/env python3
# 🧬️ Union oracle: the five canvas crates mounted as modules of ONE crate (crate-name paths rewritten to module paths), so cross-crate
# visibility and facade re-exports are resolved by the full-resolution oracle. Compared against the baseline error multiset by message.
import collections
import os
import re
import subprocess
import sys

SCRATCH = os.environ["SCRATCH"]
sys.path.insert(0, os.path.dirname(__file__))
import probe  # noqa: E402

probe.sync_split()
SRC = SCRATCH + "/split/🧰️framework/🔨️modules/🖼️canvas"
UNION = SCRATCH + "/union"
subprocess.run(["rm", "-rf", UNION], check=True)
subprocess.run(["rsync", "-a", SCRATCH + "/split/", UNION + "/"], check=True)
CANVAS = UNION + "/🧰️framework/🔨️modules/🖼️canvas"

OWNERS = {
    "semio_framework_canvas_assets": ["🔌️runtime-assets"],
    "semio_framework_canvas_camera": ["📷️camera", "🧪️tests/📷️camera-fit", "🧪️tests/🧭️camera-unit"],
    "semio_framework_canvas_scene": ["🎬️scene", "♻️retirement/➰️path", "♻️retirement/🖊️stroke", "🧪️tests/🔬️unit", "🧪️tests/🏷️label-fit", "🧪️tests/🏷️label-shapes", "🧪️tests/🎨️theme-color-fields", "🧪️tests/🔬️renderer-color-stroke-raster", "🧪️tests/🔬️renderer-opaque-scene-retirement", "⏱️trace"],
    "semio_framework_canvas_icons": ["🪪️icon-codec", "🌉️icon-name-value", "🧪️tests/🔬️icon-codec-unit"],
    "semio_framework_canvas": ["🧪️tests/🎬️draw-list"],
}
CRATE_NAMES = ["semio_framework_canvas_assets", "semio_framework_canvas_camera", "semio_framework_canvas_scene", "semio_framework_canvas_icons"]


def rewrite(path, module):
    text = open(path, encoding="utf8").read()
    fixed = re.sub(r"(?<![A-Za-z0-9_$:])crate::", f"crate::{module}::", text)
    for name in CRATE_NAMES:
        fixed = re.sub(r"(?<![A-Za-z0-9_:])" + name + r"::", f"crate::{name}::", fixed)
    fixed = re.sub(r"(?<![A-Za-z0-9_:])use (semio_framework_canvas_[a-z]+) as ", r"use crate::\1 as ", fixed)
    if fixed != text:
        open(path, "w", encoding="utf8").write(fixed)


def walk(directory):
    for root, _, files in os.walk(directory):
        if "node_modules" in root or "/dist" in root:
            continue
        for file in files:
            if file.endswith(".rs"):
                yield os.path.join(root, file)


for module, relatives in OWNERS.items():
    for relative in relatives:
        target = CANVAS + "/" + relative if not relative.startswith("⏱️") else UNION + "/🧰️framework/🔨️modules/" + relative
        if relative.startswith("⏱️"):
            continue
        for file in walk(target) if os.path.isdir(target) else [target]:
            rewrite(file, module)
for file in [CANVAS + "/🦀️.rs"]:
    rewrite(file, "semio_framework_canvas")
rewrite(CANVAS + "/🧪️tests/🎬️draw-list/🦀️.rs", "semio_framework_canvas")
rewrite(UNION + "/🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs", "semio_framework_canvas_scene")

roots = {
    "semio_framework_canvas_assets": CANVAS + "/🔌️runtime-assets/🦀️.rs",
    "semio_framework_canvas_camera": CANVAS + "/📷️camera/🦀️.rs",
    "semio_framework_canvas_scene": CANVAS + "/🎬️scene/🦀️.rs",
    "semio_framework_canvas_icons": CANVAS + "/🪪️icon-codec/🦀️.rs",
    "semio_framework_canvas": CANVAS + "/🦀️.rs",
}
union_root = UNION + "/union.rs"
open(union_root, "w", encoding="utf8").write("".join(f'#[path = "{path}"]\npub mod {name};\n' for name, path in roots.items()))
os.makedirs(SCRATCH + "/probe", exist_ok=True)
for config, flags in probe.CONFIGS.items():
    stderr = probe.run("union", union_root, flags)
    open(f"{SCRATCH}/probe/union-{config}.err", "w", encoding="utf8").write(stderr)

for config in probe.CONFIGS:
    base = probe.summarize(open(f"{SCRATCH}/probe/baseline-{config}.err", encoding="utf8").read(), [])
    union = probe.summarize(open(f"{SCRATCH}/probe/union-{config}.err", encoding="utf8").read(), [])
    def heads(rows):
        out = collections.Counter()
        for (head, _), count in rows.items():
            out[head] += count
        return out
    b, u = heads(base), heads(union)
    print(f"== {config}: baseline {sum(b.values())} union {sum(u.values())}")
    for message in sorted(set(b) | set(u)):
        if b[message] != u[message]:
            print(f"   {b[message]:3d} -> {u[message]:3d}  {message}")
