#!/usr/bin/env python3
"""Builds a verification overlay of COPIES under /private/tmp/conc-harness/ov: the staged files at their destinations, the live incremental module they import, the schema validator, the process termination module and the wire-patched catalogue; bare packages resolve through tsconfig paths to the live node_modules (read only)."""
import json,os,shutil,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
ov=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/conc-harness/ov"
shutil.rmtree(ov,ignore_errors=True);os.makedirs(ov)
manifest=json.load(open(os.path.join(here,"staged","manifest.json"),encoding="utf8"))
def put(src,rel):
    dst=os.path.join(ov,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(src,dst)
for flat,row in manifest.items(): put(os.path.join(here,"staged",flat),row["destination"])
I=names.I
for rel in (f"{I}/{names.TS}",f"{I}/{names.COMMAND}/{names.TS}",f"{I}/{names.RUNNER}/{names.TS}",f"{I}/{names.OBSERVERS}/{names.TS}",f"{I}/{names.SCHEMA}/{names.JSON}",f"{I}/{names.SCENARIOS}/{names.JSON}"): put(os.path.join(names.R,rel),rel)
V=f"{names.FRAMEWORK}/{names.MODULES}/{names.SCHEMA}/{names.VALIDATOR}"
for rel in (f"{V}/{names.TS}",f"{V}/{names.PACKAGES}/{names.TYPESCRIPT}/{names.TS}",f"{names.FRAMEWORK}/{names.MODULES}/{names.PROCESS}/{names.TERMINATION}/{names.TS}"): put(os.path.join(names.R,rel),rel)
import wire
for rel,fn in wire.EDITS:
    if rel.endswith(names.CATALOG):
        dst=os.path.join(ov,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);open(dst,"w",encoding="utf8").write(fn(open(os.path.join(names.R,rel),encoding="utf8").read()))
nm=os.path.join(names.R,"node_modules")
json.dump({"compilerOptions":{"module":"esnext","moduleResolution":"bundler","target":"esnext","strict":True,"baseUrl":".","paths":{"ajv":[nm+"/ajv"],"ajv/*":[nm+"/ajv/*"]},"types":["bun"]}},open(os.path.join(ov,"tsconfig.json"),"w"))
open(os.path.join(ov,"bunfig.toml"),"w").write("")
print(ov)
