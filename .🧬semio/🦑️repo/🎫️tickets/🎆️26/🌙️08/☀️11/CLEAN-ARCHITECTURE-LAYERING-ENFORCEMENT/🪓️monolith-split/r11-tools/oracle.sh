#!/bin/zsh
# usage: oracle.sh <tag> [SRC_DIR]  full-resolution oracle on the flow plugin (native and --test); outputs /Users/ueli/.cache/r11/o-<tag>{,-test}.short
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ulimit -n 65536
tag=$1
[ -n "$2" ] && export SRC_DIR="$2"
PL='✏️s/🔌️plugins/🌊️flow'
ROOT="$PL/🗿️artifacts/🌊️flow/🦀️.rs"
nice -n 10 bun "$F/r7-tools/orig-oracle.ts" "$PL" "$ROOT" semio-s-artifact-flow-flow /Users/ueli/.cache/r11/o-$tag
nice -n 10 bun "$F/r7-tools/orig-oracle.ts" "$PL" "$ROOT" semio-s-artifact-flow-flow /Users/ueli/.cache/r11/o-$tag-test --test
