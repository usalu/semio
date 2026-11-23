#!/bin/bash
cd /c/git/semio
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
R=/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe
ROOT="$(pwd)/$TK/🗑️generated/fix-eb2/out"
mkdir -p "$ROOT"
for c in "$@"; do
  "$R" tools exec --light bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language de --video energy_balance --chapter "$c" --profile preview --jobs 3 --no-play --output-root "$ROOT" > "$TK/🗑️generated/fix-eb2/render-$c.log" 2>&1
  echo "done $c $?" >> "$TK/🗑️generated/fix-eb2/status.txt"
done
