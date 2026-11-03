#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
R="$T/🧰️rp1/render.sh"
S="🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py"
for pair in "$@"; do
  scene="${pair%%:*}"; tag="${pair##*:}"
  bash "$R" "$S" "$scene" "$tag" "5,15,25,35,45,55,65,75,85,95,105,115,125,135,145,155,165,175" >> "$T/🗑️generated/rp1/chain.log" 2>&1
done
echo chain-done >> "$T/🗑️generated/rp1/chain.log"
