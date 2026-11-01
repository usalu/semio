#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
M="${2:-🌎️hub/Cargo.toml}"
O="$C/🗑️generated/1101/$1"; rm -f "$O.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" --no-fail-fast -p semio-s-artifact-stdio-contract-media-export -p semio-s-artifact-stdio-contract-editing-patch -p semio-s-artifact-stdio-semio-base -p semio-s-artifact-stdio-semio -p semio-s-artifact-stdio-step-geometry -p semio-s-artifact-stdio-deflate-examples -p semio-s-artifact-stdio-deflate-zlib -p semio-s-artifact-stdio-gltf-model-snapshot-root --lib > "$O.txt" 2>&1
echo $? > "$O.rc"
