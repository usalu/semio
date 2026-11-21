"""🧾 Check small documents in bounded batches and large ones individually, reusing completed schema and input hashes."""
from pathlib import Path
from hashlib import sha256
from shutil import which
import json
import os
import subprocess
import sys

ticket=Path(__file__).resolve().parent
root=ticket.parents[6]
catalog=next((root/"🎓️teaching").glob("*architecture/*staffage/*catalog"))
log=ticket/"🗑️generated/equipment-check/catalog-schema.jsonl"
log.parent.mkdir(parents=True,exist_ok=True)
schema=sha256((catalog.parent/"🧬️schema/🔣️.json").read_bytes()).hexdigest()
cache={v["id"]:v for line in (log.read_text().splitlines() if log.exists() else ()) if line and (v:=json.loads(line)) and v.get("schemaSha256")==schema and v.get("valid")}
bun=which("bun") or next((p for p in (root/".cache/b/t/m/bun").glob("*/*") if p.name in ("bun","bun.exe")),None)
if bun is None:raise SystemExit("No Bun runtime available")
batch=[]
used=0
batches=[]
for file in sorted(catalog.glob("*/*/*.json"),key=lambda p:p.stat().st_size):
    name=file.parent.name
    size=file.stat().st_size
    if size>3*1024*1024 and "--large" not in sys.argv:continue
    if cache.get(name,{}).get("sha256")==sha256(file.read_bytes()).hexdigest():continue
    if batch and used+size>4*1024*1024:
        batches.append(batch)
        batch=[]
        used=0
    batch.append(name)
    used+=size
if batch:batches.append(batch)
for group in batches:
    result=subprocess.run([sys.executable,str(ticket/"equipment-guard.py"),str(bun),"--smol",str(ticket/"equipment-schema.ts"),*group],capture_output=True,text=True,env={**os.environ,"EQUIPMENT_RSS_MIB":os.environ.get("EQUIPMENT_RSS_MIB","180")})
    rows=[json.loads(line) for line in result.stdout.splitlines() if line.startswith("{")]
    with log.open("a") as f:
        for row in rows:f.write(json.dumps(row,separators=(",",":"))+"\n")
    if result.returncode or len(rows)!=len(group):
        print("[DEBUG]",group,result.returncode,result.stderr,flush=True)
        raise SystemExit(1)
    print(" ".join(row["id"] for row in rows),flush=True)
