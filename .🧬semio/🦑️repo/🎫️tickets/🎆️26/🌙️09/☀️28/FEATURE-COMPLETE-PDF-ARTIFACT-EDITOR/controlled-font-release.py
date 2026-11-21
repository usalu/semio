"""🧷 Builds and checks one preimage roster for the staged controlled PDF import changes."""

import hashlib
import json
from pathlib import Path
import runpy
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "font-release"
FONTS = "🔨️modules/🔤️fonts"
LIFT = "🔨️modules/⬇️lift/🧩️lifter-and-document"
FONT_ROSTER = (
    f"{FONTS}/🦀️.rs", f"{FONTS}/📦️packages/🦀️rust/🦀️.rs",
    f"{FONTS}/🗺️cmap/🦀️.rs", f"{FONTS}/🗺️cmap/🎚️controlled/🦀️.rs",
    f"{FONTS}/🎚️controlled-opentype/🦀️.rs", f"{LIFT}/🧭️document-lift/🧭️controlled/🦀️.rs",
    f"{FONTS}/🧪️tests/🔬️unit/🦀️.rs", f"{FONTS}/🧪️tests/🔬️unit/📐️simple-opentype.txt",
)


def digest(file):
    return hashlib.sha256(file.read_bytes()).hexdigest() if file.is_file() else None


def layer(directory):
    return json.loads((GENERATED / directory / "stages.json").read_text(encoding="utf8"))


def prepare(base):
    names = ("controlled-dct", "controlled-font-codes", "controlled-tounicode", "controlled-opentype")
    stages = {name: runpy.run_path(str(TICKET / (name + "-stage.py"))) for name in names}
    roster = dict.fromkeys((*stages["controlled-dct"]["DEFAULT"], *stages["controlled-font-codes"]["EDITS"], *FONT_ROSTER))
    preimages = {path: digest(base / path) for path in roster}
    for name in names:
        stages[name]["stage"](base)
    files = {**layer("dct-stage"), **layer("font-opentype")}
    assert set(files) == set(roster), (set(files) - set(roster), set(roster) - set(files))
    assert preimages == {path: digest(base / path) for path in roster}
    OUT.mkdir(parents=True, exist_ok=True)
    receipt = {"live": preimages, "staged": {path: digest(Path(file)) for path, file in files.items()}}
    (OUT / "stages.json").write_text(json.dumps(files, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    (OUT / "release.json").write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Prepared {len(files)} controlled PDF preimages")


def apply_reviewed(base, apply):
    files = json.loads((OUT / "stages.json").read_text(encoding="utf8"))
    expected = json.loads((OUT / "release.json").read_text(encoding="utf8"))
    assert files.keys() == expected["live"].keys() == expected["staged"].keys()
    materialized = {}
    for path, file in files.items():
        target = base / path
        assert len(str(target.resolve()).encode("utf-16-le")) // 2 <= 256, path
        assert digest(target) == expected["live"][path], path
        value = Path(file).read_bytes()
        assert hashlib.sha256(value).hexdigest() == expected["staged"][path], file
        materialized[target] = value
    if apply:
        for target, value in materialized.items():
            assert digest(target) == expected["live"][str(target.relative_to(base)).replace("\\", "/")], target
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(value)
        assert all(digest(base / path) == expected["staged"][path] for path in files), "released sources changed"
    print(f"[DEBUG] {'Released' if apply else 'Verified'} {len(files)} controlled PDF paths")


if __name__ == "__main__":
    base, operation = Path(sys.argv[1]), sys.argv[2]
    if operation == "--prepare":
        prepare(base)
    elif operation in ("--verify", "--apply-reviewed"):
        apply_reviewed(base, operation == "--apply-reviewed")
    else:
        raise ValueError(operation)
