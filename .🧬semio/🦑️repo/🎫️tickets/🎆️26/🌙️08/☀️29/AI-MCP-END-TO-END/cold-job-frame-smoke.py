"""Compare the compact Rust receiver with a Python struct implementation."""
import json
from pathlib import Path
import struct
import subprocess
import sys

ticket = Path(__file__).parent
law = json.loads((ticket / "cold-job-frame-law.json").read_text(encoding="utf-8"))
suffix = ".exe" if sys.platform == "win32" else ""
binary = (ticket / "🗑️generated" / f"cold-job-frame-smoke{suffix}").resolve()


def make_wire(source):
    kind = source["kind"].encode()
    payload = source["input"]
    checkpoint = source["checkpoint"]
    input_bytes = bytes(payload or [])
    checkpoint_bytes = bytes(checkpoint or [])
    flags = int(payload is not None) + 2 * int(checkpoint is not None)
    wire = struct.pack("<4sB3xQQQQ", b"SJS1", flags, int(source["job"]), len(kind), len(input_bytes), len(checkpoint_bytes))
    fields = (source["job"], kind.hex(), input_bytes.hex() if payload is not None else "-", checkpoint_bytes.hex() if checkpoint is not None else "-")
    return wire + kind + input_bytes + checkpoint_bytes, " ".join(map(str, fields))


def run(wire):
    return subprocess.run([str(binary)], input=wire.hex(), capture_output=True, check=True, text=True).stdout.rstrip("\r\n")


for row in law["cases"]:
    frame, expected = make_wire(row["source"])
    assert frame.hex() == row["hex"]
    assert run(frame) == expected

for row in law["malformed"]:
    frame = bytearray.fromhex(law["cases"][row["case"]]["hex"])
    frame[row["index"]] = row["value"]
    assert run(frame) == "InvalidValue"

assert run(b"SJS1") == "InvalidValue"
large = law["largeInput"]
frame, expected = make_wire({"job":large["job"], "kind":large["kind"], "input":large["pattern"]*large["repeat"], "checkpoint":None})
assert run(frame) == expected
split, expected = make_wire({"job":7, "kind":"a"*1491 + "ü"*1000, "input":[], "checkpoint":[]})
assert run(split) == expected
print(f"[DEBUG] Passed 3 independent wire vectors, 5 refusals, {len(frame)} bytes in bounded pages, and a split UTF-8 Job kind")
