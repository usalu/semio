#!/usr/bin/env bash
W="/c/git/semio"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/r3-valid"
S="$W/🎓️teaching/🏛️architecture/🧍️staffage"
mkdir -p "$G"
cd "$S/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-alias.config.ts" > "$G/vitest-lib.log" 2>&1 < /dev/null
echo "lib rc=$?" >> "$G/status.txt"
cd "$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-react-alias.config.ts" > "$G/vitest-react.log" 2>&1 < /dev/null
echo "react rc=$?" >> "$G/status.txt"
bash "$T/🧰️r3-valid/catalog_check.sh"
echo ALL_TS_DONE >> "$G/status.txt"
