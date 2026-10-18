#!/bin/zsh
# usage: gate.sh <logname>
ulimit -n 65536
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript" || exit 9
W=$(nice -n 10 bun "$F/rg7-tools/envelope.ts") || exit 8
SEMIO_SCRIPT_PROCESS_INVOCATION="$W" nice -n 10 bun ./📜️script.ts lint rust-source-direction > "$F/🗑️generated/$1" 2>&1
echo "exit $?" >> "$F/🗑️generated/$1"
