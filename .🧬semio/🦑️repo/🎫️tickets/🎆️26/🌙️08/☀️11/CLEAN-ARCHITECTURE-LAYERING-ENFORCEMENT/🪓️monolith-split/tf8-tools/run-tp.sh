#!/bin/zsh
# usage: run-tp.sh <phase> <test file relative to the package dir>...
export LC_ALL=en_US.UTF-8
ROOT=/Users/ueli/Documents/semio
F="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
PK="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript"
phase=$1; shift
export TF8_ARTIFACT_ROOT=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/tf8-art/$phase
cd "$PK"
perl -e 'alarm 3000; exec @ARGV' -- bun test --preload "$F/tf8-tools/preload-storage.ts" "$@" > "$G/tf8-$phase-tp.log" 2>&1
echo "exit=$?" >> "$G/tf8-$phase-tp.log"
