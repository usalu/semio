#!/bin/bash
# usage: chkt.sh <tag> <list-file> [extra]  — native cargo check --tests --keep-going for crates listed one per line
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"; tag=$1; list=$2; shift 2
rm -f "$G/testport-$tag.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml --tests --keep-going $(sed 's/^/-p /' "$C/1101-tools/testport/$list" | tr '\n' ' ') "$@" > "$G/testport-$tag.txt" 2>&1
echo $? > "$G/testport-$tag.rc"
