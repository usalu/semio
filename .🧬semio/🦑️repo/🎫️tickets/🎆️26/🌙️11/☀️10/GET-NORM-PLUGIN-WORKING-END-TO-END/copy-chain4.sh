#!/usr/bin/env bash
# 🔁 Reruns every private-copy entry that ended on a killed compiler or on compile memory, after the other parts. Waits for copy-chain3.sh.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"; N="✏️s/🔌️plugins/📕️norm/🗿️artifacts"; P="📦️packages/🦀️rust"
export RUST_MIN_STACK=67108864
until grep -q '^done3' "$G/copy-summary.txt" 2>/dev/null; do sleep 30; done
for f in 🏋️en1991 🧱️din4108 🫨️en1998 ⚡️din18599 🌍️en1997 🌬️din16798 📇️iso16757 🔩️en1993 🧩️en1994 🪵️en1995 🪨️en1996 🏛️en1992; do
  fam="${f//[^a-z0-9]/}"; log="cp-mv-$fam"
  grep -q '^test result' "$G/$log.log" 2>/dev/null && continue
  export SEMIO_MUTATION_VECTORS_PRODUCED_DIR="$G/cp-produced-$fam"; mkdir -p "$SEMIO_MUTATION_VECTORS_PRODUCED_DIR"
  "$T/copy-test.sh" "$log" "$N/$f/$A/🧬️schema/🧬️mutations/$P" test -p "semio-s-artifact-norm-$fam-mutations" --test mutation_vectors --offline -j 1
  echo "$(date +%H:%M) $log rerun exit $? $(grep -a '^test result' "$G/$log.log" | tail -1) $(grep -aE '^error' "$G/$log.log" | head -1 | cut -c1-140)" >> "$G/copy-summary.txt"
done
echo done4 >> "$G/copy-summary.txt"
