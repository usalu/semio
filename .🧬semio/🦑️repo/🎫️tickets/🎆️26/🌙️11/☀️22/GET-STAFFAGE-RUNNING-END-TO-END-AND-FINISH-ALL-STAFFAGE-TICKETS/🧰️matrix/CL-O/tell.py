"""📣️ Tells lane MZ that an asset was rewritten: `python3 tell.py <id> [--dry]` appends `CL-O rewrote <id> at <time>` to `requests.md` at the end of the closing section of the asset (the gas-hob section or the nine-assets section), reading the file right before the write; a second call within the same minute adds nothing."""
import sys
import time
from pathlib import Path

asset = sys.argv[1]
path = Path(__file__).resolve().parents[2] / "requests.md"
head = "## closing — gas-hob drawn plan depth" if asset == "gas-hob" else "## closing — nine assets against the detail pins"
line = f"CL-O rewrote {asset} at {time.strftime('%Y-%m-%d %H:%M')}"
text = path.read_text(encoding="utf-8")
if line in text:
    print(f"requests.md already says: {line}")
    sys.exit(0)
start = text.index(head)
following = text.find("\n## ", start + 1)
if following < 0:
    result = text.rstrip("\n") + "\n\n" + line + "\n"
else:
    result = text[:following].rstrip("\n") + "\n\n" + line + "\n" + text[following:]
if "--dry" in sys.argv:
    print(result[start:start + 200].replace("\n", " | ")[:200], "...", result[max(0, len(result) - 0):], repr(result[following - 40:following + 80]) if following >= 0 else repr(result[-120:]))
else:
    path.write_text(result, encoding="utf-8")
    print(f"requests.md: {line}")
