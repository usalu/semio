#!/bin/zsh
# usage: infrun.sh <label> <level> : runs the print-viz-inference differential harness (every check against its d3 oracle) without the script envelope
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4ts-tools/env.sh
I="$PR/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript"
cd "$I"
nice -n 10 perl -e 'alarm 1500; exec @ARGV' -- bun -e "import {verifyVizChartInferences} from '$I/📜️script.ts'; await verifyVizChartInferences(['$2']);" > "$G/c4ts-inf-$1.log" 2>&1
echo "exit=$?" >> "$G/c4ts-inf-$1.log"
tail -14 "$G/c4ts-inf-$1.log"
