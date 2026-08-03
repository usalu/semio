#!/usr/bin/env python3
from __future__ import annotations
import subprocess
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], cwd=TICKET, text=True).strip())

CRATES = {
    "editor": "semio-framework-editor",
    "flow-core": "semio-framework-os-kernel-flow-core",
}

def crate_script(package_name: str) -> Path:
    for cargo in ROOT.rglob("Cargo.toml"):
        if "node_modules" in cargo.parts or "target" in cargo.parts:
            continue
        try:
            head = cargo.read_text(encoding="utf-8")[:800]
        except Exception:
            continue
        if f'name = "{package_name}"' not in head:
            continue
        candidates = list(cargo.parent.glob("*script.ts"))
        if not candidates:
            raise SystemExit(f"no script.ts beside {cargo}")
        return candidates[0]
    raise SystemExit(f"crate {package_name} not found")

def rebuild(label: str, script: Path) -> None:
    print(f"[rebuild] {label} -> {script}", flush=True)
    pkg = script.parent / "pkg"
    for wasm in pkg.glob("*_bg.wasm"):
        wasm.unlink(missing_ok=True)
    log = TICKET / f"{label}-wasm-build.txt"
    with log.open("w", encoding="utf-8") as fh:
        proc = subprocess.run(["bun", str(script), "wasm"], cwd=ROOT, stdout=fh, stderr=subprocess.STDOUT)
    print(log.read_text(encoding="utf-8")[-2500:], flush=True)
    if proc.returncode != 0:
        raise SystemExit(f"{label} failed with {proc.returncode}")

def main() -> None:
    for label, pkg in CRATES.items():
        rebuild(label, crate_script(pkg))

if __name__ == "__main__":
    main()
