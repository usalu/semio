"""Z2 pending codemod (land after W2's Hub Handoff; changes Cargo.lock): give winit its Linux backends.

`winit` is declared `default-features = false, features = ["rwh_06"]`; on Linux that selects no windowing backend and
winit's `compile_error!` stops every native Linux build that enables `semio-framework-ui/wgpu-engine` (semio-hub,
semio-os-mcp, the wgpu host, …). macOS and Windows need no backend feature, which is why only Linux breaks. The backends are
the runtime-loaded ones (`x11` via x11-dl, `wayland` + `wayland-dlopen`), so no system development package is needed.
Usage: python3 winit-linux-backends.py <repo-root> [--dry-run]
"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
DRY = "--dry-run" in sys.argv
BACKENDS = '["rwh_06", "x11", "wayland", "wayland-dlopen"]'
LINUX = "cfg(all(target_os = \"linux\", not(target_arch = \"wasm32\")))"
EDITS = {
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml": 'winit = { version = "0.30.12", default-features = false, features = ["rwh_06"], optional = true }',
    "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml": 'winit = { version = "0.30.12", default-features = false, features = ["rwh_06"] }',
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml": 'winit = { version = "0.30.12", default-features = false, features = ["rwh_06"] }',
}
for relative, line in EDITS.items():
    path = ROOT / relative
    text = path.read_text(encoding="utf-8")
    if text.count(line) != 1:
        sys.exit(f"{relative}: expected exactly one winit declaration, found {text.count(line)}")
    if LINUX in text:
        sys.exit(f"{relative}: Linux winit table already present")
    linux_line = line.replace('["rwh_06"]', BACKENDS)
    block = f"\n# 🐧️ winit selects no windowing backend under `default-features = false`; Linux needs one, loaded at runtime.\n[target.'{LINUX}'.dependencies]\n{linux_line}\n"
    print(f"{'DRY ' if DRY else ''}{relative}: + [target.'{LINUX}'.dependencies] winit {BACKENDS}")
    if not DRY:
        path.write_text(text.rstrip("\n") + "\n" + block, encoding="utf-8")
