#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/a1"
declare -A POSES=( [seated-office-worker]="typing reading-screen leaning-back" [sleeping-person]="supine side curled" )
for A in seated-office-worker sleeping-person; do
  for V in ${VIEWS:-dark-line dark-solid thermal light-line light-solid forces moments}; do
    D="$G/v-$A-$V"; rm -rf "$D"
    for p in ${POSES[$A]}; do "$T/🧰️a1/render.sh" $A $p $V ${SCALE:-20} "$D" >/dev/null; done
    repo tools exec bun -- "$T/🧰️a1/sheet.mjs" "$G/v-$A-$V.png" 700 3 "$D/"*.svg 2>&1 | grep -v "^\["
  done
done
echo alldone
