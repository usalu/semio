#!/bin/bash
# usage: c.sh <tag> <check|test> <cargo args...>   (root workspace, native)
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1101"
tag=$1; verb=$2; shift 2
rm -f "$G/compute-$tag.rc"
flag=--keep-going; [ "$verb" = test ] && flag=--no-fail-fast
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- $verb --manifest-path Cargo.toml $flag "$@" > "$G/compute-$tag.txt" 2>&1
echo $? > "$G/compute-$tag.rc"
