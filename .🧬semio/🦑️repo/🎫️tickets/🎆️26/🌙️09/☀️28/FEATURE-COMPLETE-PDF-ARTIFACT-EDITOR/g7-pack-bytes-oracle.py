"""🧮 Independent oracle for the Pack dynamic-bytes corpus: writes the language-neutral JSON with wire hex from the documented grammar (TAG_BYTES 0x08, varint length, raw bytes)."""
import json, sys
from pathlib import Path

def varint(n):
    out = bytearray()
    while n >= 0x80:
        out.append(n & 0x7F | 0x80); n >>= 7
    return bytes(out + bytes([n]))

def top(payload):
    return b"\x00\x01\x01\x11" + b"\x08" + varint(len(payload)) + payload

def keyed(payload):
    inline = lambda text: b"\x07" + varint(len(text)) + text
    return (varint(1) + varint(1) + b"x" + b"\x01\x01\x11" + b"\x10" + varint(2)
            + inline(b"archive") + b"\x08" + varint(len(payload)) + payload + inline(b"kind") + b"\x06\x00")

cases = [("empty", b""), ("small-with-high-bytes", bytes([0, 1, 255, 16])), ("two-byte-length", bytes((i * 7) & 255 for i in range(200)))]
law = {"schema": "semio.pack.dynamic-bytes/v1", "version": 1, "cases": []}
for name, payload in cases:
    law["cases"].append({"id": name, "bytesHex": payload.hex(), "topLevelWireHex": top(payload).hex(), "keyedWireHex": keyed(payload).hex()})
Path(sys.argv[1]).write_text(json.dumps(law, indent=1) + "\n", encoding="utf8")
print("[DEBUG] wrote", len(law["cases"]), "dynamic bytes cases")
