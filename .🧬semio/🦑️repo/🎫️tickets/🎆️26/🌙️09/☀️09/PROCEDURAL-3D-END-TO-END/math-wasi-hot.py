#!/usr/bin/env python3
"""⏱️ Selects a completed Flow SDK unit whose linked runtime includes the hot slot fix."""

from pathlib import Path
import subprocess
import sys
import time

lane = Path(sys.argv[1]).resolve()
output = Path(sys.argv[2]).resolve()
toolchain = sys.argv[3]
sdk_units = lane / "wasm32-wasip2/debug/build/semio-os-flow-extensions"
deadline = time.monotonic() + 1200
checked = set()
print("[DEBUG] Waiting for a complete warm WASI SDK with the slot fix", flush=True)
while time.monotonic() < deadline:
    for artifact in sdk_units.glob("*/out/libsemio_os_flow_extensions-*.rmeta"):
        unit = artifact.parent.parent
        if not artifact.with_suffix(".rlib").is_file() or not (unit / "fingerprint/lib-semio_os_flow_extensions.json").is_file():
            continue
        key = (artifact, artifact.stat().st_mtime_ns)
        if key in checked:
            continue
        checked.add(key)
        dependencies = subprocess.check_output(["rustup", "run", toolchain, "rustc", "-Zls=root", str(artifact)], text=True)
        if " semio_plugin_reactor_turn-7f51a2aee075e62a " not in dependencies:
            continue
        if " semio_framework_async-58d3909b58082c10 " not in dependencies:
            continue
        print(f"[DEBUG] Warm hot slot SDK unit {unit.name}", flush=True)
        subprocess.run([
            sys.executable, str(Path(__file__).with_name("math-wasi-native.py")), str(lane), str(output), toolchain,
            "--refresh-schema", f"--sdk-unit={unit.name}",
        ], check=True)
        sys.exit(0)
    time.sleep(15)
sys.exit("No complete matching WASI SDK unit was published within the probe window")
