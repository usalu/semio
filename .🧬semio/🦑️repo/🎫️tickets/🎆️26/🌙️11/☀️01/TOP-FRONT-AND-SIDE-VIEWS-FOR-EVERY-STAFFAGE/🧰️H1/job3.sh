#!/bin/bash
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"; G="$T/🗑️generated/H1"; S="🎓️teaching/🏛️architecture/🧍️staffage"
for a in jogger leaning elderly gardener courier cyclist scooter; do python3 "$T/🧰️H1/a_$a.py" || echo "BUILD FAIL $a"; done
for a in jogger leaning-person elderly-with-cane kneeling-gardener courier-with-hand-truck cyclist child-on-kick-scooter; do echo "## $a"; bun "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset $a 2>&1 | tail -30 | cut -c1-250; done > "$G/official-check.txt" 2>&1
for a in child-on-kick-scooter courier-with-hand-truck cyclist elderly-with-cane jogger kneeling-gardener leaning-person; do echo "## $a"; bun "$T/🧰️H1/probe.ts" $a check 2>&1 | grep -v "^$"; done > "$G/chk-all.txt" 2>&1
R() { PPM=$P bun "$T/🧰️H1/render.ts" "$@" 2>&1 | tail -1 | cut -c1-80; }
P=300; R cyclist 50 "$G/r-cyclist50.svg" pose:riding-upright clip:pedal-cycle@1.0
P=300; R courier-with-hand-truck 50 "$G/r-courier50.svg" pose:waiting pose:tilted-back
P=300; R child-on-kick-scooter 50 "$G/r-scooter50.svg" pose:pushing-off
P=300; R leaning-person 50 "$G/r-leaning50.svg" pose:phone-call pose:leaning-arms-folded
P=300; R elderly-with-cane 50 "$G/r-elderly50.svg" pose:cane-planted
P=300; R kneeling-gardener 50 "$G/r-gardener50.svg" pose:kneeling-weeding pose:reaching-trug
P=240; R cyclist 500 "$G/r-cyclist500.svg" pose:riding-upright
P=240; R courier-with-hand-truck 500 "$G/r-courier500.svg" pose:waiting pose:tilted-back
P=240; R jogger 500 "$G/r-jogger500.svg" pose:initial-contact-l pose:flight-r
P=240; R child-on-kick-scooter 200 "$G/r-scooter200.svg" pose:pushing-off
python3 "$T/🧰️H1/topng.py" "$G"/r-*.svg > /dev/null
echo done > "$G/job3-done"
