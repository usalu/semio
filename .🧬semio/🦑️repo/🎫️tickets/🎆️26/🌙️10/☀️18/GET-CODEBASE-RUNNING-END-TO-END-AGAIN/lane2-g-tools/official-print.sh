#!/bin/sh
# lane2-g: ONE official run of an artifact consumer that goes through buildRepositoryCargoArtifacts (print-rs build -> runArtifactRustPackageMain)
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/lane2-g/official-print.log"
date > "$OUT"
S=$(date +%s)
SEMIO_BUILD_GRAPH_POLICY=current /usr/bin/time -l bun repo run @semio-tech/print-rs:build --skip-cache >> "$OUT" 2>&1
echo "exit $? wall $(($(date +%s)-S)) s" >> "$OUT"
date >> "$OUT"
