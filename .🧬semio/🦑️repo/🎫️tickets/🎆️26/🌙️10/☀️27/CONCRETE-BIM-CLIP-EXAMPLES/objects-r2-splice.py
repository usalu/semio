"""🪡 Replace def blocks of the objects scene by the text of other files: python objects-r2-splice.py <def name> <replacement file> [<def name> <replacement file> ...]."""
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
target = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py"
lines = target.read_bytes().decode("utf-8").split("\n")
arguments = sys.argv[1:]
for name, replacement in zip(arguments[0::2], arguments[1::2]):
    text = Path(replacement).read_bytes().decode("utf-8")
    start = next(i for i, line in enumerate(lines) if line.startswith(f"    def {name}("))
    stop = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("    def ") or lines[i].startswith("class ")), len(lines))
    new = text.rstrip("\n").split("\n") + [""]
    lines[start:stop] = new
    print("[DEBUG] spliced", name, stop - start, "->", len(new))
payload = "\n".join(lines).encode("utf-8")
for attempt in range(30):
    try:
        target.write_bytes(payload)
        print("[DEBUG] written after", attempt, "retries")
        break
    except OSError as error:
        time.sleep(1)
else:
    raise SystemExit("[DEBUG] write failed")
