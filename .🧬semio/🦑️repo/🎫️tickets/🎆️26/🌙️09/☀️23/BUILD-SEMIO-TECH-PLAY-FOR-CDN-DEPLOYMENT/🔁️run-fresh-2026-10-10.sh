#!/bin/bash
# Runs one cold play release generation and records its exit status beside the log.
cd /Users/ueli/Documents/semio || exit 2
L=".🧬semio/🦑️repo/⚡️cache/play-fleet/coordinator"
N="${1:-1}"
export SEMIO_TICKET_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-8}"
echo "start $(date '+%F %T')" > "$L/fresh-$N.status"
bun nx run @semio-tech/semio-tech-play:build-fresh > "$L/fresh-$N.log" 2>&1
echo "exit $? $(date '+%F %T')" >> "$L/fresh-$N.status"
