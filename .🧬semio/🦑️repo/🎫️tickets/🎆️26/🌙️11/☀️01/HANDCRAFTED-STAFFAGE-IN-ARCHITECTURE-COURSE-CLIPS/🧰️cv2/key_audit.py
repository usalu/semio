import hashlib
import importlib
import json
import shutil
import subprocess
import sys
import time
from pathlib import Path

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
T = Path(__file__).resolve().parent.parent
G = T / "🗑️generated/cv2/v"
SRC = S._SOURCES
ASSETS = {"pendant-lamp": "🔌️appliances", "panel-radiator": "🧱️components"}
CASES = {
    "line": {"asset": "pendant-lamp", "pose": "on", "view": "dark-line", "theme": "dark"},
    "thermal": {"asset": "pendant-lamp", "pose": "on", "view": "thermal", "environment": "office-neutral", "theme": "dark", "thermalSpanK": 25},
}
EXPORTER = "🔨️modules/📤️export/🎭️cast/🟦️.ts"
ASSET = "🗂️catalog/🔌️appliances/pendant-lamp/🔣️.json"
OTHER = "🗂️catalog/🧱️components/panel-radiator/🔣️.json"
ENV = "🗂️catalog/🌤️environments/🔣️.json"
PROFILES = "🗂️catalog/🖨️print-profiles/🔣️.json"
CATALOG = "🗂️catalog/🔣️.json"


def build(destination: Path) -> None:
    shutil.rmtree(destination, ignore_errors=True)
    for folder in ("🔨️modules", "🧬️schema"):
        shutil.copytree(SRC / folder, destination / folder, ignore=shutil.ignore_patterns("node_modules", "target"))
    shutil.copytree(SRC / "🎯️targets/⚛️react/🔨️modules/📤️downloads/🗜️zip", destination / "🎯️targets/⚛️react/🔨️modules/📤️downloads/🗜️zip")
    for name in (CATALOG, ENV, PROFILES, ASSET, OTHER):
        (destination / name).parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(SRC / name, destination / name)


def edit(root: Path, name: str, old: str, new: str) -> None:
    path = root / name
    text = path.read_text(encoding="utf-8")
    assert old in text, (name, old)
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def rx_edit(root: Path, name: str, pattern: str, replacement: str) -> None:
    import re

    path = root / name
    text, count = re.subn(pattern, replacement, path.read_text(encoding="utf-8"), count=1)
    assert count == 1, (name, pattern)
    path.write_text(text, encoding="utf-8")


def append(root: Path, name: str, text: str) -> None:
    path = root / name
    path.write_text(path.read_text(encoding="utf-8") + text, encoding="utf-8")


def profile_switch(root: Path) -> None:
    path = root / PROFILES
    document = json.loads(path.read_text(encoding="utf-8"))
    others = [p["id"] for p in document["profiles"] if p["id"] != document["defaultProfile"]]
    document["defaultProfile"] = others[0]
    path.write_text(json.dumps(document, ensure_ascii=False, indent=2), encoding="utf-8")


def profile_number(root: Path) -> None:
    path = root / PROFILES
    document = json.loads(path.read_text(encoding="utf-8"))
    profile = next(p for p in document["profiles"] if p["id"] == document["defaultProfile"])
    profile["minStrokeMm"] = 0.12
    path.write_text(json.dumps(document, ensure_ascii=False, indent=2), encoding="utf-8")


VARIANTS = {
    "baseline": lambda r: None,
    "asset: cord width (drawn)": lambda r: edit(r, ASSET, '"w": 0.0066', '"w": 0.0166'),
    "asset: summary text": lambda r: rx_edit(r, ASSET, r'("summary":\s*\{\s*"en":\s*")', r"\1Changed. "),
    "asset: unused own environment": lambda r: edit(r, ASSET, '"relativeHumidity": 0.5', '"relativeHumidity": 0.45'),
    "asset: bulb area (thermal)": lambda r: edit(r, ASSET, '"areaM2": 0.012', '"areaM2": 0.02'),
    "other asset: edited": lambda r: append(r, OTHER, "\n"),
    "environment: used preset office-neutral": lambda r: edit(r, ENV, '"airTempC": 22', '"airTempC": 24'),
    "environment: unused preset summer-hot-sun": lambda r: edit(r, ENV, '"id": "summer-hot-sun"', '"id": "summer-hot-sun", "x-note": 1'),
    "profiles: default profile switched": profile_switch,
    "profiles: a number of the default profile": profile_number,
    "catalog manifest: category title": lambda r: edit(r, CATALOG, '"en": "Humans"', '"en": "People"'),
    "exporter: margin 1 -> 2 mm (behaviour)": lambda r: edit(r, EXPORTER, "const MARGIN_MM = 1;", "const MARGIN_MM = 2;"),
    "exporter: comment only": lambda r: append(r, EXPORTER, "\n// comment\n"),
    "module outside the closure: pdf exporter comment": lambda r: append(r, "🔨️modules/📤️export/📕️pdf/🟦️.ts", "\n// comment\n"),
    "module outside the closure: dwg rust source": lambda r: append(r, "🔨️modules/📤️export/🖊️dwg/📦️packages/🦀️rust/🦀️.rs", "\n// comment\n"),
    "module in the closure: schema twin comment": lambda r: append(r, "🧬️schema/🟦️.ts", "\n// comment\n"),
    "stray editor file in the modules": lambda r: (r / "🔨️modules/stray.orig").write_text("x", encoding="utf-8"),
}


def render(root: Path, out: Path) -> str:
    shutil.rmtree(out, ignore_errors=True)
    manifest = root.parent / (root.name + ".json")
    manifest.write_text(json.dumps({"version": 1, "casts": CASES}, ensure_ascii=False), encoding="utf-8")
    command = [S._repo(), "tools", "exec", "--light", "bun", "--", str(root / EXPORTER), "--manifest", str(manifest), "--ids", ",".join(CASES), "--out", str(out), "--catalog", str(root / "🗂️catalog")]
    run = subprocess.run(command, cwd=S._WORKSPACE, capture_output=True, text=True, encoding="utf-8", errors="replace", check=False)
    if run.returncode:
        failure = "error:" + hashlib.sha256(run.stderr.strip().splitlines()[-1].encode()).hexdigest()[:8]; print("   (render failed:", run.stderr.strip().splitlines()[-1][:120], ")", flush=True); return {name: failure for name in CASES}
    return {name: hashlib.sha256(b"".join(f.name.encode() + f.read_bytes() for f in sorted((out / name).iterdir()))).hexdigest()[:10] for name in CASES}


def keys(root: Path) -> dict:
    return {name: S.cast_key(entry, root)[:10] for name, entry in CASES.items()}


def main(names: list[str]) -> None:
    G.mkdir(parents=True, exist_ok=True)
    base = G / "base"
    build(base)
    base_keys, base_out = keys(base), render(base, G / "base-out")
    again = render(base, G / "base-out2")
    print("determinism (same tree rendered twice):", again == base_out, flush=True)
    for position, name in enumerate(names):
        root = G / f"t{position}"
        shutil.rmtree(root, ignore_errors=True)
        shutil.copytree(base, root)
        VARIANTS[name](root)
        k, o = keys(root), render(root, G / f"t{position}-out")
        verdict = []
        for case in CASES:
            key_changed, out_changed = k[case] != base_keys[case], o[case] != base_out[case]
            tag = "ok" if key_changed == out_changed else ("OVER-INVALIDATES" if key_changed else "STALE-CACHE")
            verdict.append(f"{case}: key {'changes' if key_changed else 'same'}, output {'changes' if out_changed else 'same'} -> {tag}")
        print(f"{name}\n    " + "\n    ".join(verdict), flush=True)
        shutil.rmtree(root, ignore_errors=True)
        shutil.rmtree(G / f"t{position}-out", ignore_errors=True)


if __name__ == "__main__":
    wanted = sys.argv[1:] or [n for n in VARIANTS if n != "baseline"]
    main(wanted)
