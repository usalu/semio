#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
A='✏️s/🔌️plugins/🗄️stdio'
t(){ L=$1; M=$2; shift 2; O="$C/🗑️generated/1101/$L"; rm -f "$O.rc"; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" --no-fail-fast "$@" > "$O.txt" 2>&1; echo $? > "$O.rc"; }
t dt-contract "$A/Cargo.toml" -p semio-s-artifact-stdio-contract-media-export -p semio-s-artifact-stdio-contract-editing-patch --lib
t dt-semio "$A/🗿️artifacts/🧿️semio/Cargo.toml" -p semio-s-artifact-stdio-semio-base -p semio-s-artifact-stdio-semio --lib
t dt-deflate "$A/🗿️artifacts/🗜️deflate/Cargo.toml" -p semio-s-artifact-stdio-deflate-examples -p semio-s-artifact-stdio-deflate-zlib --lib
t dt-step "$A/🗿️artifacts/📐️step/Cargo.toml" -p semio-s-artifact-stdio-step-geometry --lib
t dt-gltf "$A/🗿️artifacts/🧊️gltf/Cargo.toml" -p semio-s-artifact-stdio-gltf-model-snapshot-root --lib
echo done > "$C/🗑️generated/1101/dt-all.rc"
