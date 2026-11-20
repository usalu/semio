#!/bin/sh
# 👀 Renders one list of one scale of animals/sheep from several directions into 🗑️generated/r3-sheep/<list><scale>-<az>-<el>.png. Usage: sh r3-sheep-look.sh <scale> <list> "<az>:<el> ..." [width]
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for d in $3; do
  az=${d%%:*}; el=${d##*:}
  "$BUN" volumes-render.ts animals/sheep --scale "$1" --list "$2" --width "${4:-1300}" --az "$az" --el "$el" >/dev/null 2>&1 || echo "render failed $d"
  cp "🗑️generated/volumes-tools/render/animals/sheep.s$1.$2.png" "🗑️generated/r3-sheep/$2$1-$az-$el.png"
done
