#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/pubtoken-$1-p3d.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml" --workspace --lib --keep-going > "$C/🗑️generated/1029/pubtoken-$1-p3d.txt" 2>&1
echo $? > "$C/🗑️generated/1029/pubtoken-$1-p3d.rc"
bash "$C/1029-tools/dispatch/chk.sh" 9 3d
