#!/bin/bash
# usage: wchk.sh <W-label> <list-file>  — wasm --lib check through the slot
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1102"; tag=$1; list=$2
rm -f "$G/testport-$tag.rc"
SEMIO_LANGUAGE=en "$C/1102-tools/slot.sh" $tag repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml --target wasm32-wasip2 --lib --keep-going $(sed 's/^/-p /' "$C/1102-tools/testport/$list" | tr '\n' ' ') > "$G/testport-$tag.txt" 2>&1
echo $? > "$G/testport-$tag.rc"
