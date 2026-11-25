#!/usr/bin/env bash
# 🔗 Second half of the private-copy backlog: insert tests, editors, en1994 checks, example crates. Waits for copy-chain.sh.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"; N="✏️s/🔌️plugins/📕️norm/🗿️artifacts"; P="📦️packages/🦀️rust"
export RUST_MIN_STACK=67108864
until grep -q '^done' "$G/copy-summary.txt" 2>/dev/null; do sleep 30; done
:
one() {
  "$T/copy-test.sh" "$@" ; code=$?
  echo "$(date +%H:%M) $1 exit $code $(grep -a '^test result' "$G/$1.log" | tail -3 | tr '\n' ' ') $(grep -aE '^error' "$G/$1.log" | head -2 | cut -c1-140 | tr '\n' ' ')" >> "$G/copy-summary.txt"
}
one cp-livecore "✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧲️live-core/$P" test -p semio-s-plugin-norm-app-surface-live-core --offline --no-fail-fast -j 3
for f in 🫨️en1998 🌍️en1997 🏛️en1992 ⚡️din18599 ⚖️en1990; do fam="${f//[^a-z0-9]/}"; one "cp-editor-$fam" "$N/$f/$A/✏️editor/$P" test -p "semio-s-artifact-norm-$fam-editor" --offline --no-fail-fast -j 3 -- --test-threads=4 --skip latency_bench; done
one cp-checks-en1994 "$N/🧩️en1994/$A/🧬️schema/💡️inferences/⚖️checks/$P" test -p semio-s-artifact-norm-en1994-checks -p semio-s-artifact-norm-en1994-inferences --offline --no-fail-fast -j 3
for f in 🌬️din16798 🧱️din4108 🏛️en1992 🫨️en1998 🏋️en1991; do fam="${f//[^a-z0-9]/}"; one "cp-examples-$fam" "$N/$f/$A/📚️examples/$P" test -p "semio-s-artifact-norm-$fam-examples" --offline --no-fail-fast -j 3; done
echo done2 >> "$G/copy-summary.txt"
