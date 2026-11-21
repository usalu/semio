"""🧷 Releases the reviewed controlled JPEG changes against their exact preimages."""

import hashlib
import json
from pathlib import Path
import runpy
import sys

ticket = Path(__file__).resolve().parent
out = ticket / "🗑️generated" / "dct-stage"
receipt = out / "release.json"
base = Path(sys.argv[1])
requested = sys.argv[2:]


def sha256(path):
    """🔏 Computes the byte identity of one live or staged source."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


if requested == ["--prepare"]:
    stage = runpy.run_path(str(ticket / "controlled-dct-stage.py"))
    roster = tuple(stage["DEFAULT"])
    originals = {path: sha256(base / path) for path in roster}
    files = stage["stage"](base)
    assert tuple(files) == roster
    assert originals == {path: sha256(base / path) for path in roster}
    staged = {path: sha256(Path(file)) for path, file in files.items()}
    receipt.write_text(json.dumps({"live": originals, "staged": staged}, ensure_ascii=False) + "\n", encoding="utf8")
    print(f"[DEBUG] Captured {len(files)} controlled JPEG preimages")
elif requested == ["--apply-reviewed"]:
    files = json.loads((out / "stages.json").read_text(encoding="utf8"))
    expected = json.loads(receipt.read_text(encoding="utf8"))
    assert files.keys() == expected["live"].keys() == expected["staged"].keys()
    for path, file in files.items():
        target = base / path
        assert len(str(target.resolve()).encode("utf-16-le")) // 2 <= 256, path
        assert sha256(target) == expected["live"][path], path
        assert sha256(Path(file)) == expected["staged"][path], file
    for path, file in files.items():
        target = base / path
        assert sha256(target) == expected["live"][path], path
        target.write_bytes(Path(file).read_bytes())
        assert sha256(target) == expected["staged"][path], path
    print(f"[DEBUG] Released {len(files)} verified controlled JPEG paths")
else:
    raise ValueError("Expected --prepare or --apply-reviewed")
