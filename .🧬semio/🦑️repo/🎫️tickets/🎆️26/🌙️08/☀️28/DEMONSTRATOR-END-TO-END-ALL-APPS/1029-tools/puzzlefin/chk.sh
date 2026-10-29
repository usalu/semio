#!/bin/bash
# usage: chk.sh <N> <3d|2d|5d>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
case "$2" in 3d) W="🧊️3d";; 2d) W="◻️2d";; 5d) W="🖐️5d";; esac
rm -f "$C/🗑️generated/1029/puzzlefin-chk$1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/$W/Cargo.toml" --target wasm32-wasip2 --keep-going --workspace > "$C/🗑️generated/1029/puzzlefin-chk$1.txt" 2>&1
echo $? > "$C/🗑️generated/1029/puzzlefin-chk$1.rc"
