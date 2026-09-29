#!/usr/bin/env python3
"""🧪️ CD1 kernel probe workspace: a standalone cargo workspace under the CD1 hub dir holding an APFS clone of the
stdio-semio artifact crate (so kernel edits can be proven without touching the live tree), the root's workspace
package/dependency/lint/profile tables with absolute paths, and a `probe` crate. Never a member of the root workspace.
Usage: cd1-kprobe-setup.py [--refresh-clone]"""
import os, re, shutil, subprocess, sys
REPO = "/Users/ueli/Documents/semio"
PROBE = f"{REPO}/.🧬semio/🌐hub/s14-cd1-work/kprobe"
LIVE_SEMIO = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
LIVE_STEP = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step"
CLONE = f"{PROBE}/🗿️artifacts/🧿️semio"
root = open(f"{REPO}/Cargo.toml", encoding="utf-8").read()

def section(name):
    m = re.search(rf"^\[{re.escape(name)}\]\n(.*?)(?=^\[|\Z)", root, re.S | re.M)
    return m.group(1)

deps = section("workspace.dependencies")
deps = re.sub(r'path = "([^"]+)"', lambda m: f'path = "{m.group(1) if m.group(1).startswith("/") else REPO + "/" + m.group(1)}"', deps)
deps = re.sub(r'^semio-s-artifact-stdio-semio = \{[^\n]*\}$', 'semio-s-artifact-stdio-semio = { path = "🗿️artifacts/🧿️semio/📦️packages/🦀️rust" }', deps, flags=re.M)
manifest = "\n".join([
    "[workspace]", 'members = ["probe", "🗿️artifacts/🧿️semio/📦️packages/🦀️rust"]', 'resolver = "2"', "",
    "[workspace.package]", section("workspace.package").strip(), "",
    "[workspace.dependencies]", deps.strip(), "",
    "[workspace.lints.rust]", section("workspace.lints.rust").strip(), "",
    "[workspace.lints.clippy]", section("workspace.lints.clippy").strip(), "",
    "[profile.dev]", "debug = false", "incremental = false", "",
])
os.makedirs(f"{PROBE}/🗿️artifacts", exist_ok=True)
if "--refresh-clone" in sys.argv and os.path.exists(CLONE):
    shutil.rmtree(CLONE)
if not os.path.exists(CLONE):
    subprocess.run(["cp", "-c", "-R", LIVE_SEMIO, CLONE], check=True)
if not os.path.lexists(f"{PROBE}/🗿️artifacts/📐️step"):
    os.symlink(LIVE_STEP, f"{PROBE}/🗿️artifacts/📐️step")
open(f"{PROBE}/Cargo.toml", "w", encoding="utf-8").write(manifest)
if os.path.exists(f"{REPO}/Cargo.lock") and not os.path.exists(f"{PROBE}/Cargo.lock"):
    shutil.copy(f"{REPO}/Cargo.lock", f"{PROBE}/Cargo.lock")
os.makedirs(f"{PROBE}/probe/src", exist_ok=True)
open(f"{PROBE}/probe/Cargo.toml", "w", encoding="utf-8").write('[package]\nname = "cd1-kernel-probe"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n[dependencies]\nsemio-s-artifact-stdio-semio = { workspace = true }\nparry3d = "0.17"\nserde_json = { workspace = true }\n')
print("kprobe ready", PROBE)
