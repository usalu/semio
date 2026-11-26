"""📮️ Independently derive the persisted multipart transport fixture identity."""

from hashlib import sha256
from json import loads
from pathlib import Path
from sys import argv

fixture = loads(Path(argv[1]).read_text())
prefix = bytes.fromhex("80004700")
message = prefix + bytes([fixture["atomicPayloadByte"]]) * (fixture["atomicPayloadBytes"] - len(prefix))
assert sha256(message).hexdigest() == fixture["atomicSha256"]
part_size = 1 << 16
assert [len(message[:part_size]), len(message[part_size:])] == [65536, 11]
assert bytes(fixture["ordinary"]) != bytes(fixture["trailing"])
print("[DEBUG] Verified SHA-256 and the 65,536/11-byte multipart boundary")
