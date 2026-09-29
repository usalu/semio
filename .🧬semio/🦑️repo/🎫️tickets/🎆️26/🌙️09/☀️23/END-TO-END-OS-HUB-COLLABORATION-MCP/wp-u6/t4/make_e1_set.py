#!/usr/bin/env python3
"""🧰️ Generates the E1 set script (paged DOCX route) from the 10:28 pre-E1 snapshot `s14-u6-base2` → the E1 overlay, for the E1
file list only (the overlay also carries the T4 set, which ships as row 6). Anchored hunks per edited file, new files verbatim,
removed files deleted; the generator proves itself (hunks applied to the base reproduce the overlay). `--base`/`--target` swap in a
rebased pair (`rebase_e1.py`). Usage: make_e1_set.py <out.py> [--base <dir> --target <dir>]"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from make_set import apply, hunks  # noqa: E402

HUB = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub")
BASE, OVERLAY = HUB / "s14-u6-base2", HUB / "s14-u6-overlay"
STDIO = "✏️s/🔌️plugins/🗄️stdio/"
DOCX = STDIO + "🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/"
SEMIO = STDIO + "🗿️artifacts/🧿️semio/"
EDITED = [
    STDIO + "📇️registry/🧬️contract/✏️editing/🦀️.rs",
    STDIO + "📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs",
    SEMIO + "🦀️.rs",
    SEMIO + "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/📬️preparation/🦀️.rs",
    SEMIO + "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/📬️preparation/🦀️.rs",
    DOCX + "🧱️base/✏️editor/📬️preparation/🦀️.rs",
    DOCX + "🧱️base/✏️editor/🦀️.rs",
    DOCX + "📏️strict/✏️editor/🦀️.rs",
    DOCX + "🔄️transitional/✏️editor/🦀️.rs",
    DOCX + "🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs",
]
CREATED = [STDIO + "📇️registry/🧬️contract/✏️editing/🏗️structural/🦀️.rs"]
REMOVED = [SEMIO + "✏️editor/📬️preparation/🦀️.rs"]


def main():
    out = Path(sys.argv[1])
    base = Path(sys.argv[sys.argv.index("--base") + 1]) if "--base" in sys.argv else BASE
    overlay = Path(sys.argv[sys.argv.index("--target") + 1]) if "--target" in sys.argv else OVERLAY
    sets = {}
    for rel in EDITED:
        old, new = (base / rel).read_text(), (overlay / rel).read_text()
        pairs = hunks(old, new)
        assert pairs is not None and apply(old, pairs) == new, rel
        sets[rel] = pairs
    news = {rel: (overlay / rel).read_text() for rel in CREATED}
    for rel in REMOVED:
        assert (base / rel).exists() and not (overlay / rel).exists(), rel
    template = (Path(__file__).parent / "e1_set_template.py").read_text()
    out.write_text(template.replace("__SETS__", json.dumps(json.dumps(sets))).replace("__NEWS__", json.dumps(json.dumps(news))).replace("__REMOVED__", json.dumps(REMOVED, ensure_ascii=False)))
    print(f"{out}: {len(sets)} anchored, {len(news)} new, {len(REMOVED)} removed")


if __name__ == "__main__":
    main()
