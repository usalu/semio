#!/bin/bash
# usage: runchk.sh <label> <wasm|native> <argsfile>
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
read -r -a A < "$3"
exec "$C/1102-tools/chk.sh" "$1" "${M:-🌎️hub/Cargo.toml}" "$2" "${A[@]}"
