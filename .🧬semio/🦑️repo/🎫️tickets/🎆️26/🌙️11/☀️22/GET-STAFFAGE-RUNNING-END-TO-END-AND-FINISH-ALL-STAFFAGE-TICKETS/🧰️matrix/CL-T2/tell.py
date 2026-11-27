"""📣️ Tells lane MZ that an asset was rewritten: `python3 tell.py <id>` appends `CL-T2 rewrote <id> at <time>` to the end of `requests.md`, reading the file right before the write; a second call within the same minute adds nothing."""
import sys
import time
from pathlib import Path

path = Path(__file__).resolve().parents[2] / "requests.md"
line = f"CL-T2 rewrote {sys.argv[1]} at {time.strftime('%Y-%m-%d %H:%M')}"
text = path.read_text(encoding="utf-8")
if line in text:
    print(f"requests.md already says: {line}")
else:
    path.write_text(text.rstrip("\n") + "\n\n" + line + "\n", encoding="utf-8")
    print(f"requests.md: {line}")
