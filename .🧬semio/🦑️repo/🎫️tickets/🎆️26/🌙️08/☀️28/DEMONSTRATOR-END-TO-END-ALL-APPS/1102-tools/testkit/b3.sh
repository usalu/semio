#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1102"; S="$C/1102-tools/slot.sh"
rm -f $G/b3-*.rc
SEMIO_LANGUAGE=en $S W-b3-wasm repo cargo --store=shared --configuration=prototype -- check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 --lib --keep-going -p semio-framework-plugin-testing-laws > $G/b3-wasm.txt 2>&1; echo $? > $G/b3-wasm.rc
SEMIO_LANGUAGE=en $S W-b3-law repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml --keep-going -p semio-framework-plugin --lib -- registered_construction_publishes > $G/b3-law.txt 2>&1; echo $? > $G/b3-law.rc
SEMIO_LANGUAGE=en $S W-b3-energy repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml --no-fail-fast -p semio-s-artifact-energy-model-editor-mounted-tests --lib > $G/b3-energy.txt 2>&1; echo $? > $G/b3-energy.rc
