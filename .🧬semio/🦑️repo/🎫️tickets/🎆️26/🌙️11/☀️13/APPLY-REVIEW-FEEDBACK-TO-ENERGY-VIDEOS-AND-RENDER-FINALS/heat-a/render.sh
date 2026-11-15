#!/bin/bash
cd /c/git/semio
TK="C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
for ch in Heating_01_Introduction Heating_02_Conduction Heating_03_Convection; do
  repo tools exec bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language de --video heating --chapter $ch --profile preview --audience speaker --jobs 2 --no-play --output-root "$TK/🗑️generated/heat-a-out" > "$TK/🗑️generated/heat-a-out/render-$ch.log" 2>&1
done
echo finished > "$TK/🗑️generated/heat-a-out/ALLDONE"
