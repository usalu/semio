#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1029/dispatch-native$1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📦️packages/🦀️rust/Cargo.toml" --lib --tests --keep-going > "$C/🗑️generated/1029/dispatch-native$1.txt" 2>&1
echo $? > "$C/🗑️generated/1029/dispatch-native$1.rc"
