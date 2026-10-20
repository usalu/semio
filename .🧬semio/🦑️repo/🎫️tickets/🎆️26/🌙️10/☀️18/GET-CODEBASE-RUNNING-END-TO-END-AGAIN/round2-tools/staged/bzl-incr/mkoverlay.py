#!/usr/bin/env python3
"""Builds a verification overlay of COPIES under /private/tmp/bzlincr/ov: the staged files at their destinations plus the schema validator they import; bare packages resolve through tsconfig paths to the live node_modules (read only)."""
import json,os,shutil,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
ov=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/bzlincr/ov"
shutil.rmtree(ov,ignore_errors=True);os.makedirs(ov)
manifest=json.load(open(os.path.join(here,"staged","manifest.json"),encoding="utf8"))
def put(src,rel):
    dst=os.path.join(ov,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(src,dst)
for flat,row in manifest.items(): put(os.path.join(here,"staged",flat),row["destination"])
V=f"{names.FRAMEWORK}/{names.MODULES}/{names.SCHEMA}/{names.VALIDATOR}"
for rel in (f"{V}/{names.TS}",f"{V}/{names.PACKAGES}/{names.TYPESCRIPT}/{names.TS}"): put(os.path.join(names.R,rel),rel)
import wire
for rel,fn in wire.EDITS:
    if rel.endswith(f"{names.n(0x1F9ED,'commands')}/{names.TS}") or rel.endswith(names.CATALOG) or rel.endswith(f"{names.SCHEMA}/{names.JSON}") and names.OPERATIONS in rel:
        dst=os.path.join(ov,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);open(dst,"w",encoding="utf8").write(fn(open(os.path.join(names.R,rel),encoding="utf8").read()))
CMD=f"{names.B}/{names.TOOLS}/{names.BAZEL}/{names.n(0x1F9ED,'commands')}"
put(os.path.join(names.R,CMD,names.SCHEMA,names.JSON),f"{CMD}/{names.SCHEMA}/{names.JSON}")
nm=os.path.join(names.R,"node_modules")
paths={"ajv":[nm+"/ajv"],"ajv/*":[nm+"/ajv/*"]}
json.dump({"compilerOptions":{"module":"esnext","moduleResolution":"bundler","target":"esnext","strict":True,"baseUrl":".","paths":paths,"types":["bun"]}},open(os.path.join(ov,"tsconfig.json"),"w"))
open(os.path.join(ov,"bunfig.toml"),"w").write("")
print(ov)
