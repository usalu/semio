#!/bin/bash
# [DEBUG] waits for the fix loop, then republishes space, stdio and stdio-office, refreshes support and activation, and reruns the full S journeys against the live 6070
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; G=$T/🗑️generated/claude-pane; F=$T/🗑️generated/claude-full-s; S=$G/lanes.status
R=/Users/ueli/Documents/semio; B=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
N=$G/native/target/wasm32-wasip2/wasm-prototype
say() { echo "$(date +%H:%M:%S) $*" >> $S; }
START=$(( $(wc -l < $S) + 1 ))
until tail -n +$START $S | grep -qE "fixloop PANE JOURNEY|fixloop cargo exit [1-9]"; do sleep 30; done
tail -n +$START $S | grep -q "fixloop cargo exit [1-9]" && { say "redo aborted: fixloop cargo failed"; exit 1; }
cd $R
$B $T/claude-fulls-publish.ts space=$N/semio_hub_space_component.wasm stdio=$N/semio_hub_stdio_component.wasm stdio-office=$N/semio_hub_stdio_office_component.wasm > $F/redo-publish.log 2>&1; say "redo fulls publish exit $?"
$B $T/claude-pane-support.ts > $F/redo-support.log 2>&1; say "redo support exit $?"
$B $T/claude-fulls-activate.ts s space stdio stdio-office > $F/redo-activate.log 2>&1; say "redo fulls activate exit $?"
lsof -nP -iTCP:6070 -sTCP:LISTEN > /dev/null 2>&1 || say "WARNING 6070 not listening"
for l in en de; do $B $T/claude-fulls-journey.ts $l http://127.0.0.1:6070/ > $F/redo-journey-$l.log 2>&1; say "redo FULLS $l exit $?"; done
say "redo DONE"
