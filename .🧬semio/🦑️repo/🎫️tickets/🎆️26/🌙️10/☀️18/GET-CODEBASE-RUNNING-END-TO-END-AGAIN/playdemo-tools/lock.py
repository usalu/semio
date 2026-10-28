#!/usr/bin/env python3
"""🔐️ Regenerates `src/s-Cargo.lock` from the CURRENT live manifests without touching the live tree: copies every tracked `Cargo.toml` plus `✏️s/Cargo.lock` and the repo registry index/cache into a scratch dir, runs `cargo update --offline -p wasm-bindgen -p js-sys -p web-sys -p wasm-bindgen-futures` there, then proves the result with `cargo metadata --locked --offline` (cargo = the independent oracle) and a single resolved wasm-bindgen equal to the root pin.
usage: python3 -I lock.py <scratch dir>"""
import json, os, re, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
HOME = os.path.join(ROOT, ".🧬semio", "🦑️repo", "⚡️cache", "rust", "fixed", "m", "cargo", "home", "registry")
def pinned(text): return re.findall(r'^name = "wasm-bindgen"\nversion = "([^"]+)"$', text, re.M)
def main(scratch):
    shutil.rmtree(scratch, ignore_errors=True)
    tree, home = os.path.join(scratch, "tree"), os.path.join(scratch, "home", "registry")
    listed = subprocess.run(["git", "ls-files", "-z", "--", "*Cargo.toml", ":!.🧬semio"], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0")
    for rel in filter(None, listed):
        if os.path.exists(os.path.join(ROOT, rel)):
            os.makedirs(os.path.join(tree, os.path.dirname(rel)), exist_ok=True); shutil.copyfile(os.path.join(ROOT, rel), os.path.join(tree, rel))
    for name in ("rust-toolchain.toml",): shutil.copyfile(os.path.join(ROOT, name), os.path.join(tree, "✏️s", name))
    shutil.copyfile(os.path.join(ROOT, "✏️s", "Cargo.lock"), os.path.join(tree, "✏️s", "Cargo.lock"))
    for part in ("index", "cache"): shutil.copytree(os.path.join(HOME, part), os.path.join(home, part))
    env = {**os.environ, "CARGO_HOME": os.path.dirname(home)}
    cwd = os.path.join(tree, "✏️s")
    subprocess.run(["cargo", "update", "--offline", "-p", "wasm-bindgen", "-p", "js-sys", "-p", "web-sys", "-p", "wasm-bindgen-futures"], cwd=cwd, env=env, check=True)
    meta = subprocess.run(["cargo", "metadata", "--locked", "--offline", "--format-version", "1"], cwd=cwd, env=env, capture_output=True, check=True)
    versions = sorted({p["version"] for p in json.loads(meta.stdout)["packages"] if p["name"] == "wasm-bindgen"})
    root_pin = pinned(open(os.path.join(ROOT, "Cargo.lock"), encoding="utf8").read())
    text = open(os.path.join(cwd, "Cargo.lock"), encoding="utf8").read()
    if versions != root_pin or pinned(text) != root_pin: sys.exit(f"[DEBUG] lock.py: resolved {versions}, lock {pinned(text)}, root pin {root_pin}")
    shutil.copyfile(os.path.join(cwd, "Cargo.lock"), os.path.join(HERE, "src", "s-Cargo.lock"))
    print("[DEBUG] lock.py: ✏️s/Cargo.lock regenerated, cargo --locked OK, wasm-bindgen", versions)
    shutil.rmtree(scratch, ignore_errors=True)
main(sys.argv[1])
