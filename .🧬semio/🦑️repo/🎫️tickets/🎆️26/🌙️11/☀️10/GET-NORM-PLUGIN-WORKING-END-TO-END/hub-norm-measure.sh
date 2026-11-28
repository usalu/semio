#!/bin/bash
# usage: hub-norm-measure.sh <log-name> <memory-MiB> <package>... ; one-worker prototype wasm build of the carved hub-norm crates through the capacity gate.
cd "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" || exit 1
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/hub-norm-unit-ceiling"
name=$1; mem=$2; shift 2; args=(); for p in "$@"; do args+=(-p "$p"); done
date +%s000 > "$T/$name.since"
repo capacity run --memory="$mem" --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- build --offline --locked "${args[@]}" --target wasm32-wasip2 > "$T/$name.log" 2>&1
echo "exit $?" > "$T/$name.status"
