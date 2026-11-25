#!/usr/bin/env bash
# 🔗 Third part of the private-copy backlog: the en1993 core checks and both editor carves. Waits for copy-chain2.sh.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"; N="✏️s/🔌️plugins/📕️norm/🗿️artifacts"; P="📦️packages/🦀️rust"
export RUST_MIN_STACK=67108864
until grep -q '^done2' "$G/copy-summary.txt" 2>/dev/null; do sleep 30; done
one() {
  "$T/copy-test.sh" "$@" ; code=$?
  echo "$(date +%H:%M) $1 exit $code $(grep -a '^test result' "$G/$1.log" | tail -3 | tr '\n' ' ') $(grep -aE '^error' "$G/$1.log" | head -2 | cut -c1-140 | tr '\n' ' ')" >> "$G/copy-summary.txt"
}
one cp-carve-en1993-core "$N/🔩️en1993/$A/🧬️schema/💡️inferences/⚖️checks/🧬️core/$P" test -p semio-s-artifact-norm-en1993-checks-core --offline --no-fail-fast -j 3
one cp-carve-editor-en1993 "$N/🔩️en1993/$A/✏️editor/$P" test -p semio-s-artifact-norm-en1993-editor -p semio-s-artifact-norm-en1993-editor-laws --offline --no-fail-fast -j 3
one cp-carve-editor-en1994 "$N/🧩️en1994/$A/✏️editor/$P" test -p semio-s-artifact-norm-en1994-editor -p semio-s-artifact-norm-en1994-editor-laws --offline --no-fail-fast -j 3
echo done3 >> "$G/copy-summary.txt"
