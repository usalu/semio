"""Splice scratch method files into the representations scene before a marker method."""
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
target = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/👁️representations/🐍️.py"
text = target.read_text(encoding="utf-8")
marker = sys.argv[1]
assert text.count(marker) == 1, marker
tail = "".join(Path(path).read_text(encoding="utf-8") for path in sys.argv[2:])
target.write_text(text.replace(marker, tail + marker), encoding="utf-8")
print("[DEBUG] spliced", len(tail))
