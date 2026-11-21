"""🔬 Audits staged WGPU Save with native and browser Rust source biases."""

from pathlib import Path
import json
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
STAGE = TICKET / "🗑️generated/native-wgpu-save" / (sys.argv[1] if len(sys.argv) > 1 else "bridge")
RUSTC = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
TARGETS = ("aarch64-apple-darwin", "x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc", "wasm32-unknown-unknown")

biases = {}
for target in TARGETS:
    rows = subprocess.check_output([*RUSTC, "--print", "cfg", "--target", target], text=True).splitlines()
    biases[target] = [row.replace(chr(34), "") for row in rows if row and not row.startswith("[")]
    if "debug_assertions" not in biases[target]:
        biases[target].append("debug_assertions")
runner = ["repo", "tools", "exec", "--light", "bun", "--", str(TICKET / "📜️native-wgpu-save-closure.ts")]
subprocess.run(runner, input=json.dumps({"stage": str(STAGE), "targets": biases, "live": "--live" in sys.argv[2:], "kernel": STAGE.name == "kernel-visibility"}), text=True, check=True)
