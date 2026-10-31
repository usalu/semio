#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
rm -f "$C/🗑️generated/1030/perf-chk$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" --keep-going -p semio-s-artifact-flow-flow-editor --lib --tests > "$C/🗑️generated/1030/perf-chk$N-flow.txt" 2>&1
echo $? > "$C/🗑️generated/1030/perf-chk$N-flow.rc"
echo done > "$C/🗑️generated/1030/perf-chk$N.rc"
