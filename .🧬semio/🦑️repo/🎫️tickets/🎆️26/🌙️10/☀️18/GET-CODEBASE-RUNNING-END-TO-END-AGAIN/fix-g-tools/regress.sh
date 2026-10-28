#!/bin/bash
# Runs every touched single-file suite one after the other (nice 10) and prints one verdict line each.
cd /Users/ueli/Documents/semio || exit 1
export LC_ALL=en_US.UTF-8 SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/fix-g/art"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
P="🧰️framework/🔨️modules/🏃️process"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"
run(){ d="$1"; shift; out=$(cd "$d" && nice -n 10 bun test "$@" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)|\(fail\)" | tr '\n' ' '); echo "$(basename "$(dirname "$d")")/$(basename "$d") $*: $out"; }
run "$P/🏗️build/🌡️host-capacity" ./🧪️tests/🟦️.ts
run "$P/🏗️build/🌡️host-capacity" ./🧪️tests/📐️estimates/🟦️.ts
run "$P/🏗️build/🌡️host-capacity" ./🧪️tests/🚪️gate/🟦️.ts
run "$P/🏗️build/🌡️host-capacity" ./🧪️tests/📡️sampling/🟦️.ts
run "$P/🏗️build/🌡️host-capacity" ./🧪️tests/🤝️handover/🟦️.ts
run "$B/🌡️host-capacity/🧪️tests" ./🟦️.ts
run "$B/🌡️host-capacity/🧪️tests/📈️memory-fit" ./🟦️.ts
run "$B/🌡️host-capacity/🧪️tests/🏊️farm" ./🟦️.ts
run "$B/🌡️host-capacity/🧪️tests/🔁️restart" ./🟦️.ts
run "$B/🌡️host-capacity/🧪️tests/🧮️arguments" ./🟦️.ts
run "$P/🔒️leases/📈️standing/🧪️tests" ./🟦️.ts
LC_ALL=de_DE.UTF-8 run "$P/🔒️leases/📈️standing/🧪️tests/🚦️live" ./🟦️.ts
run "$P/🔒️leases/🧭️order/🧪️tests" ./🟦️.ts
run "$P/🔒️leases/⏳️wait/🧪️tests" ./🟦️.ts
run "$P/🔒️leases/🧪️tests/📋️status" ./🟦️.ts
run "$B/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests" ./🟦️.ts
run "$B/📥️source/🔏️closure/🗓️scheduling/🧪️tests" ./🟦️.ts
run "$B/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests" ./🟦️.ts
run "$B/🧹️cache-gc/🧪️tests/💰️budget" ./🟦️.ts
run "$B/🧹️cache-gc/🧪️tests" ./🟦️.ts
run "$L/⚡️caching/💾️storage/🧾️ledger/🧪️tests" ./🟦️.ts
run "$L/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests" ./🟦️.ts
run "$B/🕸️graph/🕰️generations/🧩️claims/🧪️tests" ./🟦️.ts
run "$B/🕸️graph/🕰️generations/🔌️load/🧪️tests" ./🟦️.ts
run "$L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🧹️supersede" ./🟦️.ts
run "$B/📥️inputs/🌳️owner-view/🧪️tests" ./🟦️.ts
run "$B/📥️inputs/🌳️owner-view/🧪️tests/🏝️reach" ./🟦️.ts
run "$B/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference" ./🟦️.ts
echo DONE
