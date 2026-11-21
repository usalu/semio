"""📦 Prepares the terminal PDF publication quote against the current source."""

import hashlib
import json
from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
base = Path(sys.argv[1])
source = base / "🚪️io/📥️import/🔁️controlled/🦀️.rs"
work = ticket / "🗑️generated" / "terminal-copy"
receipt = work / "receipt.json"
staged = work / "controlled.rs"
original = source.read_bytes()
before = "Phase::TransferLift | Phase::Publish => size_of::<PdfSnapshot>(),"
after = (
    "Phase::TransferLift => size_of::<PdfSnapshot>(),\n"
    "            Phase::Publish => size_of::<PreparedPdfFile>() + size_of::<PdfSnapshot>(),"
)


def digest(content):
    """🔑 Identifies one complete source file."""
    return hashlib.sha256(content).hexdigest()


assert original.count(before.encode("utf8")) == 1, source
candidate = original.replace(before.encode("utf8"), after.encode("utf8"))
expected = {"before": digest(original), "after": digest(candidate)}
if sys.argv[2:] == ["--prepare"]:
    work.mkdir(parents=True, exist_ok=True)
    staged.write_bytes(candidate)
    receipt.write_text(json.dumps(expected) + "\n", encoding="utf8")
elif sys.argv[2:] == ["--apply-reviewed"]:
    assert len(str(source.resolve()).encode("utf-16-le")) // 2 <= 256, source
    assert json.loads(receipt.read_text(encoding="utf8")) == expected
    assert staged.read_bytes() == candidate
    assert source.read_bytes() == original, source
    source.write_bytes(candidate)
    assert source.read_bytes() == candidate
else:
    raise ValueError("Expected --prepare or --apply-reviewed")
