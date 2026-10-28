"""↩️ Independent protobuf varints and BLAKE3 verify portable Revert/Reinstate payloads."""
import json
from pathlib import Path
import blake3
from google.protobuf.internal.encoder import _VarintBytes
from google.protobuf.internal.decoder import _DecodeVarint
class ProtobufVarints:
    @staticmethod
    def encode(value):
        return _VarintBytes(value)
    @staticmethod
    def decode(data,offset):
        return _DecodeVarint(data,offset)
folder=Path(__file__).resolve().parents[7]/"🧰️framework/🔨️modules/📡️replication/🔗️causal/🧪️tests/🧫️fixtures/🧫️history-transition"
f=json.loads((folder/"🔣️.json").read_text())
checked=0
for row in f["cases"]:
    expected=row.get("expect",{}).get("transition",{})
    if row["expect"]["outcome"]!="accepted" or expected.get("kind") not in ("revert","reinstate"):
        continue
    checked+=1
    tag=0 if expected["kind"]=="revert" else 1
    data=ProtobufVarints.encode(tag)+ProtobufVarints.encode(len(expected["mutationIds"]))
    for identity in expected["mutationIds"]:
        text=identity.encode()
        data+=ProtobufVarints.encode(len(text))+text
    assert data.hex()==row["payloadHex"]
    decoded_tag,offset=ProtobufVarints.decode(data,0)
    count,offset=ProtobufVarints.decode(data,offset)
    decoded=[]
    for _ in range(count):
        length,offset=ProtobufVarints.decode(data,offset)
        decoded.append(data[offset:offset+length].decode())
        offset+=length
    assert decoded_tag==tag and decoded==expected["mutationIds"] and offset==len(data)
    clock=f["idClock"]
    preimage=b"".join(ProtobufVarints.encode(clock[key]) for key in ("actor","physicalMs","logical"))+ProtobufVarints.encode(len(data))+data
    assert "transition-"+blake3.blake3(preimage).hexdigest()[:16]==row["expect"]["transitionId"]
assert checked>=3
print("[DEBUG] Revert/Reinstate corpus matches independent protobuf varints and BLAKE3 event identities")

row=next(row for row in f["cases"] if row["id"]=="repin")
expected=row["expect"]["transition"]
def text(value):
    encoded=value.encode()
    return ProtobufVarints.encode(len(encoded))+encoded
data=ProtobufVarints.encode(5)+text(expected["checkpointId"])+text(expected["pinnedCheckpointId"])+ProtobufVarints.encode(len(expected["pins"]))
for pin in expected["pins"]:
    data+=text(pin["childUri"])+text(pin["checkpointId"])
assert data.hex()==row["payloadHex"]
preimage=b"".join(ProtobufVarints.encode(f["idClock"][key]) for key in ("actor","physicalMs","logical"))+ProtobufVarints.encode(len(data))+data
assert "transition-"+blake3.blake3(preimage).hexdigest()[:16]==row["expect"]["transitionId"]
print("[DEBUG] Repin fixture matches independent protobuf framing and BLAKE3 event identity")
