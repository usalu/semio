"""🪪️ Re-derives 🪪️presence-normalization-v1's normalized peers for the bit-11 principal kind.

The rule is the wire's own, quoted from 📡️replication/📡️wire/🦀️.rs:1643 — "Bit 11 carries
`principal_kind` as one declaration-order tag byte (`0` human, `1` agent)", appended after every
other optional field. So the transform of a pre-agent normalized peer into the shape a hub that
stamps the principal server-side emits is exact and mechanical: set bit 11 in the LEB128 flag word
that follows the actor text, and append the human tag byte. Nothing is copied from hub output; the
neutral TS oracle re-derives the result independently.
"""
import json, sys, pathlib

FIXTURE = pathlib.Path("🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json")

def uleb(value):
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        out.append(byte | (0x80 if value else 0))
        if not value:
            return bytes(out)

def read_uleb(data, at):
    value = shift = 0
    while True:
        byte = data[at]
        at += 1
        value |= (byte & 0x7F) << shift
        if not byte & 0x80:
            return value, at
        shift += 7

def stamp(hex_text):
    data = bytes.fromhex(hex_text)
    length, at = read_uleb(data, 0)
    actor_end = at + length
    flags, flags_end = read_uleb(data, actor_end)
    if flags & (1 << 11):
        return hex_text, False
    return (data[:actor_end] + uleb(flags | (1 << 11)) + data[flags_end:] + b"\x00").hex(), True

fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
changed = 0
for vector in fixture["vectors"]:
    current = vector["expected"].get("normalizedPeerHex")
    if not current:
        continue
    updated, did = stamp(current)
    vector["expected"]["normalizedPeerHex"] = updated
    changed += did
if "--write" in sys.argv:
    FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(f"presence-principal-kind: vectors={len(fixture['vectors'])} stamped={changed} written={'--write' in sys.argv}")
