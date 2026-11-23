#!/usr/bin/env bash
# usage: prev.sh <video> <chapter> <outname>
cd /c/git/semio
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
R=/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe
"$R" tools exec --light bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language de --video "$1" --chapter "$2" --profile preview --jobs 3 --no-play --output-root "$TK/🗑️generated/fix-vent2/out" > "$TK/🗑️generated/fix-vent2/$3.log" 2>&1
tail -5 "$TK/🗑️generated/fix-vent2/$3.log"
