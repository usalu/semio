#!/bin/bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/Cargo.toml" --target wasm32-wasip2 --keep-going -p semio-framework-mesh-engine > $G/gis-mesh-wasm.txt 2>&1; echo $? > $G/gis-mesh-wasm.rc
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/Cargo.toml" --keep-going -p semio-framework-mesh-engine > $G/gis-mesh-native.txt 2>&1; echo $? > $G/gis-mesh-native.rc
