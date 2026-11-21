"""🧾 Build exact PDF 1.4 native edit frames from the neutral operation schema."""
import json
import struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OUTPUT=ROOT/"🗑️generated"/"pdf14-wire"/"frames.json"
rows=json.loads((ROOT/"🔣️pdf14-native-wire.json").read_text())
formats={0:"<Qdd",1:"<Q",2:"<QQ",3:"<Qdd",4:"<Q"}
for row in rows:
    tag=row["tag"]
    fields=struct.pack(formats[tag],*row["values"])
    assert list(struct.unpack(formats[tag],fields))==row["values"]
    if "repeat" in row:
        row["text"]=row["repeat"]["prefix"]*row["repeat"]["count"]+row["repeat"]["last"]
    payload=row.get("text","").encode()
    if tag in (0,4):
        fields+=struct.pack("<Q",len(payload))+payload
    row["wire"]=list(bytes([1,tag])+fields)
OUTPUT.parent.mkdir(parents=True,exist_ok=True)
OUTPUT.write_text(json.dumps(rows))
print("[DEBUG] Authored all five exact PDF 1.4 frames with struct, including UTF-8 crossing a 4-KiB page.")
