"""🧾 Verify the emitted WIT metadata, full export surface and binary function limit."""
import hashlib
from pathlib import Path
import runpy
import sys

root=Path(__file__).resolve().parent
inventory=runpy.run_path(str(root/"compact-requester-core.py"))["inventory"]
source,candidate,destination=map(Path,sys.argv[1:4])
old_count,old_wit,old_exports=inventory(source)
new_count,new_wit,new_exports=inventory(candidate)
if old_wit!=new_wit or old_exports!=new_exports or new_count>=1_000_000:
    raise ValueError(f"Rejected core with {new_count} functions, {len(new_wit)} WIT names and {len(new_exports)} exports")
digest=hashlib.sha256()
with candidate.open("rb") as stream:
    for buf in iter(lambda:stream.read(1024*1024),b""):
        digest.update(buf)
candidate.replace(destination)
print(f"[DEBUG] Accepted {old_count} -> {new_count} functions, {len(new_wit)} exact WIT section names, {len(new_exports)} exact exports, {destination.stat().st_size} bytes, sha256 {digest.hexdigest()}",flush=True)
