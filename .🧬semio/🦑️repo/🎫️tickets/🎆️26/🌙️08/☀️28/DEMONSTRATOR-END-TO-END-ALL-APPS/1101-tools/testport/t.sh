#!/bin/bash
# usage: t.sh <tag> <manifest> <cargo test args...>
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1101"
tag=$1; M=$2; shift 2
rm -f "$G/testport-$tag.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" --no-fail-fast "$@" > "$G/testport-$tag.txt" 2>&1
echo $? > "$G/testport-$tag.rc"
