#!/usr/bin/env bash
# V1: waits for the first chain, then renders the radiator scene (Beat5_Strahlung), still one scene at a time.
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/v1"
until grep -q '^done' "$G/chain.log" 2>/dev/null; do sleep 20; done
bash "$T/v1_render.sh" "🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py" Beat5_Strahlung > "$G/r_Beat5_Strahlung.txt" 2>&1
echo "Beat5_Strahlung exit $?" >> "$G/chain2.log"
