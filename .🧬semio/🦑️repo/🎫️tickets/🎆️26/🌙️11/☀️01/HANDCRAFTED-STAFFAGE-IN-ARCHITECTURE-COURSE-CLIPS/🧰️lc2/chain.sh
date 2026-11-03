#!/bin/bash
# chain.sh "<domain>|<clip>|<Scene>" ... : renders in en one at a time, extracts frames at 45% and 95%
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/lc2"
for spec in "$@"; do
  IFS='|' read -r dom clip scene <<< "$spec"
  rm -f "$G/media/videos/🐍️/480p15/$scene.mp4"
  bash "$T/🧰️lc2/render.sh" "$dom" "$clip" "$scene" en | head -1 >> "$G/chain.log"
  PYTHONUTF8=1 .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "$T/r2_frames.py" "$G/media/videos/🐍️/480p15/$scene.mp4" "$G/f" 0.45,0.95 > /dev/null 2>&1
done
echo done >> "$G/chain.log"
