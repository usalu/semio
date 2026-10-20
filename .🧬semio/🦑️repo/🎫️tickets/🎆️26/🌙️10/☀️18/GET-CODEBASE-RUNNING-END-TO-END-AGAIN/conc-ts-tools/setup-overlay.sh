#!/bin/sh
# usage: setup-overlay.sh <name> <family tsconfig, repo-relative>   -> /private/tmp/conc-ts/ov-<name> (+ mono.json, listed.txt, engine copy)
# one tsc (--listFilesOnly) at niceness 10; the overlay is a frozen copy of the program's workspace files.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"; R=/Users/ueli/Documents/semio; N="$1"; F="$2"; T=/private/tmp/conc-ts
mkdir -p $T/mono
cat > $T/mono/base-$N.json <<J
{"extends":"$R/$F","compilerOptions":{"noEmit":true,"incremental":false,"tsBuildInfoFile":null,"typeRoots":["$R/node_modules/@types"]}}
J
(cd $R && nice -n 10 node node_modules/typescript/bin/tsc -p $T/mono/base-$N.json --listFilesOnly > $T/mono/listed-$N.txt 2>/dev/null) || true
python3 "$HERE/mkoverlay.py" $T/mono/listed-$N.txt "$F" $T/ov-$N
cat > $T/ov-$N/mono.json <<J
{"extends":"./$F","compilerOptions":{"noEmit":true,"incremental":false,"tsBuildInfoFile":null}}
J
python3 "$HERE/prep-engine.py" >/dev/null
ln -sfn $R/node_modules $T/engine/node_modules
