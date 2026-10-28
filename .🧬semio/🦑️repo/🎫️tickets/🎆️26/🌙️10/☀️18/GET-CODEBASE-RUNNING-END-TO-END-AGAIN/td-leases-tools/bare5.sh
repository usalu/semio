#!/bin/sh
# 🧪 [td-leases] runs the five lease test files bare (`bun test ./🟦️.ts` in each test directory) inside mirror $1 (sealed), optional SEMIO_TEST_ARTIFACT_DIR from the environment; prints one summary line per file, full output in C/🗑️generated/tl/<tag>-<name>.out. usage: bare5.sh <mirror> <tag>
C="$(cd "$(dirname "$0")/.." && pwd)"; G="$C/🗑️generated/tl"; mkdir -p "$G"; P="🧰️framework/🔨️modules/🏃️process/🔒️leases"
for t in "claims:$P/♻️retirement/🧪️tests/📋️claims" "frontier:$P/♻️retirement/🧪️tests/📋️frontier" "inventory:$P/♻️retirement/🧪️tests/📋️inventory" "sweep:$P/♻️retirement/🧹️sweep/🧪️tests" "writer:$P/🧪️tests/🎟️writer"; do
 n=${t%%:*}; d=${t#*:}
 python3 -I "$C/mirror-tools/mirror.py" run "$1" --peers-ok ${SEMIO_TEST_ARTIFACT_DIR:+--seal-allow ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tl"} --cwd "$d" -- bun test --timeout 170000 ./🟦️.ts > "$G/$2-$n.out" 2>&1
 echo "$n rc=$? $(grep -E '^ *[0-9]+ (pass|fail)$' "$G/$2-$n.out" | tr -s ' \n' ' ') $(grep -c 'GUARD FAIL' "$G/$2-$n.out") guard-fail"
 grep -E '^\(fail\)' "$G/$2-$n.out" | head -5 | cut -c1-200
done
