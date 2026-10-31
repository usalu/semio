#!/bin/bash
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
for spec in "pz3d|/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" "gen3d|/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d" "cad|/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad" "cur|/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation"; do
  "$C/1029-tools/rta/chk.sh" "enum-${spec%%|*}" "${spec#*|}" --workspace --lib --tests --keep-going
done
echo done > "$C/🗑️generated/1029/rta-enum.done"
