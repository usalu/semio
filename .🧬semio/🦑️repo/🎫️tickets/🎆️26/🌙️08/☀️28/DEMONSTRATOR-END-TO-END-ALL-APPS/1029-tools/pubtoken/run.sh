#!/bin/bash
# usage: run.sh <batch> ; native --lib check of the four hosts, sequential, one .rc per workspace
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
B=$1
cd "$ROOT" || exit 1
for pair in "p3d:✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" "g2d:✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d" "g3d:✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d" "ras:✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster"; do
  N=${pair%%:*}; WS=${pair#*:}
  rm -f "$C/🗑️generated/1029/pubtoken-$B-$N.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$WS/Cargo.toml" --workspace --lib --keep-going > "$C/🗑️generated/1029/pubtoken-$B-$N.txt" 2>&1
  echo $? > "$C/🗑️generated/1029/pubtoken-$B-$N.rc"
done
echo done > "$C/🗑️generated/1029/pubtoken-$B-all.rc"
