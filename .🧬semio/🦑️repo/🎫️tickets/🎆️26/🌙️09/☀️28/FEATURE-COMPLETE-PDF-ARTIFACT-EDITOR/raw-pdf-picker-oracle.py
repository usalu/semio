"""🧮️ Independently packs the private picker carrier from the language-neutral fixture."""

import json
from pathlib import Path
import struct

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated/rm-sdk/picker-marker.bin"


def verify():
    row = json.loads((TICKET / "raw-pdf-picker-issue.json").read_bytes())
    body = row["documentId"].encode("utf8")
    loan = (row["version"].encode("ascii"), int(row["operationId"], 16), int(row["generation"], 16),
            row["instanceId"], row["kind"], row["maximumFileBytes"], bytes.fromhex(row["baseRevision"]),
            *row["placementWords"], len(body))
    frame = struct.Struct("<4sQQIBI32s8QH")
    assert frame.size == row["headerBytes"] and len(loan[6]) == 32
    encoded = frame.pack(*loan) + body
    assert frame.unpack_from(encoded) == loan and encoded[frame.size:] == body
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_bytes(encoded)
    print(f"[DEBUG] Python struct independently verified the {len(encoded)}-byte private original picker carrier")


if __name__ == "__main__":
    verify()
