#!/bin/bash
# usage: assemble-nx.sh <workspace dir>  - builds <ws>/node_modules from the pristine nx@23.2.0 and its exactly pinned dependencies in the local bun cache (no network)
set -e
W="$1"; K="$HOME/.bun/install/cache"; rm -rf "$W/node_modules"; mkdir -p "$W/node_modules/@nx"
cp -R "$K/nx@23.2.0@@@1" "$W/node_modules/nx"
cp -R "$K/@nx/nx-darwin-arm64@23.2.0@@@1" "$W/node_modules/@nx/nx-darwin-arm64"
python3 - "$W" "$K" <<'P'
import json,os,shutil,sys
w,k=sys.argv[1:3]
deps=json.load(open(f"{k}/nx@23.2.0@@@1/package.json"))["dependencies"]
missing=[]
for name,ver in deps.items():
    src=f"{k}/{name}@{ver}@@@1"
    dst=f"{w}/node_modules/{name}"
    if not os.path.isdir(src): missing.append(f"{name}@{ver}"); continue
    os.makedirs(os.path.dirname(dst),exist_ok=True)
    shutil.copytree(src,dst)
print("missing:",missing)
P
