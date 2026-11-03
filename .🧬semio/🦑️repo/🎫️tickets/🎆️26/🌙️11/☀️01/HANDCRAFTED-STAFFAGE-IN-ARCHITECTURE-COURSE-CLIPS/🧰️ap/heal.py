"""🩹 ap: blank the stale tail after a failed truncate (spaces), so an Edit can drop it."""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "🧰️pa"))
from jio import locate

path = locate(sys.argv[1])
raw = path.read_bytes()
text = raw.decode("utf8")
data, end = json.JSONDecoder().raw_decode(text)
keep = len(text[:end].encode("utf8")) + 1
print("bytes", len(raw), "valid", keep, "tail", len(raw) - keep, repr(text[end:end + 3]))
with open(path, "r+b") as handle:
    handle.seek(keep)
    handle.write(b" " * (len(raw) - keep))
