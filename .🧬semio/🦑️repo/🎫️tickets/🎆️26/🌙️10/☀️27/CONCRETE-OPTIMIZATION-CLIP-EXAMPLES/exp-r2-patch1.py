"""Replace the model block of the experiments scene with the block file (usage: patch.py <start-marker> <end-marker> <block-file>)."""
import sys
from pathlib import Path

ROOT = Path.cwd()
path = next(ROOT.glob("*teaching/*architecture/*optimization/*clip/*experiments/*.py"))
text = path.read_text(encoding="utf-8")
start = text.index(sys.argv[1])
end = text.index(sys.argv[2])
new = Path(sys.argv[3]).read_text(encoding="utf-8")
path.write_text(text[:start] + new + "\n" + text[end:], encoding="utf-8")
print("patched", path.name, len(text), "->", len(path.read_text(encoding="utf-8")))
