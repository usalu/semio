#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1030"
tag=$1; shift
rm -f "$G/hostfix-$tag.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml --no-fail-fast "$@" > "$G/hostfix-$tag.txt" 2>&1
echo $? > "$G/hostfix-$tag.rc"
