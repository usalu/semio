#!/bin/zsh
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --manifest-path 🧰️framework/Cargo.toml -p semio-framework-actor-retained-turn --lib > "$G/jobpage-t1.txt" 2>&1
a=$?
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --manifest-path Cargo.toml -p semio-framework-plugin-reactor-context -p semio-framework-plugin-host-guest-runtime --lib > "$G/jobpage-t2.txt" 2>&1
echo "$a $?" > "$G/jobpage-t1.rc"
