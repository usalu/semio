#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1030"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml -p semio-framework-os-store-one-item-batch --test mutation_apply cold_replay > "$G/policy-law.txt" 2>&1
echo $? > "$G/policy-law.rc"
