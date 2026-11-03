#!/bin/bash
# usage: t.sh <tag> <package-dir> <cargo test args...>   (native test; rc marker)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
tag=$1; dir=$2; shift 2
rm -f "$C/🗑️generated/1102/runtime-$tag.rc"
RUST_BACKTRACE=1 SEMIO_LANGUAGE=en "$C/1102-tools/slot.sh" W-runtime-$tag repo cargo --store=shared --configuration=prototype -- test --manifest-path "$dir/Cargo.toml" --no-fail-fast "$@" > "$C/🗑️generated/1102/runtime-$tag.txt" 2>&1
echo $? > "$C/🗑️generated/1102/runtime-$tag.rc"
