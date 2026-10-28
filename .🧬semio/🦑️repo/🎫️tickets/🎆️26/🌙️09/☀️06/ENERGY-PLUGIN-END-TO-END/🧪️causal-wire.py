from pathlib import Path
import json
from google.protobuf.internal.encoder import _VarintBytes
from jsonschema import Draft202012Validator
root=Path.cwd()/"🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/🔗️causal/🛫️native"
f=json.loads((root/"🔣️.json").read_text());Draft202012Validator(json.loads((root/"🧬️.json").read_text())).validate(f)
def blob(value):return _VarintBytes(len(value))+value
def text(value):return blob(value.encode())
def encode(rows):
 out=_VarintBytes(len(rows))
 for row in rows:
  for name in ["mutationId","documentId","actor"]:out+=text(row[name])
  out+=_VarintBytes(len(row["dependencies"]))
  for value in row["dependencies"]:out+=text(value)
  out+=_VarintBytes(int(row["observed"] is not None))
  if row["observed"] is not None:out+=text(row["observed"])
  out+=_VarintBytes(len(row["target"]))
  for value in row["target"]:out+=text(value)
  for field in ["payloadHex","inverseHex"]:out+=text(row["schema"])+blob(bytes.fromhex(row[field]))
  for value in row["timestamp"]:out+=_VarintBytes(int(value))
  out+=_VarintBytes(int(row["transaction"] is not None)|(int(row["verb"] is not None)<<1)|(int(row["line"] is not None)<<2))
  if row["transaction"] is not None:
   for field in ["id","tool"]:out+=text(row["transaction"][field])
  for field in ["verb","line"]:
   if row[field] is not None:out+=text(row[field])
 return out
out=encode(f["rows"])
expected=(root/"📬️.hex").read_text().strip()
assert out.hex()==expected
print("PASS independent Protocol Buffers varints + jsonschema causal batch",len(out),"bytes")

import copy,hashlib
rows=copy.deepcopy(f["rows"]);source=f["source"];rows[0]["payloadHex"]=bytes(source["pattern"][i%len(source["pattern"])] for i in range(source["payloadBytes"])).hex();wide=encode(rows)
assert len(wide)==source["wireBytes"]
assert hashlib.sha256(wide).hexdigest()==source["wireSha256"]
print("PASS independent paged-source wire",len(wide),"bytes",source["wireSha256"])
