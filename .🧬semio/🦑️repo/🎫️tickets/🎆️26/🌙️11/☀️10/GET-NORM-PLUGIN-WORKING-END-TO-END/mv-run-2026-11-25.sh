#!/usr/bin/env bash
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
G="$T/🗑️generated"
mkdir -p "$G"
for family in "$@"; do
  mkdir -p "$G/produced-$family"
  D=$(ls -d "$R"/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*"$family"/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust)
  ( cd "$D" && SEMIO_MUTATION_VECTORS_PRODUCED_DIR="$G/produced-$family" python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- test -p "semio-s-artifact-norm-$family-mutations" --test mutation_vectors --offline > "$G/mv-1125-$family.log" 2>&1; echo "exit $?" >> "$G/mv-1125-$family.log" )
done
echo ALLDONE >> "$G/mv-1125-status.log"
