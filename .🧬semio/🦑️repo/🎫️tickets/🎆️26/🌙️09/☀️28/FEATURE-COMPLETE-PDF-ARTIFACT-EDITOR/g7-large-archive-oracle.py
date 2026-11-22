"""🧮 Independent oracle for the large persisted document archive law: Python framing, SHA-256 and FNV-1a64. The schema is checked by Ajv in the TypeScript law."""
import hashlib
import json
import sys
from pathlib import Path

MASK = (1 << 64) - 1
LAW = Path(sys.argv[1])


def stream(seed, length):
    state, out = int(seed), bytearray()
    while len(out) < length:
        state = (state + 0x9E3779B97F4A7C15) & MASK
        z = ((state ^ (state >> 30)) * 0xBF58476D1CE4E5B9) & MASK
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
        out += ((z ^ (z >> 31)) & MASK).to_bytes(8, "little")
    return bytes(out[:length])


def varint(value):
    out = bytearray()
    while value >= 0x80:
        out.append(value & 0x7F | 0x80)
        value >>= 7
    return bytes(out + bytes([value]))


def fnv1a64(data):
    h = 0xCBF29CE484222325
    for byte in data:
        h = ((h ^ byte) * 0x100000001B3) & MASK
    return f"{h:016x}"


def archive(case):
    pack, spr = stream(case["packSeed"], case["packBytes"]), stream(case["sprSeed"], case["sprBytes"])
    return b"\x01" + varint(len(pack)) + pack + varint(len(spr)) + spr + b"\x00"


law = json.loads(LAW.read_text(encoding="utf8"))
if "--write" in sys.argv:
    for case in law["cases"]:
        data = archive(case)
        case.update(archiveBytes=len(data), fnv1a64=fnv1a64(data), sha256=hashlib.sha256(data).hexdigest())
        case["outcome"] = "accepted" if len(data) <= law["ceiling"] else "refused"
    LAW.write_text(json.dumps(law, indent=2) + "\n", encoding="utf8")
for case in law["cases"]:
    data = archive(case)
    assert len(data) == case["archiveBytes"] and fnv1a64(data) == case["fnv1a64"] and hashlib.sha256(data).hexdigest() == case["sha256"], case["name"]
    assert (len(data) <= law["ceiling"]) == (case["outcome"] == "accepted"), case["name"]
assert any(case["archiveBytes"] > law["former"] and case["outcome"] == "accepted" for case in law["cases"])
print("[DEBUG] large archive law matches the independent Python framing", [c["archiveBytes"] for c in law["cases"]])
