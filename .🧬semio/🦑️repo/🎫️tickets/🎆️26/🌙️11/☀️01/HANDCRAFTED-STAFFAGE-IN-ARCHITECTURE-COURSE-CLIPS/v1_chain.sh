#!/usr/bin/env bash
# V1: render scenes strictly one after another (never parallel), quality l, 15 fps.
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/v1"
R="📊️demand/🎬️clip/🔁️recap/🐍️.py"
F="🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py"
for job in "$R Beat1_Bilanzgrenze" "$R Beat3_Gewinnseite" "$F Beat2_EnergieImAlltag" "$F Beat3_Energieerhaltung" "$F Beat4_Waermepumpe" "$F Beat6_ThermischeMasse" "$F Beat1_Leistung" "$R Beat2_Verlustseite" "$R Beat4_WinterUndSommer"; do
  set -- $job
  bash "$T/v1_render.sh" "$1" "$2" > "$G/r_$2.txt" 2>&1
  echo "$2 exit $?" >> "$G/chain.log"
done
echo done >> "$G/chain.log"
