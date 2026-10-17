#!/bin/zsh
# usage: oracle-rw.sh <tag> [pristine]  plain full-resolution oracle on the rewriting artifact (native and --test); `pristine` runs on the untouched copy in /Users/ueli/.cache/r7b/orig/trinity
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
tag=$1
PL='✏️s/🔌️plugins/🔱️trinity'
ROOT="$PL/🗿️artifacts/♻️rewriting/🦀️.rs"
[ "$2" = pristine ] && export SRC_DIR=/Users/ueli/.cache/r7b/orig/trinity
nice -n 10 bun "$F/r7-tools/orig-oracle.ts" "$PL" "$ROOT" semio-s-artifact-trinity-rewriting /Users/ueli/.cache/r7b/o-$tag
nice -n 10 bun "$F/r7-tools/orig-oracle.ts" "$PL" "$ROOT" semio-s-artifact-trinity-rewriting /Users/ueli/.cache/r7b/o-$tag-test --test
