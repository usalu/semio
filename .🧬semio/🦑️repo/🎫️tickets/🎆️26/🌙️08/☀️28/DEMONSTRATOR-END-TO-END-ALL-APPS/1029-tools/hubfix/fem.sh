#!/bin/bash
# usage: fem.sh N native|wasm|test crate...
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; MODE=$2; shift 2
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/hubfix-fem$N.rc"
P=(); for c in "$@"; do if [ "$c" = "@workspace" ]; then P+=(--workspace); else P+=(-p "$c"); fi; done
case $MODE in
 native) ARGS=(check --keep-going "${P[@]}");;
 wasm) ARGS=(check --target wasm32-wasip2 --keep-going "${P[@]}");;
 test) ARGS=(test --no-run --no-fail-fast "${P[@]}");;
 run) ARGS=(test --no-fail-fast "${P[@]}" ${TEST_FILTER:+-- $TEST_FILTER --nocapture --test-threads=1});;
esac
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "${ARGS[0]}" --manifest-path "$ROOT/${FEM_WS:-✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d}/Cargo.toml" "${ARGS[@]:1}" > "$C/🗑️generated/1029/hubfix-fem$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/hubfix-fem$N.rc"
