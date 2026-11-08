#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"
while [ ! -f "$G/hostcheck-1107a.done" ]; do sleep 20; done
P3="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d"
P5="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d"
n3=$(/usr/bin/grep -m1 '^name' "$P3/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml" | cut -d'"' -f2)
n5=$(/usr/bin/grep -m1 '^name' "$P5/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml" | cut -d'"' -f2)
bash "$C/1105b-tools/hostcheck/tst.sh" hostcheck-103 "$P3/Cargo.toml" -p "$n3" --lib
bash "$C/1105b-tools/hostcheck/tst.sh" hostcheck-104 "$P5/Cargo.toml" -p "$n5" --lib
bash "$C/1105b-tools/chk.sh" hostcheck-105 "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.toml" native --tests -p semio-s-artifact-stdio-gltf-editor
bash "$C/1105b-tools/hostcheck.sh" hostcheck-106
echo done > "$G/hostcheck-1107b.done"
