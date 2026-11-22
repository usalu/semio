"""🧩️ Link native Puzzle scene probes with warm dependency graphs."""

import argparse
import os
from pathlib import Path
import re
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("phase", choices=("producer", "renderer", "board"))
phase = parser.parse_args().phase
ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
build = generated / "cargo-puzzle-clipboard-build/debug/build"
work = generated / ("puzzle2d-board-readback" if phase == "board" else "puzzle3d-scene-handoff")
dependencies = work / ("native" if phase == "board" else phase)
dependencies.mkdir(parents=True, exist_ok=True)

anchors = {
    "board": {"semio_os_infinite_board_normal_host": "84dcd3ec09f56cb2", "semio_ui_wgpu_render": "4607e638c954bb13"},
    "producer": {"semio_s_artifact_puzzle_3d_editor": "0da5dceeed90cee5", "semio_plugin_testing_laws": "6050633217b913f0"},
    "renderer": {"semio_os_infinite_world_host": "ea57833e49524ca3", "semio_ui_wgpu_render": "4607e638c954bb13"},
}[phase]
externs = {
    "board": ("semio_os_infinite_board_normal_host",
              "semio_os_infinite_board_normal_focus", "semio_framework_canvas_scene",
              "semio_framework_value_native_owned", "semio_framework_job", "semio_framework_async",
              "serde_json", "wgpu"),
    "producer": ("semio_s_artifact_puzzle_3d_editor", "semio_plugin", "semio_os_kernel", "semio_framework_async",
                 "semio_ui_contract", "semio_ui_locale", "serde_json"),
    "renderer": ("semio_os_infinite_world_host", "semio_os_infinite_world_bridge", "semio_os_infinite_world_scene",
                 "semio_os_infinite_world_model", "semio_os_infinite_world_interaction", "semio_ui_wgpu_render", "semio_ui_scene", "semio_ui_declaration",
                 "semio_ui_theme", "semio_ui_text", "semio_ui_draw_contract", "semio_ui_wgpu_widget_kit",
                 "semio_ui_prepared", "semio_ui_wgpu_input", "semio_framework_job", "semio_framework_async",
                 "semio_framework_mesh_engine", "semio_ui_scene_mesh", "serde_json"),
}[phase]

fingerprints = {}
for name, fingerprint in anchors.items():
    metadata = build / name.replace("_", "-") / fingerprint / "out" / f"lib{name}-{fingerprint}.rmeta"
    records = subprocess.check_output(["rustc", "-Z", "ls=all", metadata], text=True)
    for crate, digest in [(name, fingerprint), *re.findall(r"^\d+ (\w+)-([0-9a-f]+) hash ", records, re.M)]:
        fingerprints.setdefault(crate, set()).add(digest)
known = {f"lib{name}-{fingerprint}" for name, hashes in fingerprints.items() for fingerprint in hashes}
native = set()
for output in build.glob("*/*/out"):
    for artifact in output.iterdir():
        if artifact.stem in known and artifact.suffix in (".rmeta", ".rlib", ".so", ".dylib", ".dll"):
            link = dependencies / artifact.name
            if not link.exists():
                link.symlink_to(artifact)
        if artifact.suffix in (".a", ".lib"):
            native.add(output)

executable = work / f"scene-{phase}"
source = ticket / ("verify-puzzle2d-board-readback.rs" if phase == "board" else f"verify-puzzle3d-scene-{phase}.rs")
command = [
    "rustc", "--edition=2021", "-Cdebuginfo=0", "-Zthreads=1",
    "-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options",
    "-L", f"dependency={dependencies}", "-o", executable, source,
]
for name in externs:
    matches = list(dependencies.glob(f"lib{name}-*.rlib"))
    if len(matches) != 1:
        raise SystemExit(f"[DEBUG] Expected one warm {name} artifact, found {len(matches)}")
    for artifact in (matches[0], matches[0].with_suffix(".rmeta")):
        command.extend(("--extern", f"{name}={artifact}"))
for output in sorted(native):
    command.extend(("-L", f"native={output}"))
environment = {key: value for key, value in os.environ.items() if key not in ("DISPLAY", "WAYLAND_DISPLAY")}
environment.update(TMPDIR=str(work), RUST_MIN_STACK="134217728")
environment.setdefault("ZIG_GLOBAL_CACHE_DIR", str(generated / "wgpu-native-readback/zig-global"))
environment.setdefault("ZIG_LOCAL_CACHE_DIR", str(generated / "wgpu-native-readback/zig-local"))
if phase != "producer" and "VK_DRIVER_FILES" not in environment:
    environment["VK_DRIVER_FILES"] = str(next(Path(environment["PLAYWRIGHT_BROWSERS_PATH"]).glob(
        "chromium_headless_shell-*/chrome-headless-shell-linux64/vk_swiftshader_icd.json")))
subprocess.run(command, cwd=root, env=environment, check=True)
subprocess.run([executable, work], cwd=root, env=environment, check=True)
