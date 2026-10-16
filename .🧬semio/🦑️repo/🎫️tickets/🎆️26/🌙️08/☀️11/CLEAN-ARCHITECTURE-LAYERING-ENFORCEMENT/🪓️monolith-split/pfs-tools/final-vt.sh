#!/bin/zsh
. "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pfs-tools/env.sh"
PH=$1
"$T/vt.sh" "$PH-presentation-core" "$T/presentation-core.vitest.mjs"
"$T/vt.sh" "$PH-presentation-react" "$PR/🎤️presentation/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts"
"$T/vt.sh" "$PH-pets" "$P/🧪️tests/🎚️config/🟦️.ts"
"$T/vt.sh" "$PH-quiz" "$PR/❓️quiz/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts"
echo done > "$G/pfs-vt-$PH-DONE"
