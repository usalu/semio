#!/bin/bash
# usage: native.sh <N>   (native lib check of the touched crates)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1029/resume-native$1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📦️packages/🦀️rust/Cargo.toml" --lib --keep-going -p semio-framework-plugin-instance-app -p semio-framework-plugin-instance-history -p semio-framework-plugin-instance-dispatch -p semio-framework-plugin-instance-commands -p semio-framework-plugin-runtime-ops -p semio-framework-plugin-reactor-turn -p semio-framework-plugin-reactor-app > "$C/🗑️generated/1029/resume-native$1.txt" 2>&1
echo $? > "$C/🗑️generated/1029/resume-native$1.rc"
