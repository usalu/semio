#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1030"
M='🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📐️model/📦️packages/🦀️rust/Cargo.toml'
rm -f "$O"/stdiofix-drm$1.*
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$M" --target wasm32-wasip2 --lib --keep-going -p semio-framework-dsl-record-model > "$O/stdiofix-drm$1.txt" 2>&1
echo $? > "$O/stdiofix-drm$1.rc"
