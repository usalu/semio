"""🧪 Builds independent fixed-page causal transport vectors for the actual TypeScript codec."""

import hashlib
import json
from pathlib import Path
import re
import runpy
import struct
import subprocess

TICKET = Path(__file__).resolve().parent
CODE = runpy.run_path(str(TICKET / "raw-spr-transport-stage.py"))
CODE["stage"]()
POST = json.loads(CODE["OUTPUT"].read_text(encoding="utf8"))["candidate"]
OUTPUT = TICKET / "🗑️generated/raw-spr-transport"
OUTPUT.mkdir(parents=True, exist_ok=True)
(OUTPUT / "atomic.ts").write_text(POST[str(CODE["ASSEMBLY"])], encoding="utf8")
source = POST[str(CODE["BATCH"])].replace("./🧩️atomic-group/🟦️.ts", "./atomic.ts")
(OUTPUT / "causal.ts").write_text(source, encoding="utf8")
message_path = (CODE["ROOT"] / CODE["MESSAGE"]).resolve()
def resolve_import(match):
    specifier = match[1]
    if specifier.endswith("/🧩️atomic-group/🟦️.ts"):
        return 'from "./atomic.ts"'
    return 'from "' + (message_path.parent / specifier).resolve().as_posix() + '"'

message = re.sub(r'from "(\.\.[^"]+)"', resolve_import, POST[str(CODE["MESSAGE"])])
(OUTPUT / "message.ts").write_text(message, encoding="utf8")


def integer(number):
    digits = []
    while number > 127:
        digits.append(128 | (number & 127))
        number >>= 7
    return bytes([*digits, number])


def field(data):
    return integer(len(data)) + data


def envelope(index, part):
    return b"".join((
        field(f"registered-png/{index}".encode()), field(b"fixture-document"), field(b"fixture-actor"),
        integer(int(index != 0)),
        field(f"registered-png/{index - 1}".encode()) if index else b"",
        b"\x00\x00",
        field(b"pdf/1.7"), field(part), field(b"pdf/1.7"), field(b"undo"),
        integer(3), integer(18446744073709551615), integer(index),
        b"\x01", field(b"transaction-82"), field(b"s.stdio.pdf#original-media"),
    ))


png = (TICKET / "🗑️generated/large-picked/large-picked.png").read_bytes()
rows = [envelope(index, png[offset:offset + (1 << 15)]) for index, offset in enumerate(range(0, len(png), 1 << 15))]
prepared = bytes.fromhex("80004700") + field(b"edit-png-82") + integer(len(rows)) + b"".join(rows)
sha = hashlib.sha256(prepared).digest()
limit = 1 << 16
parts = [struct.pack("<4s32sIH", bytes.fromhex("80004701"), sha, len(prepared), index) + prepared[offset:offset + limit]
         for index, offset in enumerate(range(0, len(prepared), limit))]
for index, part in enumerate(parts):
    (OUTPUT / f"{index}.part").write_bytes(part)
(OUTPUT / "prepared").write_bytes(prepared)
(OUTPUT / "vector.json").write_text(json.dumps({
    "parts": len(parts), "extent": len(prepared), "source": len(png),
    "sha": hashlib.sha256(png).hexdigest(),
}) + "\n", encoding="utf8")

subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", str(TICKET / "raw-spr-transport-law.ts")], check=True)
