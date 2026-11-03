#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
S="$W/🎓️teaching/🏛️architecture/🧍️staffage"
G="$T/🗑️generated/ty"
cd "$S" && repo tools exec bun -- "./🔨️modules/🗂️catalog/🩺️check/🟦️.ts" > "$G/catalog.log" 2>&1 < /dev/null
echo "catalog rc=$?" >> "$G/status.txt"
cd "$S/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-alias.config.ts" > "$G/vitest-lib.log" 2>&1 < /dev/null
echo "lib rc=$?" >> "$G/status.txt"
cd "$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-react-alias.config.ts" > "$G/vitest-react.log" 2>&1 < /dev/null
echo "react rc=$?" >> "$G/status.txt"
echo DONE >> "$G/status.txt"
