#!/bin/bash
TK="C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
for pair in "🧲️fundamentals Beat5_Zusammenfassung" "🧲️fundamentals Beat8_UWert" "🧲️fundamentals Beat9_WaermestromFormel" "🧱️conduction Beat3_UWertUndGradient" "🧱️conduction Beat6_Waermebruecken" "🧱️conduction Beat5_Gebaeudehuelle" "🌬️convection Beat1_LueftungImWinter" "🧱️conduction Beat2_RWert"; do
  bash "$TK/heat-a/one.sh" $pair
done
echo x > "$TK/🗑️generated/heat-a-out/MANYDONE"
