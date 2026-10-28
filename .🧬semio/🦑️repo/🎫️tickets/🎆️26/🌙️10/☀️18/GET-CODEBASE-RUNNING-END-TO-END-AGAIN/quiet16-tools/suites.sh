#!/bin/sh
# 🧪 quiet16: runs the host-capacity, upgrade, marks and graph-upgrade suites of a root and prints the failure set (sorted test names) to stdout, the bun summary lines to stderr: sh suites.sh <root>
root=${1:?root}
cd "$root" || exit 2
H=./🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests
L=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests
G=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests
bun test "$H/🔼️upgrade/👥️agents/🟦️.ts" "$H/🔼️upgrade/🟦️.ts" "$H/🔼️upgrade/🤝️mutual/🟦️.ts" "$H/🔼️upgrade/🌱️grow/🟦️.ts" "$H/🔼️upgrade/🚪️gate/🟦️.ts" "$H/🌤️followup/🟦️.ts" "$H/📐️estimates/🟦️.ts" "$H/🪜️phases/🟦️.ts" "$H/🪜️phases/🚪️gate/🟦️.ts" "$H/🚪️gate/🟦️.ts" "$H/🔏️fence/🟦️.ts" "$H/🧾️blockers/🟦️.ts" "$H/🤝️handover/🟦️.ts" "$H/🟦️.ts" "$L/👥️agents/🟦️.ts" "$L/🌤️followup/🟦️.ts" "$L/🌱️grow/🟦️.ts" "$L/🔼️upgrade/🟦️.ts" "$L/🪟️partial/🟦️.ts" "$L/🏊️farm/🟦️.ts" "$L/📈️memory-fit/🟦️.ts" "$L/🪜️phases/🟦️.ts" "$L/🧾️label/🟦️.ts" "$L/🟦️.ts" "$G/🔼️upgrade-before-custody/🟦️.ts" "$G/🎟️prefetch-booking/🟦️.ts" ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🟦️.ts 2>&1 | tee /dev/stderr | grep -a -E "^\(fail\)" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort -u
