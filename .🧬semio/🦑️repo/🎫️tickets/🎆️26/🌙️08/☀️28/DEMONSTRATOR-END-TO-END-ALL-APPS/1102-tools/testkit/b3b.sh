#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1102"; S="$C/1102-tools/slot.sh"
while [ ! -f $G/b3-energy.rc ]; do sleep 20; done
rm -f $G/b3b-*.rc
SEMIO_LANGUAGE=en $S W-b3b-law repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml -p semio-framework-plugin-tests-04-2 -- registered_construction_publishes > $G/b3b-law.txt 2>&1; echo $? > $G/b3b-law.rc
SEMIO_LANGUAGE=en $S W-b3b-wasm repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml --target wasm32-wasip2 --lib --keep-going -p semio-framework-plugin-testing-laws > $G/b3b-wasm.txt 2>&1; echo $? > $G/b3b-wasm.rc
