"""📣️ Tells lane MZ that an asset was rewritten: `python3 tell.py <id>` appends `CL-3D rewrote <id> at <time>` to `requests.md` at the end of the section `## closing — model-catalog budgets` (created at the end of the file when missing), reading the file right before the write; a second call within the same minute adds nothing. `tell.py --note <text>` appends the text there instead."""
import sys
import time
from pathlib import Path

path = Path(__file__).resolve().parents[2] / "requests.md"
head = "## closing — model-catalog budgets"
line = sys.argv[2] if sys.argv[1] == "--note" else f"CL-3D rewrote {sys.argv[1]} at {time.strftime('%Y-%m-%d %H:%M')}"
text = path.read_text(encoding="utf-8")
if line in text:
    print(f"requests.md already says: {line[:80]}")
    sys.exit(0)
if head not in text:
    text = text.rstrip("\n") + "\n\n" + head + "\n"
start = text.index(head)
following = text.find("\n## ", start + 1)
result = text.rstrip("\n") + "\n\n" + line + "\n" if following < 0 else text[:following].rstrip("\n") + "\n\n" + line + "\n" + text[following:]
path.write_text(result, encoding="utf-8")
print(f"requests.md: {line[:80]}")
