#!/bin/bash
cd /c/git/semio
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
R=/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe
"$R" tools exec --light bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language de --video heating --chapter "$1" --profile preview --jobs 3 --no-play --output-root "$TK/🗑️generated/fix-heat2/out" > "$TK/🗑️generated/fix-heat2/$1.log" 2>&1
echo done $1 >> "$TK/🗑️generated/fix-heat2/done.txt"
