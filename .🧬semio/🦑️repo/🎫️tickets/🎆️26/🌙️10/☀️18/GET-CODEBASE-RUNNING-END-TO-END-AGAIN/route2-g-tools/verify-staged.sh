#!/bin/sh
# 🧪 route2-g: runs every staged test file (🟦️.ts under a tests directory) through stage-run.mjs and prints one verdict line per file.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/route2-g-tools"
find "$T/staged" -name '🟦️.ts' -path '*🧪️tests*' | sed "s#$T/staged/##" | sort | while read -r f; do
  CENSUS_ARTROOT="${ART:-/private/tmp/claude-501/rg2o}" bun "$T/stage-run.mjs" "$f" "" 200 2>&1 | sed -n 1p | sed 's#^.*mirror/##' | cut -c1-200
done
