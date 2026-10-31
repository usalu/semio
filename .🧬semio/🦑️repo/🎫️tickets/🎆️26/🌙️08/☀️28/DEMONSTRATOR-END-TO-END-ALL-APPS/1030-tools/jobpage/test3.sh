#!/bin/zsh
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml -p semio-framework-plugin-guest-contract --features original-actor-context-testing --lib original_ > "$G/jobpage-t3.txt" 2>&1
echo $? > "$G/jobpage-t3.rc"
