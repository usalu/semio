#!/bin/bash
cd /c/git/semio
S="🎓️teaching/🏛️architecture/🧍️staffage"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
a=$1; p=$2; sc=$3; zoom=$4; shift 4
files=()
for v in "$@"; do
  d="$T/🗑️generated/a3/$a-$p-$sc-$v"
  rm -rf "$d"
  repo tools exec bun -- "$S/🔨️modules/📤️export/🎬️cli/🟦️.ts" --asset $a --pose $p --view $v --scale $sc --format svg --lang en --out "$d" 2>&1 | grep -iv "^\[" 
  files+=("$d"/*.svg)
done
python "$T/🧰️a3/raster.py" $zoom "$T/🗑️generated/a3/$a-$p-$sc.png" "${files[@]}" 2>&1 | tail -1
