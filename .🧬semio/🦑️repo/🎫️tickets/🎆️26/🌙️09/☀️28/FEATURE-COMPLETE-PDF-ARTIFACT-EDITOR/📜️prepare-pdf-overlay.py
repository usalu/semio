"""📦️ Prepares and checks the combined PDF overlay against the current shared files."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
root = Path.cwd()
arguments = argparse.ArgumentParser()
arguments.add_argument("base", type=Path)
arguments.add_argument("--apply-reviewed", action="store_true")
args = arguments.parse_args()
base = args.base
receipt = generated / "pdf-overlay-receipt.json"

paths = {
    "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs": "mixed/document.rs",
    "🔨️modules/🖼️images/🎞️controlled/🦀️.rs": "mixed/images.rs",
    "🧪️tests/🧫️fixtures/🎞️controlled-images/🔣️.json": "mixed/fixture.json",
    "🔨️modules/🔤️fonts/🦀️.rs": "text-font-edit/codec.rs",
    "🔨️modules/🔤️fonts/✒️cff/🦀️.rs": "simple-cff-stage/cff.rs",
    "🔨️modules/🔤️fonts/✒️cff/🧫️contracts.json": "simple-cff-stage/🧫️contracts.json",
    "✏️editor/🖼️page/🗺️scene/🔠️truetype-text/🦀️.rs": "simple-cff-stage/painter.rs",
    "✏️editor/🖼️page/🔤️metrics/🦀️.rs": "text-font-edit/metrics.rs",
    "✏️editor/🖼️page/🧮️content-ops/🦀️.rs": "text-font-edit/content-ops.rs",
    "✏️editor/🖼️page/🖱️object-edits/🦀️.rs": "text-font-edit/object-edits.rs",
    "✏️editor/🧪️tests/🔬️unit/🦀️.rs": "text-font-edit/editor-tests.rs",
    "🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json": "text-font-edit/natural-file-fixture.json",
}
owners = {str(base / destination): candidate for destination, candidate in paths.items()}
boundary = json.loads((generated / "boundary-draft-narrow/overlay.json").read_text())
owners.update({destination: "boundary-draft-narrow/" + candidate for destination, candidate in boundary.items()})
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None

def current_files():
    return {destination: digest(root / destination) for destination in owners}

if args.apply_reviewed:
    expected = json.loads(receipt.read_text())
    assert expected["owners"] == owners, "overlay roster changed after preparation"
    assert expected["live"] == current_files(), "live files changed after overlay preparation"
    for original, candidate in owners.items():
        assert digest(generated / candidate) == expected["staged"][candidate], candidate
        assert len(str(root / original).encode("utf-16-le")) // 2 <= 256, original
    for original, candidate in owners.items():
        path = root / original
        assert digest(path) == expected["live"][original], f"live file changed before {original}"
        path.write_bytes((generated / candidate).read_bytes())
    print(f"[DEBUG] Applied and verified input digests for {len(owners)} PDF overlay paths")
else:
    before = current_files()
    environment = dict(os.environ, PYTHONPATH=str(generated / "fonttools"), UV_CACHE_DIR=str(generated / "uv"))
    for name in ["📜️mixed-resource-stage.py", "📜️mixed-resource-editor-stage.py", "📜️raw-cff-stage.py", "📜️simple-cff-stage.py",
                 "📜️cff-contract-stage.py", "📜️font-reassignment-stage.py", "📜️font-reassignment-editor-stage.py", "📐️boundary-draft-narrow.py"]:
        tool = ["python", "--"]
        if name == "📜️cff-contract-stage.py":
            tool = ["uv", "--", "run", "--offline", "--no-project", "--with", "PyMuPDF==1.24.14"]
        subprocess.run(["repo", "tools", "exec", *tool, str(ticket / name), str(base)], env=environment, check=True)
    assert boundary == json.loads((generated / "boundary-draft-narrow/overlay.json").read_text()), "boundary roster changed"
    assert before == current_files(), "live sources changed during overlay preparation"
    staged = {candidate: digest(generated / candidate) for candidate in owners.values()}
    assert all(staged.values())
    receipt.write_text(json.dumps({"owners": owners, "live": before, "staged": staged}, ensure_ascii=False, indent=2) + "\n")
    print(f"[DEBUG] Prepared {len(owners)} PDF overlay paths against verified input digests")
