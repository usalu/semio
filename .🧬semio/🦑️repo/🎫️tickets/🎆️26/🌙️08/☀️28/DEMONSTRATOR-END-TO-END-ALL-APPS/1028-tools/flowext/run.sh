#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
E="✏️s/🔌️plugins/🌊️flow/🧩️extensions"
n=$1; shift
for d in "$@"; do
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$E/$d/Cargo.toml" --target wasm32-wasip2 --keep-going --workspace > "$C/🗑️generated/1028/flowext-chk$n-$d.txt" 2>&1
  echo $? > "$C/🗑️generated/1028/flowext-chk$n-$d.rc"
done
