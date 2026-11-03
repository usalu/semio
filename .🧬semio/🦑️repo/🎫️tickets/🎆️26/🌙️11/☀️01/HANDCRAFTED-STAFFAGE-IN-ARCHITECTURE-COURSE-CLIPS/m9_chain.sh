#!/usr/bin/env bash
# M9: render each "<script>|<Scene>|<tag>|<times>" line of $1 one at a time
T="$(ls -d /c/git/semio/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
while IFS='|' read -r rel scene tag times; do
  [ -z "$scene" ] && continue
  [ -f "$T/🗑️generated/m9/$tag-sheet.png" ] && continue
  echo "$(date +%T) start $tag" >> "$T/🗑️generated/m9/chain.log"
  bash "$T/m9_render.sh" "$rel" "$scene" "$tag" "$times" >> "$T/🗑️generated/m9/chain.log" 2>&1
done < "$1"
echo "$(date +%T) chain complete" >> "$T/🗑️generated/m9/chain.log"
