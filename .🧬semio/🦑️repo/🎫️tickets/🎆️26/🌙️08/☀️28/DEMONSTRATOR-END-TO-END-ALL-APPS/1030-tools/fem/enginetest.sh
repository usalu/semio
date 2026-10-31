#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1030/fem-etest$1.rc"
MODE=${2:-test --no-run --no-fail-fast}
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- $MODE --manifest-path "$ROOT/✏️s/Cargo.toml" -p semio-s-fem-payload-turns -p semio-s-fem-test-support -p semio-s-fem-sparse -p semio-s-fem-sparse-ldlt -p semio-s-fem-sparse-pcg -p semio-s-fem-sparse-subspace -p semio-s-fem-mesh -p semio-s-fem-analyses-assembly --lib > "$C/🗑️generated/1030/fem-etest$1.txt" 2>&1
echo $? > "$C/🗑️generated/1030/fem-etest$1.rc"
