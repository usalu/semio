#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"; G="$T/🗑️generated/H1"
S="🎓️teaching/🏛️architecture/🧍️staffage"
for a in jogger leaning-person elderly-with-cane kneeling-gardener courier-with-hand-truck cyclist child-on-kick-scooter; do echo "## $a"; bun "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset $a 2>&1 | tail -60; done > "$G/official-check.txt" 2>&1
echo done > "$G/job2-done"
