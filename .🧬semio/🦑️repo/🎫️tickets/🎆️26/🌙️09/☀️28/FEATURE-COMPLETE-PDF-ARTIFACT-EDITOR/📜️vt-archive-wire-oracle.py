"""🔬 Independently reproduces the aggregate PDF operation's retained Pack rejection."""

import json
from pathlib import Path
import re
import sys
import google_crc32c

TICKET=Path(__file__).resolve().parent
base=Path(sys.argv[1])
fixture=json.loads((TICKET/"🔣️vt-input-cases.json").read_text(encoding="utf8"))[0]
protocol=(base/"🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio").read_text(encoding="utf8")
tag=int(re.search(r"^record insert-object tag=(\d+)$",protocol,re.MULTILINE)[1])
payload=json.dumps(fixture["payload"],ensure_ascii=False,separators=(",",":")).encode("utf8")
length=len(payload)
prefix=bytearray()
while length>=128:
    prefix.append((length&127)|128)
    length>>=7
prefix.append(length)
legacy=bytes([1,tag])+prefix+payload
assert len(legacy)>=84
stored=int.from_bytes(legacy[20:24],"little")
computed=google_crc32c.value(legacy[:20])
assert stored!=computed
result={"operation":fixture["mutation"],"wireBytes":len(legacy),"prefix":legacy[:8].hex(),"storedHeaderWord":f"{stored:08x}","crc32c":f"{computed:08x}","firstRefusalOffset":20}
out=TICKET/"🗑️generated"/"vt-archive"
out.mkdir(parents=True,exist_ok=True)
(out/"old-frame.json").write_text(json.dumps(result)+"\n",encoding="utf8")
print("[DEBUG]",json.dumps(result))
