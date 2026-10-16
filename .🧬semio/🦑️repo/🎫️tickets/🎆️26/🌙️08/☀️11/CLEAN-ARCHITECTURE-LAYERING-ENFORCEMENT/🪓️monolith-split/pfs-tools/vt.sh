#!/bin/zsh
# usage: vt.sh <label> <config> [vitest args...]  -> $G/pfs-vt-<label>.json + .log ; cwd = repo root
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
LABEL=$1; CFG=$2; shift 2
cd "$R"
perl -e 'alarm 1750; exec @ARGV' -- node node_modules/vitest/vitest.mjs run --config "$CFG" --testTimeout 120000 --reporter=json --outputFile="$G/pfs-vt-$LABEL.json" "$@" > "$G/pfs-vt-$LABEL.log" 2>&1
echo "exit $?" >> "$G/pfs-vt-$LABEL.log"
