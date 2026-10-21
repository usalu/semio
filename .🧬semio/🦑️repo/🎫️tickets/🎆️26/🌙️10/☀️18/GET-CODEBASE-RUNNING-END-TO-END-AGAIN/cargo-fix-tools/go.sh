#!/bin/zsh
# usage: go.sh FROM UNTIL  (starts the runner for steps FROM..UNTIL in lane cargofix, detached)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
W=$C/🗑️generated/cargo-fix/window; rm -f $W/STOPPED
LANE=cargofix GATE_MODE=${GATE_MODE:-none} STOP_ON_FAIL=${STOP_ON_FAIL-1} FROM=$1 UNTIL=$2 nohup nice -n 5 $C/cargo-fix-tools/plan/run.sh >> $W/runner.out 2>&1 &
RP=$!; disown
