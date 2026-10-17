#!/bin/zsh
# usage: probes.sh <label> ; reader probes: tool-job failures dump, tool-job evidence of the trinity and draw roots, completion oracle
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
T="$F/c4p30-tools"
L=$1
cd $R
perl -e 'alarm 1500; exec @ARGV' -- bun "$T/failures-dump.ts" > "$G/c4p30-failures-$L.txt" 2> "$G/c4p30-failures-$L.err"
perl -e 'alarm 1500; exec @ARGV' -- bun "$T/tooljob-probe.ts" $L > "$G/c4p30-tooljob-$L.txt" 2> "$G/c4p30-tooljob-$L.err"
perl -e 'alarm 600; exec @ARGV' -- bun "$T/completion-probe.ts" > "$G/c4p30-completion-$L.log" 2>&1
echo done > "$G/c4p30-probes-$L.done"
