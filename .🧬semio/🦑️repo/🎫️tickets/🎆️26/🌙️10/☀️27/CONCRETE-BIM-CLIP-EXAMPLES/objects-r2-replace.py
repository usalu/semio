"""🩹 Literal replacements in the objects scene: python objects-r2-replace.py <pairs.py> where the file defines PAIRS = [(old, new), ...]."""
import runpy
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
target = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py"
text = target.read_bytes().decode("utf-8")
for old, new in runpy.run_path(sys.argv[1])["PAIRS"]:
    count = text.count(old)
    print("[DEBUG]", count, old[:60].replace("\n", " "))
    if count == 1:
        text = text.replace(old, new)
payload = text.encode("utf-8")
for attempt in range(40):
    try:
        target.write_bytes(payload)
        print("[DEBUG] written")
        break
    except OSError:
        time.sleep(1)
