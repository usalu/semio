#!/bin/zsh
. "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pfs-tools/env.sh"
PH=$1
"$T/tsc.sh" "$P/📦️packages/🟦️typescript/tsconfig.json" "$G/pfs-tsc-$PH-pets.log"
"$T/tsc.sh" "$T/tsconfig.presentation.json" "$G/pfs-tsc-$PH-presentation.log"
"$T/tsc.sh" "$PR/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" "$G/pfs-tsc-$PH-quiz.log"
"$T/tsc.sh" "$T/tsconfig.print.json" "$G/pfs-tsc-$PH-print.log"
echo done > "$G/pfs-tsc-$PH-DONE"
