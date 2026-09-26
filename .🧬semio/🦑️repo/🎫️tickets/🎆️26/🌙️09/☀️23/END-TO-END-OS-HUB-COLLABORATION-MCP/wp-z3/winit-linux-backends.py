"""🐧️ Z3 landing codemod (session 13 landing window; = Z2 B4 / H10 `patches/linux-winit-backends.py`, re-derived).

`winit` is declared `default-features = false, features = ["rwh_06"]` in the three crates that name it. On Linux the
windowing backends ARE features, so winit's `compile_error!` stopped every native Linux build that enables
`semio-framework-ui/wgpu-engine` (semio-hub through os-infinite, os-mcp, the native wgpu shell). The backends added are
the runtime-loaded ones (`x11` via x11-dl, `wayland` + `wayland-dlopen`): no system development package is needed and a
process that never opens a window never loads them. Each Linux table sits next to the declaration it extends; the ui host
reuses its existing `cfg(target_os = "linux")` table.
Usage: python3 winit-linux-backends.py <repo-root> [--apply]   (dry run without --apply)
"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
APPLY = "--apply" in sys.argv
LINUX = "[target.'cfg(target_os = \"linux\")'.dependencies]"
NOTE = "# 🐧️ winit's Linux backends are features; these load their system libraries at run time (no dev packages)."
BACKENDS = '["rwh_06", "x11", "wayland", "wayland-dlopen"]'
UI = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml"
HOST = "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml"
WGPU = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml"
UI_WINIT = 'winit = { version = "0.30.12", default-features = false, features = ["rwh_06"], optional = true }\n'
PLAIN_WINIT = 'winit = { version = "0.30.12", default-features = false, features = ["rwh_06"] }\n'
HUNKS = {
    UI: (UI_WINIT, UI_WINIT + f"\n{NOTE}\n{LINUX}\n" + UI_WINIT.replace('["rwh_06"]', BACKENDS)),
    HOST: (f"{LINUX}\n", f"{NOTE}\n{LINUX}\n" + PLAIN_WINIT.replace('["rwh_06"]', BACKENDS)),
    WGPU: ('tiny-skia = "0.11.4"\nusvg = "0.45.1"\n', 'tiny-skia = "0.11.4"\nusvg = "0.45.1"\n' + f"\n{NOTE}\n{LINUX}\n" + PLAIN_WINIT.replace('["rwh_06"]', BACKENDS)),
}
problems = 0
for relative, (anchor, replacement) in HUNKS.items():
    path = ROOT / relative
    text = path.read_text(encoding="utf-8")
    if BACKENDS in text:
        print(f"ALREADY {relative}")
        continue
    if text.count(anchor) != 1 or (relative != HOST and text.count(LINUX) != 0):
        print(f"FAIL {relative}: anchor x{text.count(anchor)}, linux table x{text.count(LINUX)}")
        problems += 1
        continue
    print(f"{'APPLY' if APPLY else 'DRY'} {relative}: + winit {BACKENDS} under {LINUX}")
    if APPLY:
        path.write_text(text.replace(anchor, replacement), encoding="utf-8")
sys.exit(1 if problems else 0)
