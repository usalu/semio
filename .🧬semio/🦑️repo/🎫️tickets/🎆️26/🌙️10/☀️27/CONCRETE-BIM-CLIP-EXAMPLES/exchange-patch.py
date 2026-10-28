"""🩹 Replace a block of methods of the exchange scene by the text of a file: python exchange-patch.py <start-def> <end-def> <file>."""
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
target = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py"
start, end, source = sys.argv[1:4]
text = target.read_text(encoding="utf-8")
first, last = text.index("    def " + start), text.index("    def " + end)
target.write_text(text[:first] + (TICKET / source).read_text(encoding="utf-8") + text[last:], encoding="utf-8")
print("[DEBUG] patched", start, "..", end)
