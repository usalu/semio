"""🌀 Reproduce the picker cursor guard in an isolated copy of the profiled Wasm."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
from sys import argv

parser = run_path(str(Path(__file__).with_name("symbolicate-note-startup.py")))
source, output = map(Path, argv[1:])
original = source.read_bytes()
assert sha256(original).hexdigest() == "f41fbb35c5b1e0137524dd9d2fd6b2f925b9716a43f22440565f4057e74d6739"
assert output.parent.name == "🗑️generated"
assert not output.exists()
_, code, end = next(section for section in parser["sections"](original, 0, len(original)) if section[0] == 10)
body_start = 31323800
body_size = 3101
insertion = body_start + 289
assert original[insertion - 2:insertion + 2] == bytes.fromhex("03 40 02 40")
guard = bytes.fromhex("20 03 28 02 04 20 10 49 0d 01")


def encode_leb(value):
    encoded = bytearray()
    while value >= 128:
        encoded.append((value & 127) | 128)
        value >>= 7
    encoded.append(value)
    return encoded


section_size = encode_leb(end - code)
function_size = encode_leb(body_size)
assert original[code - len(section_size):code] == section_size
assert original[body_start - len(function_size):body_start] == function_size
output.write_bytes(b"".join([
    original[:code-len(section_size)],
    encode_leb(end-code+len(guard)),
    original[code:body_start-len(function_size)],
    encode_leb(body_size+len(guard)),
    original[body_start:insertion],
    guard,
    original[insertion:],
]))
print("[DEBUG] Wrote one cursor-guard experiment copy", output)
