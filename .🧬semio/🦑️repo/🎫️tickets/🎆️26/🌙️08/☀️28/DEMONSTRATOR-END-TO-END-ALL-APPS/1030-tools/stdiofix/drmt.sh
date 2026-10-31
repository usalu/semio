#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1030"
M='🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📐️model/📦️packages/🦀️rust/Cargo.toml'
rm -f "$O"/stdiofix-drmt$1.*
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-framework-dsl-record-model --lib --no-fail-fast > "$O/stdiofix-drmt$1.txt" 2>&1
echo $? > "$O/stdiofix-drmt$1.rc"
