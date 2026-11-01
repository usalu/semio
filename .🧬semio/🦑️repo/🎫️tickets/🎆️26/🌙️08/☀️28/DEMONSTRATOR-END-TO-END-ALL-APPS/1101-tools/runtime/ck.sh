#!/bin/bash
# usage: ck.sh <tag> <cargo check args...>  (root workspace native check; waits for runtime-p2.rc first)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
tag=$1; shift
rm -f "$C/🗑️generated/1030/runtime-$tag.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --keep-going "$@" > "$C/🗑️generated/1030/runtime-$tag.txt" 2>&1
echo $? > "$C/🗑️generated/1030/runtime-$tag.rc"
