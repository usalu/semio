#!/bin/bash
# usage: t.sh <tag> <list-file> [test-binary args after --]  — native cargo test through the fleet slot
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1102"; tag=$1; list=$2; shift 2
LABEL=testport-$tag; case "$tag" in W-*) LABEL=$tag;; esac
rm -f "$G/testport-$tag.rc"
SEMIO_LANGUAGE=en "$C/1102-tools/slot.sh" "$LABEL" repo cargo --store=shared --configuration=prototype -- test --manifest-path "${MANIFEST:-Cargo.toml}" --no-fail-fast $(sed 's/^/-p /' "$C/1102-tools/testport/$list" | tr '\n' ' ') "$@" > "$G/testport-$tag.txt" 2>&1
echo $? > "$G/testport-$tag.rc"
