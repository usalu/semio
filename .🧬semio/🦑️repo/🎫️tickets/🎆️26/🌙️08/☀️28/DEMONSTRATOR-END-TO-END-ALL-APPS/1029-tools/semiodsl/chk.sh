#!/bin/bash
# usage: chk.sh <n> [--target wasm32-wasip2] -- crate...
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
TARGET=""
if [ "$1" = "--target" ]; then TARGET="--target $2"; shift 2; fi
MODE=${MODE:-check}
M='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📦️packages/🦀️rust/Cargo.toml'
P=""
for c in "$@"; do if [ "$c" = "TOP" ]; then P="$P -p semio-s-artifact-stdio-semio"; else P="$P -p semio-s-artifact-stdio-semio-$c"; fi; done
rm -f "$C/🗑️generated/1029/semiodsl-chk$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- $MODE --manifest-path "$M" $TARGET --keep-going $P > "$C/🗑️generated/1029/semiodsl-chk$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/semiodsl-chk$N.rc"
