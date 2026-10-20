#!/bin/bash
# usage: overlay.sh [--rebuild]   builds the scratch overlay /private/tmp/bzlcov-ov (a COPY of the Bazel adapter closure and tests; the absolute Source path limit of 256 units forbids a root inside the ticket tree) with the staged set applied
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/bzl-coverage"
OV=/private/tmp/bzlcov-ov
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build/🔧️tools/🏗️bazel"
if [ "$1" = "--rebuild" ]; then rm -rf "$OV"; fi
mkdir -p "$OV" "$G"
rm -rf /private/tmp/bzlcov-meta; mkdir -p /private/tmp/bzlcov-meta
bun build --target=bun --packages=external --metafile="$G/meta.json" --outdir=/private/tmp/bzlcov-meta "$B/📜️script.ts" "$B/🧪️tests/🟦️.ts" "$L/🏃️process/🔌️providers/🟦️.ts" "$B/🧪️tests/📥️actions/🟦️.ts" "$B/🧪️tests/📥️sources/🟦️.ts" "$B/🧪️tests/📥️workspace/🟦️.ts" "$B/🧪️tests/📥️fingerprint/🟦️.ts" > /dev/null 2>&1 || { echo "metafile build failed"; exit 3; }
python3 -I - "$G/meta.json" "$G/overlay-files.txt" <<'PY'
import json,sys,os,subprocess
meta=json.load(open(sys.argv[1]))
files=set(meta["inputs"])
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
for extra in ["bunfig.toml","package.json","bun.lock","tsconfig.json",".bazelrc",".bazelversion","MODULE.bazel"]:
    if os.path.exists(extra): files.add(extra)
roots=[L,"🧰️framework/🔨️modules/🏃️process","🧰️framework/🔨️modules/🧬️schema"]
for pkg in ["nx-bootstrap-tools"]:
    target=os.readlink("node_modules/@semio-tech/"+pkg)
    roots.append(os.path.relpath(target,os.getcwd()))
out=subprocess.run(["git","ls-files","-z","--"]+roots,capture_output=True).stdout.decode().split("\0")
files.update(f for f in out if f)
files={f for f in files if os.path.isfile(f)}
open(sys.argv[2],"w").write("\n".join(sorted(files))+"\n")
print(len(files),"files")
PY
rsync -aL --files-from="$G/overlay-files.txt" ./ "$OV/" || exit 4
mkdir -p "$OV/node_modules/@semio-tech"
for pkg in process schema-validator-ts nx-bootstrap-tools; do
  target="$(readlink node_modules/@semio-tech/$pkg)"; relative="${target#/Users/ueli/Documents/semio/}"
  ln -sfn "$OV/$relative" "$OV/node_modules/@semio-tech/$pkg"
done
python3 -I "$C/bzl-coverage-tools/nodemods.py" "$OV" ajv typescript minimatch @iarna/toml
[ -d "$OV/.git" ] || git -C "$OV" init -q
python3 -I "$C/bzl-coverage-tools/apply.py" apply --root "$OV" --overwrite
