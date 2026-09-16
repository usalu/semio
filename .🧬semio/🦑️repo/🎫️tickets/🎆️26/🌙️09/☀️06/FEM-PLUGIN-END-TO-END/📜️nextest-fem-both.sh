#!/bin/zsh
# 🧪 Sequential no-fail-fast nextest runs of both fem artifact crates: `nextest-fem-both.sh <run>`.
RUN="${1:-1}"
cd /Users/ueli/Documents/semio
"$(dirname "$0")/📜️nextest-fem-crate.sh" semio-s-artifact-fem-2d "$RUN"
"$(dirname "$0")/📜️nextest-fem-crate.sh" semio-s-artifact-fem-3d "$RUN"
echo "BOTH-DONE" >> "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🗑️generated/nextest-semio-s-artifact-fem-3d-${RUN}.txt"
