#!/bin/bash
# usage: port.sh <tag>  wasm --lib check of the runtime lane crates (pkgs from /tmp/claude-501/pkgs.txt copy)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
args=(); for n in $(cat "$C/1030-tools/runtime/pkgs.txt"); do args+=(-p "$n"); done
exec "$C/1030-tools/runtime/ck.sh" "$1" "${args[@]}" --target wasm32-wasip2 --lib
