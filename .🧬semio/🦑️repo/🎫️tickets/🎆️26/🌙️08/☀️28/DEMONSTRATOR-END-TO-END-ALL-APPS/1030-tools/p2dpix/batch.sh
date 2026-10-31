#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"; D="$C/1030-tools/p2dpix"
N=$1
rm -f "$G/p2dpix-$N-all.rc"
"$D/chk.sh" "$N-fw" check --manifest-path "🧰️framework/Cargo.toml" --keep-going -p semio-framework-pixels -p semio-framework-surface-paint --lib --tests
"$D/chk.sh" "$N-wasm" check --manifest-path "🧰️framework/Cargo.toml" --target wasm32-unknown-unknown --keep-going -p semio-framework-pixels -p semio-framework-surface-paint --lib
"$D/chk.sh" "$N-ras" check --manifest-path "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml" --keep-going -p semio-s-artifact-raster-raster-io -p semio-s-artifact-raster-raster-editor -p semio-s-artifact-raster-raster-editor-commands --lib --tests
echo done > "$G/p2dpix-$N-all.rc"
