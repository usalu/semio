"""🎞️ Count decoded source frames before deriving finite composition and audio duration."""
from pathlib import Path
import ast
import os

ROOT = Path(__file__).resolve().parents[7]
TARGET = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🐍️.py"
value = TARGET.read_text(encoding="utf-8")
before = 'run(["-i", str(content), "-map", "0:v:0", "-c:v", "copy", "-f", "null", "-"], "probe.log")'
after = 'run(["-i", str(content), "-map", "0:v:0", "-an", "-f", "null", "-"], "probe.log")'
if before not in value:
    raise RuntimeError("The owned frame count probe span changed")
value = value.replace(before, after, 1)
ast.parse(value)
temporary = Path(__file__).parent / "🗑️generated/content-camera/frame-count/probe.tmp"
temporary.parent.mkdir(parents=True, exist_ok=True)
temporary.write_text(value, encoding="utf-8")
os.replace(temporary, TARGET)
print("[DEBUG] Source frame probe now counts decoded frames")
