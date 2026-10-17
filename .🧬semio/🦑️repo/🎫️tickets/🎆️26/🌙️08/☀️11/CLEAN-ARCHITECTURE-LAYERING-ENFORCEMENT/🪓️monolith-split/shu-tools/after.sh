#!/bin/zsh
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/shu-tools/env.sh
$T/tsc.sh "$1"
$T/vitest.sh "$1"
$T/puzzle-vt.sh "$1"
echo done > "$G/shu-after-$1.done"
