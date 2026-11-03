#!/usr/bin/env bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"; S="🎓️teaching/🏛️architecture/🧍️staffage"; G="$T/🗑️generated/pv"
for a in oil-pumpjack circulation-pump hot-water-cylinder gas-boiler gas-hob; do repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset $a > "$G/check2-$a.txt" 2>&1; echo "$a $?" >> "$G/checks.txt"; done
cd "$S/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "/c/git/semio/$T/q2-vitest-alias.config.ts" > "/c/git/semio/$G/vitest-lib.txt" 2>&1
cd "/c/git/semio/$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "/c/git/semio/$T/q2-vitest-react-alias.config.ts" > "/c/git/semio/$G/vitest-react.txt" 2>&1
echo done > "/c/git/semio/$G/tests.done"
