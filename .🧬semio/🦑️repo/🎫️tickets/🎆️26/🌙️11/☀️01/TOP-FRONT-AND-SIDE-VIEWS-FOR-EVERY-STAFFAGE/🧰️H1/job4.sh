#!/bin/bash
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"; G="$T/🗑️generated/H1"; S="🎓️teaching/🏛️architecture/🧍️staffage"
python3 "$T/🧰️H1/fix_cyclist200.py"
for a in jogger leaning elderly gardener courier cyclist scooter; do python3 "$T/🧰️H1/a_$a.py" || echo "BUILD FAIL $a"; done
for a in jogger leaning-person elderly-with-cane kneeling-gardener courier-with-hand-truck cyclist child-on-kick-scooter; do echo "## $a"; bun "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset $a 2>&1 | grep -v "DEBUG\|undefined\|^\]\|^  \"\|^\[" | cut -c1-250; done > "$G/official-check.txt" 2>&1
for a in child-on-kick-scooter courier-with-hand-truck cyclist elderly-with-cane jogger kneeling-gardener leaning-person; do echo "## $a"; bun "$T/🧰️H1/probe.ts" $a check 2>&1 | grep -v "^$"; done > "$G/chk-all.txt" 2>&1
PPM=300 bun "$T/🧰️H1/render.ts" child-on-kick-scooter 50 "$G/r-scooter50.svg" pose:pushing-off clip:kick-and-glide@1.0 2>&1 | tail -1 | cut -c1-60
python3 "$T/🧰️H1/topng.py" "$G"/r-scooter50.svg > /dev/null
echo done > "$G/job4-done"
