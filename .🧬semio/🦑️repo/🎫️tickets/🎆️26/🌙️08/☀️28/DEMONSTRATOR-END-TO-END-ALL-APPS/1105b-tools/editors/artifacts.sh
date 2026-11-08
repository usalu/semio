#!/bin/bash
# usage: artifacts.sh <label-prefix>  sequential native test-target checks of the stdio/raster editor ports, each in its own artifact workspace
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
A="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
K="$C/1105b-tools/editors/chk.sh"
while [ ! -f "$C/🗑️generated/1105b/W-editors-109.rc" ]; do sleep 20; done
"$K" "$1-zip" "$A/🎒️zip/Cargo.toml" native -p semio-s-artifact-stdio-zip-editor --tests
"$K" "$1-semio" "$A/🧿️semio/Cargo.toml" native -p semio-s-artifact-stdio-semio-mesh-editor -p semio-s-artifact-stdio-semio-brep-io --tests
"$K" "$1-docx" "$A/📜️docx/Cargo.toml" native -p semio-s-artifact-stdio-docx-base --tests
"$K" "$1-raster" "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml" native -p semio-s-artifact-raster-raster-editor --tests
