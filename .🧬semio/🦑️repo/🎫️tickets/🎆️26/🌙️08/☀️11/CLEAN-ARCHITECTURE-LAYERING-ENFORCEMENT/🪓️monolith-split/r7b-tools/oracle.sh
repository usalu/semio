#!/bin/zsh
# usage: oracle.sh <tag> [SRC_DIR]  runs the plain full-resolution oracle on the jack artifact (native and --test); output /Users/ueli/.cache/r7b/o-<tag>{,-test}.short
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
tag=$1
PL='✏️s/🔌️plugins/🔱️trinity'
ROOT="$PL/🗿️artifacts/🔌️jack/🦀️.rs"
nice -n 10 bun "$F/r7-tools/orig-oracle.ts" "$PL" "$ROOT" semio-s-artifact-trinity-jack /Users/ueli/.cache/r7b/o-$tag
nice -n 10 bun "$F/r7-tools/orig-oracle.ts" "$PL" "$ROOT" semio-s-artifact-trinity-jack /Users/ueli/.cache/r7b/o-$tag-test --test
