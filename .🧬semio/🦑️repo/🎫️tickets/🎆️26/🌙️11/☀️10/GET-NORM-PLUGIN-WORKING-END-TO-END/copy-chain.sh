#!/usr/bin/env bash
# 🔗 Runs the Norm test backlog in the private copy, one cargo at a time, and appends one summary line per entry.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"; N="✏️s/🔌️plugins/📕️norm/🗿️artifacts"; P="📦️packages/🦀️rust"
export RUST_MIN_STACK=67108864 SEMIO_PYTHON="${SEMIO_PYTHON:-}"
one() { # log dir args...
  "$T/copy-test.sh" "$@" ; code=$?
  echo "$(date +%H:%M) $1 exit $code $(grep -a '^test result' "$G/$1.log" | tail -3 | tr '\n' ' ') $(grep -aE '^error' "$G/$1.log" | head -2 | cut -c1-140 | tr '\n' ' ')" >> "$G/copy-summary.txt"
}
NM="${1:-all}"
if [ "$NM" = all ] || [ "$NM" = mv ]; then
  for f in 🏛️en1992 🏋️en1991 🧱️din4108 🫨️en1998 ⚡️din18599 🌍️en1997 🌬️din16798 📇️iso16757 🔩️en1993 🧩️en1994 🪵️en1995 🪨️en1996; do
    fam="${f//[^a-z0-9]/}"; export SEMIO_MUTATION_VECTORS_PRODUCED_DIR="$G/cp-produced-$fam"; mkdir -p "$SEMIO_MUTATION_VECTORS_PRODUCED_DIR"; one "cp-mv-$fam" "$N/$f/$A/🧬️schema/🧬️mutations/$P" test -p "semio-s-artifact-norm-$fam-mutations" --test mutation_vectors --offline -j 3
  done
fi
if [ "$NM" = all ] || [ "$NM" = carve ]; then
  one cp-carve-en1992 "$N/🏛️en1992/$A/🧬️schema/💡️inferences/⚖️checks/$P" test -p semio-s-artifact-norm-en1992-checks -p semio-s-artifact-norm-en1992-checks-reach-and-trace --offline --no-fail-fast -j 3
  one cp-carve-en1993 "$N/🔩️en1993/$A/🧬️schema/💡️inferences/⚖️checks/$P" test -p semio-s-artifact-norm-en1993-checks -p semio-s-artifact-norm-en1993-checks-laws --offline --no-fail-fast -j 3
  one cp-carve-en1995 "$N/🪵️en1995/$A/🧬️schema/⚖️kernel/$P" test -p semio-s-artifact-norm-en1995-kernel --offline --no-fail-fast -j 3
  one cp-carve-en1998 "$N/🫨️en1998/$A/🧬️schema/📐️formulas/$P" test -p semio-s-artifact-norm-en1998-schema-formulas -p semio-s-artifact-norm-en1998-schema-formulas-worked-examples --offline --no-fail-fast -j 3
  one cp-carve-vdi3805 "$N/🏭️vdi3805/$A/🧬️schema/💡️inferences/$P" test -p semio-s-artifact-norm-vdi3805-inferences -p semio-s-artifact-norm-vdi3805-inferences-laws --offline --no-fail-fast -j 3
fi
if [ "$NM" = all ] || [ "$NM" = scene ]; then
  one cp-scene-en1994 "$N/🧩️en1994/$A/🎬️scene/$P" test -p semio-s-artifact-norm-en1994-scene --offline --no-fail-fast -j 3 -- --nocapture zz_debug_probe
  for f in 🏋️en1991 🏛️en1992 🪨️en1996 🌍️en1997 🧱️din4108 🌬️din16798 🧩️en1994; do fam="${f//[^a-z0-9]/}"; one "cp-scene-$fam" "$N/$f/$A/🎬️scene/$P" test -p "semio-s-artifact-norm-$fam-scene" --offline --no-fail-fast -j 3; done
fi
echo done >> "$G/copy-summary.txt"
