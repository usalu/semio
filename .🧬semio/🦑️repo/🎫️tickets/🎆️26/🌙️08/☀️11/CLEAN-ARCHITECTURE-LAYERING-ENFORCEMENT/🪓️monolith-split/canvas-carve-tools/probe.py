#!/usr/bin/env python3
# 🔬️ Runs the analysis-only rustc oracle (`--emit=metadata -o /dev/null`) per crate root and aggregates error messages per configuration.
import collections
import os
import re
import subprocess
import sys

SCRATCH = os.environ["SCRATCH"]
REPO_MODULES = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules"
MODULE = SCRATCH + "/split/🧰️framework/🔨️modules/🖼️canvas"
BASELINE = SCRATCH + "/baseline/🧰️framework/🔨️modules/🖼️canvas"
TOOLCHAIN = "+nightly-2026-07-20"
CONFIGS = {
    "native": [],
    "native-test": ["--test"],
    "wasm": ["--target", "wasm32-unknown-unknown"],
    "wasm-test": ["--target", "wasm32-unknown-unknown", "--test"],
    "wasip2": ["--target", "wasm32-wasip2"],
    "wasip2-test": ["--target", "wasm32-wasip2", "--test"],
}
CRATES = {
    "scene": ("semio_framework_canvas_scene", MODULE + "/🎬️scene/🦀️.rs"),
    "camera": ("semio_framework_canvas_camera", MODULE + "/📷️camera/🦀️.rs"),
    "icons": ("semio_framework_canvas_icons", MODULE + "/🪪️icon-codec/🦀️.rs"),
    "assets": ("semio_framework_canvas_assets", MODULE + "/🔌️runtime-assets/🦀️.rs"),
    "facade": ("semio_framework_canvas", MODULE + "/🦀️.rs"),
}
BASE = ("semio_framework_canvas", BASELINE + "/📦️packages/🦀️rust/🦀️.rs")


def patch(tree):
    for directory, _, files in os.walk(tree):
        if "node_modules" in directory or "/dist" in directory:
            continue
        for file in files:
            if file.endswith(".rs"):
                path = os.path.join(directory, file)
                text = open(path, encoding="utf8").read()
                fixed = re.sub(r"(?m)^extern crate [^;]*;$", "const _: () = ();", text)
                if fixed != text:
                    open(path, "w", encoding="utf8").write(fixed)


def sync_split():
    target = SCRATCH + "/split/🧰️framework/🔨️modules"
    subprocess.run(["rm", "-rf", SCRATCH + "/split"], check=True)
    os.makedirs(target, exist_ok=True)
    subprocess.run(["rsync", "-a", "--exclude", "node_modules", "--exclude", "dist", REPO_MODULES + "/🖼️canvas", target + "/"], check=True)
    os.makedirs(target + "/🖼️assets/🌱️metabolism", exist_ok=True)
    subprocess.run(["rsync", "-a", REPO_MODULES + "/🖼️assets/🔣️icons", target + "/🖼️assets/"], check=True)
    subprocess.run(["rsync", "-a", REPO_MODULES + "/🖼️assets/🌱️metabolism/🔣️icons", target + "/🖼️assets/🌱️metabolism/"], check=True)
    os.makedirs(target + "/⏱️trace/🧮️memory/🧪️testing", exist_ok=True)
    subprocess.run(["rsync", "-a", REPO_MODULES + "/⏱️trace/🧮️memory/🧪️testing/📥️requests", target + "/⏱️trace/🧮️memory/🧪️testing/"], check=True)
    shared = target + "/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"
    text = open(shared, encoding="utf8").read()
    open(shared, "w", encoding="utf8").write(text.replace("pub(in crate::m_geometry)", "pub(crate)"))
    patch(target)


def run(name, root, flags, extra=()):
    command = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", name, "--emit=metadata", "-o", "/dev/null", *flags, *extra, root]
    done = subprocess.run(command, capture_output=True, text=True)
    return done.stderr


def summarize(stderr, strip):
    rows = collections.Counter()
    blocks = re.split(r"\n(?=error)", stderr)
    for block in blocks:
        lines = block.split("\n")
        if not lines or not lines[0].startswith("error"):
            continue
        head = lines[0]
        if head.startswith("error: aborting") or head.startswith("error: could not compile"):
            continue
        location = next((line for line in lines[1:] if "-->" in line), "")
        path = location.split("-->")[-1].strip()
        path = re.sub(r":\d+:\d+$", "", path)
        for prefix in strip:
            path = path.replace(prefix, "")
        rows[(head, path)] += 1
    return rows


if __name__ == "__main__":
    which = sys.argv[1]
    configs = sys.argv[2].split(",") if len(sys.argv) > 2 else list(CONFIGS)
    out = SCRATCH + "/probe"
    os.makedirs(out, exist_ok=True)
    if which == "baseline":
        patch(SCRATCH + "/baseline")
    else:
        sync_split()
    for config in configs:
        flags = CONFIGS[config]
        if which == "baseline":
            stderr = run(BASE[0], BASE[1], flags)
            open(f"{out}/baseline-{config}.err", "w", encoding="utf8").write(stderr)
        else:
            for key, (name, root) in CRATES.items():
                if which != "all" and which != key:
                    continue
                stderr = run(name, root, flags)
                open(f"{out}/{key}-{config}.err", "w", encoding="utf8").write(stderr)
