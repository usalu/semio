#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"; G="$T/🗑️generated/H1"
R() { bun "$T/🧰️H1/render.ts" "$@" 2>&1 | tail -1; }
for a in child-on-kick-scooter courier-with-hand-truck cyclist elderly-with-cane jogger kneeling-gardener leaning-person; do echo "## $a"; bun "$T/🧰️H1/probe.ts" $a check 2>&1 | grep -v "^$" | grep -v "EMPTY" ; done > "$G/chk-all.txt" 2>&1
R child-on-kick-scooter 100 "$G/r-scooter100.svg" pose:gliding pose:pushing-off pose:braking clip:kick-and-glide@0.5
R courier-with-hand-truck 100 "$G/r-courier100.svg" pose:waiting pose:tilted-back pose:parked-upright pose:walking-contact
R cyclist 100 "$G/r-cyclist100.svg" pose:riding-upright pose:standing-on-pedals pose:coasting pose:dismounted-astride
R elderly-with-cane 100 "$G/r-elderly100.svg" pose:stooped-standing pose:cane-planted pose:passing-left clip:cane-gait@0.5
R kneeling-gardener 100 "$G/r-gardener100.svg" pose:kneeling-weeding pose:kneeling-upright pose:reaching-trug pose:dig-in
R leaning-person 100 "$G/r-leaning100.svg" pose:leaning-shoulder-wall pose:phone-call pose:pushing-off pose:standing-away
R jogger 500 "$G/r-jogger500.svg" pose:initial-contact-l pose:flight-r
python3 "$T/🧰️H1/topng.py" "$G"/r-*.svg > /dev/null
echo done > "$G/job1-done"
