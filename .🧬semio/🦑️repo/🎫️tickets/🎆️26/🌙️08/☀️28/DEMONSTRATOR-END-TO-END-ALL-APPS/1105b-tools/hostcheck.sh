#!/bin/bash
# usage: hostcheck.sh <label>   one keep-going native check of every semio-plugin-host-* package plus runtime-ops and instance-app
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
A=()
for n in "" -shard -imports -wake -guest-runtime -wit -engine -post-turn-relay -routers -process-transport -effects -operation -guest-laws -codec -source-json -test-component -shard-executor -shard-laws -test-component-refusal; do A+=(-p "semio-plugin-host$n"); done
A+=(-p semio-plugin-runtime-ops -p semio-plugin-instance-app)
exec "$C/1105b-tools/chk.sh" "$1" Cargo.toml native --tests "${A[@]}"
