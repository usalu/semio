#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
bash $C/1105b-tools/hostcheck/tst.sh W-hostcheck-66 "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust/Cargo.toml" --test viewport_receiving
P3="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d"
P5="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d"
bash $C/1105b-tools/chk.sh W-hostcheck-67 "$P3/Cargo.toml" native --tests -p $(/usr/bin/grep -m1 '^name' "$P3/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml" | cut -d'"' -f2)
bash $C/1105b-tools/chk.sh W-hostcheck-68 "$P5/Cargo.toml" native --tests -p $(/usr/bin/grep -m1 '^name' "$P5/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml" | cut -d'"' -f2)
