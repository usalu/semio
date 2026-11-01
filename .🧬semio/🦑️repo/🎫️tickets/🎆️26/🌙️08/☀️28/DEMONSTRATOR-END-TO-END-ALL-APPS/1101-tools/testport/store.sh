#!/bin/bash
# usage: store.sh <tag> [extra args]  — native test of all store crates
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
tag=$1; shift
exec "$C/1101-tools/testport/t.sh" "$tag" Cargo.toml $(sed 's/^/-p /' "$C/1101-tools/testport/${LIST:-store.txt}" | tr '\n' ' ') "$@"
