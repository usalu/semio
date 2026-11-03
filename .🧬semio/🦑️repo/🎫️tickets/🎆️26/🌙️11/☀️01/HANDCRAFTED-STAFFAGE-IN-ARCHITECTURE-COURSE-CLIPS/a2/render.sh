#!/usr/bin/env bash
# usage: render.sh <asset> <pose|clip> <name> [scale] ; renders dark-line dark-solid thermal svgs into generated/a2/<asset>
cd /c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage || exit 1
T=/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS
asset=$1; kind=$2; name=$3; scale=${4:-25}
shift 4
for view in "${@:-dark-line dark-solid thermal}"; do
  for v in $view; do
    repo tools exec bun -- 🔨️modules/📤️export/🎬️cli/🟦️.ts --asset "$asset" --"$kind" "$name" --view "$v" --scale "$scale" --format svg --out "$T/🗑️generated/a2/$asset/$name-$v" --lang en >/dev/null 2>&1 || echo "FAIL $name $v"
  done
done
