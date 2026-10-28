#!/bin/sh
# FOR gate-g: run from the repo root AFTER rev3-g/win-withheld, rev3-g/upgrade-before-custody and then
# `python3 C/rev3fix-g-tools/batch.py C/rev3fix-g-tools/staged/rev3fix apply .`
# (VERIFY_ROOT=<overlay> runs the same list in a scratch overlay). Prints one `pass/fail` line per file; every file must show `0 fail`
# (the timeout is raised because several of these laws are timing bound under the load of ~20 agents). Run on the real tree: the planner laws refuse a symlinked overlay.
ROOT="${VERIFY_ROOT:-.}"
cd "$ROOT" || exit 1
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P="🧰️framework/🔨️modules/🏃️process"
G="$B/🕸️graph/🕰️generations/🧪️tests"
U="$L/⚡️caching/🦀️cargo/🗃️unit-cache"
H="$P/🏗️build/🌡️host-capacity/🧪️tests"
run() { printf '%s  ' "$1"; ( cd "$(dirname "$1")" && nice -n 10 bun test "./$(basename "$1")" --timeout 300000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ' ); echo; }
for f in \
 "$G/🌊️patch-drift/🟦️.ts" "$G/🌱️patch-growth/🟦️.ts" "$G/♻️refine/🟦️.ts" "$G/🧮️fact-index/🟦️.ts" "$G/🚧️unfounded-withheld/🟦️.ts" "$G/🚧️unfounded-withheld/🪟️windows/🟦️.ts" "$G/🧱️index-chunks/🟦️.ts" \
 "$B/🕸️graph/🧪️tests/🎟️prefetch-booking/🟦️.ts" "$B/🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts" "$B/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts" \
 "$B/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🔒️farm-close/🟦️.ts" "$B/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts" \
 "$H/⚓️boot-anchor/🟦️.ts" "$H/🚪️gate/🟦️.ts" "$H/🤝️handover/🟦️.ts" "$H/🔼️upgrade/🟦️.ts" "$H/🪜️phases/🟦️.ts" \
 "$B/🌡️host-capacity/🧪️tests/🪜️phases/🟦️.ts" "$B/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts" \
 "$P/🔒️leases/📁️physical/🧪️tests/🔁️lock-attempts/🟦️.ts" "$P/🔒️leases/📁️physical/🧪️tests/🪟️windows/🟦️.ts" \
 "$U/🧵️lane/📤️retention/🧪️tests/🪢️link-less/🟦️.ts" "$U/🧵️lane/📤️retention/🧪️tests/🪦️dead-holder/🟦️.ts" "$U/🧵️lane/📤️retention/🧪️tests/🔐️retention-io/🟦️.ts" \
 "$U/🧵️lane/📤️retention/🧪️tests/📤️retention-model/🟦️.ts" "$U/🧵️lane/📤️retention/🧪️tests/🔀️retention-interleaving/🟦️.ts" "$U/🧵️lane/📤️retention/🧪️tests/🛣️consumer-lanes/🟦️.ts" \
 "$U/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/🟦️.ts" "$U/🧵️lane/🏊️pool/🧪️tests/🔐️pool-claims/🟦️.ts" "$U/🧹️gc/🧪️tests/🟦️.ts"
do run "$f"; done
printf '%s  ' "planner skip (repo root cwd)"
for f in "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🙈️excluded-directory/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts"; do nice -n 10 bun test "./$f" --timeout 300000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' '; done; echo
