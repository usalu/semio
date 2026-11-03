#!/usr/bin/env bash
W="/c/git/semio"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/r3-people/prev"
S="$W/🎓️teaching/🏛️architecture/🧍️staffage"
mkdir -p "$G"
cd "$S/📦️packages/🟦️typescript" || exit 1
while read -r asset pose scale; do
  repo tools exec bun -- "$S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset "$asset" --scale "$scale" --pose "$pose" --view dark-line --viewpoint plan --out "$G/$asset-$pose-$scale" < /dev/null > /dev/null 2>&1
  echo "$asset $pose $scale rc=$?" >> "$G/status.txt"
done < "$T/🧰️r3-people/jobs.txt"
echo DONE >> "$G/status.txt"
