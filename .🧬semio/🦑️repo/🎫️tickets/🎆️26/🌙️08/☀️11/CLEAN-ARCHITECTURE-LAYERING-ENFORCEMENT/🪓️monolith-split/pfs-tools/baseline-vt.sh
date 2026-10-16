#!/bin/zsh
. "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pfs-tools/env.sh"
PH=$1
"$T/vt.sh" "$PH-presentation-core" "$PR/🎤️presentation/🧪️tests/🎚️config/🟦️.ts"
"$T/vt.sh" "$PH-pets" "$P/🧪️tests/🎚️config/🟦️.ts"
"$T/vt.sh" "$PH-quiz" "$PR/❓️quiz/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts" learner-journey
echo done > "$G/pfs-vt-$PH-DONE"
