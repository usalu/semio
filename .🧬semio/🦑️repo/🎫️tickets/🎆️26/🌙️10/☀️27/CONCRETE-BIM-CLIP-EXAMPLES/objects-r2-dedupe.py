"""🧽 Delete the later duplicate of a def block in the objects scene: python objects-r2-dedupe.py <name:occurrence> ..."""
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
target = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py"
lines = target.read_bytes().decode("utf-8").split("\n")
for item in sys.argv[1:]:
    name, occurrence = item.split(":")
    starts = [i for i, line in enumerate(lines) if line.startswith(f"    def {name}(")]
    start = starts[int(occurrence)]
    stop = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("    def ") or lines[i].startswith("class ")), len(lines))
    del lines[start:stop]
    print("[DEBUG] removed", name, occurrence, stop - start)
payload = "\n".join(lines).encode("utf-8")
for attempt in range(40):
    try:
        target.write_bytes(payload)
        print("[DEBUG] written after", attempt, "retries")
        break
    except OSError:
        time.sleep(1)
