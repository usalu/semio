#!/usr/bin/env bash
# 🔁 Reruns the splits whose sources were fixed after the first copy, waiting for copy-chain5.sh (which re-syncs the copy).
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"; N="✏️s/🔌️plugins/📕️norm/🗿️artifacts"; P="📦️packages/🦀️rust"
export RUST_MIN_STACK=67108864
until grep -q '^done5' "$G/copy-summary.txt" 2>/dev/null; do sleep 30; done
one() { "$T/copy-test.sh" "$@"; code=$?; echo "$(date +%H:%M) $1 exit $code $(grep -a '^test result' "$G/$1.log" | tail -3 | tr '\n' ' ') $(grep -aE '^error' "$G/$1.log" | head -2 | cut -c1-140 | tr '\n' ' ')" >> "$G/copy-summary.txt"; }
one cp2-carve-en1992 "$N/🏛️en1992/$A/🧬️schema/💡️inferences/⚖️checks/$P" test -p semio-s-artifact-norm-en1992-checks -p semio-s-artifact-norm-en1992-checks-reach-and-trace --offline --no-fail-fast -j 2
one cp2-carve-en1993 "$N/🔩️en1993/$A/🧬️schema/💡️inferences/⚖️checks/$P" test -p semio-s-artifact-norm-en1993-checks -p semio-s-artifact-norm-en1993-checks-laws --offline --no-fail-fast -j 2
one cp2-carve-en1998 "$N/🫨️en1998/$A/🧬️schema/📐️formulas/$P" test -p semio-s-artifact-norm-en1998-schema-formulas -p semio-s-artifact-norm-en1998-schema-formulas-worked-examples --offline --no-fail-fast -j 2
echo done6 >> "$G/copy-summary.txt"
