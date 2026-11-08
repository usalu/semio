#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1105b/reviewfix2-all2.rc"
"$C/1105b-tools/runtime/tst.sh" reviewfix2-105 "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust/Cargo.toml" native -p semio-s-fem-analyses-model
echo done > "$C/🗑️generated/1105b/reviewfix2-all2.rc"
