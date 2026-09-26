#!/usr/bin/env python3
"""🐧️ H10 post-publish patch set (ticket 26/09/23 session 12): winit on Linux gets its X11 + Wayland backends.

SUPERSEDED for landing by Z2's `wp-z2/pending/winit-linux-backends.py` (the same fix for all three crates that declare
winit: `🖱️ui`, `🖱️ui/🖥️host`, the wgpu renderer target) — land that one, once. This script remains only because it is
what H10's Docker proof (`--root` on a context copy, ui crate only, which is the hub's whole winit path) was run with.

`semio-framework-ui`'s `wgpu-engine` feature links `winit` with `default-features = false` and only `rwh_06`. On macOS and
Windows winit always has its platform backend; on Linux the backends ARE features, so every Linux build of a crate
that enables `wgpu-engine` — the native wgpu shell, and the hub through `semio-framework-os-infinite` — dies in
`winit/src/platform_impl/mod.rs:78` "The platform you're compiling for is not supported by winit" (measured: the first
cold `docker build` of 🌎️hub/Dockerfile, 2026-09-26 05:19, `.🧬semio/🌐hub/s12-h10-logs/docker-build-3.txt`).
Both backends load their system libraries at runtime (`x11-dl`, `wayland-dlopen`), so the build needs no system
packages and a headless server that never opens a window never loads them.

Touches `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` (+ Cargo.lock on the next cargo run): lands only in
the post-publish window (guest freeze, rule 20). usage: linux-winit-backends.py [--apply] [--root <repository copy>]  (dry run on the tree by default; --root patches a copy,
       e.g. a docker build context, so the image can be proven before the landing window)
"""
import pathlib, sys
ROOT = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else pathlib.Path("/Users/ueli/Documents/semio")
path = ROOT / "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml"
text = path.read_text(encoding="utf-8")
anchor = """[target.'cfg(not(target_os = "wasi"))'.dependencies]
winit = { version = "0.30.12", default-features = false, features = ["rwh_06"], optional = true }
"""
addition = """
# 🐧️ On Linux winit's platform backends are features, and without one winit refuses to compile, so every Linux
# build that enables `wgpu-engine` failed. Both backends load their system libraries at run time (`x11-dl`,
# `wayland-dlopen`): a build needs no system packages and a process that never opens a window never loads them.
[target.'cfg(all(target_os = "linux", not(target_arch = "wasm32")))'.dependencies]
winit = { version = "0.30.12", default-features = false, features = ["rwh_06", "x11", "wayland", "wayland-dlopen"], optional = true }
"""
if addition.strip() in text:
    print("already applied"); sys.exit(0)
if text.count(anchor) != 1:
    print("anchor not found exactly once — re-derive the hunk"); sys.exit(1)
patched = text.replace(anchor, anchor + addition)
print(f"hunk OK: {path} (+{addition.count(chr(10))} lines after the winit target table)")
if "--apply" in sys.argv:
    path.write_text(patched, encoding="utf-8"); print("applied")
else:
    print("dry run: nothing written")
