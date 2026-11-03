#!/bin/bash
D="$(dirname "$0")"
E="⚡️energy"

"$D/rr2_render.sh" "$E/📊️demand/🎬️clip/🏭️primary-energy" Beat1_DreiBilanzgrenzen
"$D/rr2_render.sh" "$E/📊️demand/🎬️clip/🏭️primary-energy" Beat2_Primaerenergiefaktor
"$D/rr2_render.sh" "$E/🧲️physics/🎬️clip/🧲️fundamentals" Beat2_EnergieImAlltag
"$D/rr2_render.sh" "$E/📊️demand/🎬️clip/🔁️recap" Beat1_Bilanzgrenze
"$D/rr2_render.sh" "$E/📊️demand/🎬️clip/🔁️recap" Beat4_WinterUndSommer
"$D/rr2_render.sh" "🏗️bim/🎬️clip/🔗️consistency" Beat6_Consistency
"$D/rr2_render.sh" "🏗️bim/🎬️clip/📊️information" Beat1_DataToInformation
