#!/usr/bin/env bash
W="/c/git/semio"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/r3-people/prev2"
S="$W/🎓️teaching/🏛️architecture/🧍️staffage"
mkdir -p "$G"
cd "$S/📦️packages/🟦️typescript" || exit 1
while read -r asset pose scale view vp; do
  repo tools exec bun -- "$S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset "$asset" --scale "$scale" --pose "$pose" --view "$view" --viewpoint "$vp" --out "$G/$asset-$pose-$scale-$view-$vp" < /dev/null > "$G/log-$asset-$pose-$scale-$view-$vp.txt" 2>&1
  echo "$asset $pose $scale $view $vp rc=$?" >> "$G/status.txt"
done < "$T/🧰️r3-people/jobs2.txt"
echo DONE >> "$G/status.txt"
