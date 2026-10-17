#!/bin/zsh
# usage: probes.sh <label> ; runs the source-text reader probes (tool-job census, completion oracle, publication authority audit, fill audit)
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
L=$1
cd $R
perl -e 'alarm 1500; exec @ARGV' -- bun "$F/c4p6-tools/gate-probe.ts" > "$G/c4p6-gateprobe-$L.json" 2> "$G/c4p6-gateprobe-$L.err"
perl -e 'alarm 600; exec @ARGV' -- bun "$F/c4p6-tools/completion-probe.ts" > "$G/c4p6-completion-$L.log" 2>&1
perl -e 'alarm 600; exec @ARGV' -- bun "$F/c4p6-tools/audit-probe.ts" "$R" Puzzle5dPlayApp > "$G/c4p6-audit-$L.log" 2>&1
perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p6-tools/fillprobe.ts" "$G/c4p6-fillprobe-$L.json" > "$G/c4p6-fillprobe-$L.log" 2>&1
echo done > "$G/c4p6-probes-$L.done"
