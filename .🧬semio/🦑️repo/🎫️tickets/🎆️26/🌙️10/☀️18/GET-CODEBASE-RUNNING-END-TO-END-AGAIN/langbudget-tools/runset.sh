#!/bin/sh
# 🧪️ Runs the closure/loader suites on a mirror and writes the sorted failing test names (timings stripped) to OUT.
# usage: sh runset.sh MIRROR_ROOT OUT [extra ./test-file ...]
set -u
root=$1; out=$2; shift 2
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cd "$root"
bun test "./$L/🏗️build/📥️source/🔏️closure/🚪️command/🧪️tests/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🧪️tests/🔭️trace/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🧪️tests/🚫️cache-root-refusal/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🟦️.ts" "./$L/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts" "./$L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts" "./$L/🏗️build/🏛️sealed/🧪️tests/🗂️store/🟦️.ts" "$@" > "$out.raw" 2>&1
grep -E "^\((fail|pass)\)" "$out.raw" | sed -E 's/ \[[0-9.]+m?s\]$//' | sort > "$out.all"
grep "^(fail)" "$out.all" > "$out"
echo "pass $(grep -c '^(pass)' "$out.all") fail $(grep -c '^(fail)' "$out.all")"
