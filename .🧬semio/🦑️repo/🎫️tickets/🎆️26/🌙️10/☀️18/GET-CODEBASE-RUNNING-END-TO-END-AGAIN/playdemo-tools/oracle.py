#!/usr/bin/env python3
"""🐍️ Independent oracle for batch `wasmlock` (stdlib only, regex instead of a TOML parser): lists every project target declaring `wasmPackageSourceV1`, resolves its Cargo workspace (nearest manifest, the owner's own included, with a `[workspace]` or `[workspace.*]` table), and checks that the lockfile is declared in the wasm-bindgen registry and pins the root `Cargo.lock`'s wasm-bindgen. Project files come from `git ls-files` of `--git` (default ROOT), contents from ROOT (a mirror passes its own root).
usage: python3 -I oracle.py <root> [--git <git work tree>]   exit 0 = all owners consistent, 1 = at least one violation (printed)"""
import json, os, re, subprocess, sys
REGISTRY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🕸️wasm-bindgen/🔣️.json"
WORKSPACE = re.compile(r"^\s*\[workspace(\.[A-Za-z0-9_-]+)*\]\s*(#.*)?$", re.M)
def bindgen(text): return re.findall(r'^name = "wasm-bindgen"\r?\nversion = "([^"]+)"$', text, re.M)
def lock_of(root, owner):
    directory = owner
    while True:
        manifest = os.path.join(root, directory, "Cargo.toml")
        if os.path.exists(manifest) and WORKSPACE.search(open(manifest, encoding="utf8").read()): return os.path.normpath(os.path.join(directory, "Cargo.lock")).replace("\\", "/")
        if directory in ("", "."): return os.path.normpath(os.path.join(owner, "Cargo.lock")).replace("\\", "/")
        directory = os.path.dirname(directory) or "."
def main():
    root = os.path.abspath(sys.argv[1]); git = os.path.abspath(sys.argv[sys.argv.index("--git") + 1]) if "--git" in sys.argv else root
    declared = json.load(open(os.path.join(root, REGISTRY), encoding="utf8"))["wasmLocks"]
    pin = bindgen(open(os.path.join(root, "Cargo.lock"), encoding="utf8").read())
    files = subprocess.run(["git", "ls-files", "-z", "--", "*📋️project.json", ":!.🧬semio"], cwd=git, capture_output=True, check=True).stdout.decode().split("\0")
    bad = rows = 0
    for rel in filter(None, files):
        path = os.path.join(root, rel)
        if not os.path.exists(path): continue
        for name, target in (json.load(open(path, encoding="utf8")).get("targets") or {}).items():
            owner = ((target.get("metadata") or {}).get("semio") or {}).get("wasmPackageSourceV1", {}).get("sourceOwner") if isinstance(target, dict) else None
            if not owner: continue
            rows += 1
            lock = lock_of(root, owner)
            found = bindgen(open(os.path.join(root, lock), encoding="utf8").read()) if os.path.exists(os.path.join(root, lock)) else ["missing"]
            ok = lock in declared and found == pin
            bad += not ok
            print("ok  " if ok else "BAD ", name, owner[-70:], "->", lock, found, "declared" if lock in declared else "UNDECLARED")
    print(f"[DEBUG] oracle: {rows} wasm source owners, {bad} violations, root pin {pin}")
    sys.exit(1 if bad or not rows else 0)
main()
