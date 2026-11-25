#!/usr/bin/env bash
# 🧪 Waits for copy-chain.sh, re-syncs the copy and compiles the changed shared crates before the second list starts; a failure leaves the list unstarted.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; N="✏️s/🔌️plugins/📕️norm"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"
until grep -q '^done$' "$G/copy-summary.txt" 2>/dev/null; do sleep 20; done
"$T/copy-sync.sh" > "$G/copy-sync2.log" 2>&1
"$T/copy-test.sh" cp-gate-compliance "$N/⚖️compliance/📦️packages/🦀️rust" test -p semio-s-plugin-norm-compliance --lib --offline -j 3; a=$?
"$T/copy-test.sh" cp-gate-diffs "$N/🗿️artifacts/🏛️en1992/$A/🧬️schema/🔺️diff/📦️packages/🦀️rust" check -p semio-s-artifact-norm-en1992-diff --offline -j 3; b=$?
"$T/copy-test.sh" cp-gate-diff-en1994 "$N/🗿️artifacts/🧩️en1994/$A/🧬️schema/🔺️diff/📦️packages/🦀️rust" check -p semio-s-artifact-norm-en1994-diff --offline -j 3; c=$?
echo "$(date +%H:%M) gate compliance=$a en1992-diff=$b en1994-diff=$c $(grep -ahE '^error' "$G"/cp-gate-*.log | sort -u | head -3 | cut -c1-140 | tr '\n' ' ')" >> "$G/copy-summary.txt"
[ "$a$b$c" = 000 ] || exit 1
exec "$T/copy-chain2.sh"
