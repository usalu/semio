#!/bin/bash
# usage: test2d.sh <N> — native lib tests of the gen2d codec-owner crates
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
rm -f "$C/🗑️generated/1030/gen-test$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml" --no-fail-fast --lib -p semio-s-artifact-procedural-2d-model -p semio-s-artifact-procedural-2d-mutations -p semio-s-artifact-procedural-2d-io-text -p semio-s-artifact-procedural-2d-io-binary -p semio-s-artifact-procedural-2d-io-sqlite > "$C/🗑️generated/1030/gen-test$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/gen-test$N.rc"
