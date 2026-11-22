"""🧮 Independent oracle for the folder archive load wire: frames requests and replies from the documented layout and writes the language-neutral fixture."""
import hashlib, json, struct, sys
from pathlib import Path

def key(instance, activation, lifetime, sequence, action, magic=b"SEMIOL1I"):
    return magic + struct.pack("<IQQQ", instance, activation, lifetime, sequence) + bytes([action])

def request(k, action, body=b""):
    return key(*k, action) + body

digest = hashlib.sha256(b"semio folder archive").digest()
begin_key, live_key = (7, 0, 0, 1), (7, 3, 5, 2)
doc, schema = "document-1", "stdio.pdf.1.7"
page = bytes((i * 13 + 1) & 255 for i in range(4096))
cases = [
    ("begin", begin_key, 1, struct.pack("<I", 5_000_000) + digest + struct.pack("<HH", len(doc), len(schema)) + doc.encode() + schema.encode(),
     {"action": "begin", "extent": 5_000_000, "sha256": digest.hex(), "document": doc, "schema": schema}),
    ("page first", live_key, 2, struct.pack("<I", 0) + page, {"action": "page", "offset": 0, "bytes": page.hex()}),
    ("page short tail", live_key, 2, struct.pack("<I", 4096) + b"\x01\x02\x03", {"action": "page", "offset": 4096, "bytes": "010203"}),
    ("finish", live_key, 3, b"", {"action": "finish"}),
    ("publish", live_key, 4, b"", {"action": "publish"}),
    ("cancel", live_key, 5, b"", {"action": "cancel"}),
]
law = {"schema": "semio.folder-archive-load.wire/v1", "requests": [], "replies": []}
for name, k, tag, body, fields in cases:
    law["requests"].append({"name": name, "key": dict(zip(("instance", "activation", "lifetime", "sequence"), k)), **fields, "wireHex": request(k, tag, body).hex()})
def reply(name, k, tag, rtag, body, fields):
    law["replies"].append({"name": name, "request": name if False else None, "key": dict(zip(("instance", "activation", "lifetime", "sequence"), k)), "action": fields["action"], "offset": fields.get("offset", 0), "length": fields.get("length", 0),
                           "reply": fields["reply"], **{x: fields[x] for x in ("activation", "lifetime", "through") if x in fields}, "wireHex": (key(*k, tag, b"SEMIOL1O") + bytes([rtag]) + body).hex()})
reply("admitted", begin_key, 1, 1, struct.pack("<QQ", 3, 5), {"action": "begin", "reply": "admitted", "activation": 3, "lifetime": 5})
reply("received", live_key, 2, 2, struct.pack("<I", 4099), {"action": "page", "offset": 4096, "length": 3, "reply": "received", "through": 4099})
reply("pending", live_key, 3, 0, b"", {"action": "finish", "reply": "pending"})
reply("verified", live_key, 3, 3, b"", {"action": "finish", "reply": "verified"})
reply("submitted", live_key, 4, 4, b"", {"action": "publish", "reply": "submitted"})
reply("closed", live_key, 5, 5, b"", {"action": "cancel", "reply": "closed"})
reply("refused", live_key, 2, 6, b"", {"action": "page", "offset": 0, "length": 4096, "reply": "refused"})
for row in law["replies"]: del row["request"]
Path(sys.argv[1]).write_text(json.dumps(law, indent=1) + "\n", encoding="utf8")
print("[DEBUG] wrote", len(law["requests"]), "requests and", len(law["replies"]), "replies")
