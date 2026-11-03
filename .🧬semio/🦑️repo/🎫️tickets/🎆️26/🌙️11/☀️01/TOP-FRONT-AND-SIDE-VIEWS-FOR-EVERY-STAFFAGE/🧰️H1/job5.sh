#!/bin/bash
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"; G="$T/🗑️generated/H1"; S="🎓️teaching/🏛️architecture/🧍️staffage"
for a in jogger leaning elderly gardener courier cyclist scooter; do python3 "$T/🧰️H1/a_$a.py" || echo "BUILD FAIL $a"; done
for a in jogger leaning-person elderly-with-cane kneeling-gardener courier-with-hand-truck cyclist child-on-kick-scooter; do echo "## $a"; bun "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset $a 2>&1 | grep -v "DEBUG\|undefined\|^\]\|^  \"\|^\[" | cut -c1-250; done > "$G/official-check.txt" 2>&1
for a in child-on-kick-scooter courier-with-hand-truck cyclist elderly-with-cane jogger kneeling-gardener leaning-person; do echo "## $a"; bun "$T/🧰️H1/probe.ts" $a check 2>&1 | grep -v "^$"; done > "$G/chk-all.txt" 2>&1
echo done > "$G/job5-done"
