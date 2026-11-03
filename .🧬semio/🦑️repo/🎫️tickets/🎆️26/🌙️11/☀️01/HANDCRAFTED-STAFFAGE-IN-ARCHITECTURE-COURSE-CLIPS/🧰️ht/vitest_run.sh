#!/usr/bin/env bash
W="/c/git/semio"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/ht"
S="$W/🎓️teaching/🏛️architecture/🧍️staffage"
cd "$S/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-alias.config.ts" > "$G/vitest-lib.log" 2>&1 < /dev/null
echo "lib rc=$?" >> "$G/vitest-status.txt"
cd "$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-react-alias.config.ts" > "$G/vitest-react.log" 2>&1 < /dev/null
echo "react rc=$?" >> "$G/vitest-status.txt"
echo VITEST_DONE >> "$G/vitest-status.txt"
