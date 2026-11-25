#!/usr/bin/env bash
# 🔁 After the other parts: re-syncs the copy, reruns en1992 and every mutation family without a passing result, then the schema crates. Waits for copy-chain4.sh.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"; N="✏️s/🔌️plugins/📕️norm/🗿️artifacts"; P="📦️packages/🦀️rust"
export RUST_MIN_STACK=67108864
until grep -q '^done4' "$G/copy-summary.txt" 2>/dev/null; do sleep 30; done
"$T/copy-sync.sh" > "$G/copy-sync5.log" 2>&1
one() { "$T/copy-test.sh" "$@"; code=$?; echo "$(date +%H:%M) $1 exit $code $(grep -a '^test result' "$G/$1.log" | tail -2 | tr '\n' ' ') $(grep -aE '^error' "$G/$1.log" | head -1 | cut -c1-140)" >> "$G/copy-summary.txt"; }
for f in 🏛️en1992 🏋️en1991 🧱️din4108 🫨️en1998 ⚡️din18599 🌍️en1997 🌬️din16798 📇️iso16757 🔩️en1993 🧩️en1994 🪵️en1995 🪨️en1996; do
  fam="${f//[^a-z0-9]/}"; grep -q "test result: ok" "$G/cp-mv-$fam.log" 2>/dev/null && [ "$fam" != en1992 ] && continue
  export SEMIO_MUTATION_VECTORS_PRODUCED_DIR="$G/cp-produced-$fam-2"; mkdir -p "$SEMIO_MUTATION_VECTORS_PRODUCED_DIR"
  one "cp2-mv-$fam" "$N/$f/$A/🧬️schema/🧬️mutations/$P" test -p "semio-s-artifact-norm-$fam-mutations" --test mutation_vectors --offline -j 2
done
unset SEMIO_MUTATION_VECTORS_PRODUCED_DIR
for f in 🧱️din4108 🌬️din16798 ⚡️din18599 📇️iso16757 🏋️en1991 🏛️en1992 🔩️en1993 🧩️en1994 🪵️en1995 🪨️en1996 🫨️en1998 🪶️en1999; do fam="${f//[^a-z0-9]/}"; one "cp-schema-$fam" "$N/$f/$A/🧬️schema/$P" test -p "semio-s-artifact-norm-$fam-schema" --offline -j 2; done
echo done5 >> "$G/copy-summary.txt"
