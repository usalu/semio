#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
tag=$1; shift
args=()
for p in "$@"; do args+=(-p "$p"); done
rm -f "$C/🗑️generated/1030/testsplit-$tag.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --keep-going --tests "${args[@]}" > "$C/🗑️generated/1030/testsplit-$tag.txt" 2>&1
echo $? > "$C/🗑️generated/1030/testsplit-$tag.rc"
