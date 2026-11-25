#!/bin/bash
# [DEBUG] guarded office rebuild, then publish, support, activate and pane journey against the live 6272 vite
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; G=$T/🗑️generated/claude-pane; S=$G/lanes.status
R=/Users/ueli/Documents/semio; B=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
N=$G/native/target/wasm32-wasip2/wasm-prototype
say() { echo "$(date +%H:%M:%S) $*" >> $S; }
say "fixloop start"
$T/claude-resource-guard.sh $T/claude-pane-cargo.sh > $G/fixloop-cargo.log 2>&1; code=$?
say "fixloop cargo exit $code: $(tail -1 $G/fixloop-cargo.log | cut -c1-80)"
[ $code -eq 0 ] || exit $code
cd $R
$B $T/claude-pane-publish.ts stdio $N/semio_hub_stdio_component.wasm stdio-office $N/semio_hub_stdio_office_component.wasm > $G/fixloop-publish.log 2>&1; say "fixloop publish exit $?"
$B $T/claude-pane-support.ts > $G/fixloop-support.log 2>&1; say "fixloop support exit $?"
$B $T/claude-pane-activate.ts > $G/fixloop-activate.log 2>&1; say "fixloop activate exit $?"
lsof -nP -iTCP:6272 -sTCP:LISTEN > /dev/null 2>&1 || say "WARNING 6272 not listening"
$B $T/standalone-pane-journey.ts > $G/fixloop-journey.log 2>&1; say "fixloop PANE JOURNEY exit $?"
