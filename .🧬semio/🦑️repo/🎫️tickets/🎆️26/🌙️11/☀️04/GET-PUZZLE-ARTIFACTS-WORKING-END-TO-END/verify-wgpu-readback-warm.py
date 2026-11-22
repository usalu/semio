"""📤️ Run the original production readback laws with the existing native WGPU artifacts."""

import os
from pathlib import Path
import subprocess


ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
workspace = generated / "wgpu-native-readback"
workspace.mkdir(exist_ok=True)
dependencies = generated / "2d-recorder-warm-deps"
source = root / "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/📤️readback/🦀️.rs"
wrapper = workspace / "readback.rs"
wrapper.write_text(f'#[path = "{source.as_posix()}"]\nmod readback;\n', encoding="utf-8")
executable = workspace / "readback-tests"
command = [
    "rustc", "--edition=2021", "--test", wrapper, "-o", executable,
    "-Cdebuginfo=0", "-Zthreads=1", "-Clinker-features=+lld",
    "-Clink-self-contained=+linker", "-Zunstable-options",
    "-L", f"dependency={dependencies}",
]
for name in ("semio_ui_wgpu_device", "semio_ui_wgpu_draw_execution", "semio_framework_pixels", "semio_framework_async", "serde_json"):
    artifacts = list(dependencies.glob(f"lib{name}-*.rlib"))
    if len(artifacts) != 1:
        raise SystemExit(f"[DEBUG] Expected one warm {name} artifact, found {len(artifacts)}")
    for artifact in (artifacts[0], artifacts[0].with_suffix(".rmeta")):
        command.extend(("--extern", f"{name}={artifact}"))

environment = {name: value for name, value in os.environ.items() if name not in ("DISPLAY", "WAYLAND_DISPLAY")}
if "VK_DRIVER_FILES" not in environment:
    browser_cache = Path(environment["PLAYWRIGHT_BROWSERS_PATH"])
    drivers = sorted(browser_cache.glob("chromium_headless_shell-*/chrome-headless-shell-linux64/vk_swiftshader_icd.json"))
    if not drivers:
        raise SystemExit("[DEBUG] Chromium SwiftShader Vulkan ICD is absent")
    environment["VK_DRIVER_FILES"] = str(drivers[-1])
environment["TMPDIR"] = str(workspace)
environment.setdefault("ZIG_GLOBAL_CACHE_DIR", str(workspace / "zig-global"))
environment.setdefault("ZIG_LOCAL_CACHE_DIR", str(workspace / "zig-local"))
print(f"[DEBUG] Vulkan driver {environment['VK_DRIVER_FILES']}", flush=True)
subprocess.run(command, cwd=root, env=environment, check=True)
subprocess.run([executable, "--test-threads=1", "--nocapture"], cwd=root, env=environment, check=True)
