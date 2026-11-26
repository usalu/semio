"""🧹 Identify stale ticket-private Cargo units with superseded dependency fingerprints."""
import argparse
import json
from pathlib import Path
import shutil
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--prune",action="store_true")
parser.add_argument("--guest",action="store_true")
parser.add_argument("--age-minutes",type=int,default=65)
args = parser.parse_args()
generated=Path(__file__).resolve().parent/"🗑️generated"
root=(generated/"wfc/wasm32-wasip2/prototype/build" if args.guest else generated/"native-mcp/debug/build")
roots=list(root.glob("*/*/fingerprint"))
available=set()
host_fingerprints=list((generated/"wfc/prototype/build").glob("*/*/fingerprint")) if args.guest else []
for directory in roots+host_fingerprints:
    for entry in directory.iterdir():
        if entry.is_file() and entry.stat().st_size==16 and not entry.suffix:
            value=entry.read_bytes()
            try: available.add(int.from_bytes(bytes.fromhex(value.decode("ascii")),"little"))
            except (ValueError,UnicodeError): pass
if args.age_minutes < 30:
    parser.error("inactive units must be at least thirty minutes old")
cutoff=time.time()-args.age_minutes*60
rows=[]
for directory in roots:
    unit=directory.parent
    name=unit.parent.name
    if args.guest and (name.startswith("semio-s-artifact-note-note") or name in {
        "semio-hub-note", "semio-hub-note-component", "semio-hub-wfc", "semio-hub-wfc-component",
    }):
        continue
    manifests=list(directory.glob("lib-*.json"))
    if len(manifests)!=1: continue
    manifest=manifests[0]
    outputs=list((unit/"out").glob("*.rlib"))
    if not outputs or max(p.stat().st_mtime for p in [unit,directory,*directory.iterdir(),*outputs])>cutoff: continue
    missing=[name for _,name,_,stamp in json.loads(manifest.read_bytes())["deps"] if name!="build_script_build" and stamp not in available]
    if not missing: continue
    size=sum(p.stat().st_size for p in outputs)
    rows.append((size,unit,missing))
print(len(rows),"obsolete units;",sum(row[0] for row in rows)//(1<<20),"MiB")
for size,unit,missing in sorted(rows,reverse=True)[:12]:
    print(size//(1<<20),unit.relative_to(root),missing[:2])
if args.prune:
    for _,unit,_ in rows:
        if max(p.stat().st_mtime for p in unit.rglob("*"))<=cutoff:
            shutil.rmtree(unit)
