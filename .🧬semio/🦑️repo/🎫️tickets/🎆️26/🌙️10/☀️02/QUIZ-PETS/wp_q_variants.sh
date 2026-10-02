#!/usr/bin/env bash
# Work package Q: compares Vitest variants of the pets core suite by alternating them, so a changing host load hits all alike.
# Prints per run the variant, the wall time of the package script and Vitest's own duration line; at the end the median wall per variant.
# Usage: bash wp_q_variants.sh <rounds> "<variant arguments>" "<variant arguments>" …   (an empty string is the package as it is)
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
package="$ticket/../../../../../../../🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript"
rounds="$1"
shift
out="$ticket/🗑️generated/wp-q/variants.txt"
mkdir -p "$(dirname "$out")"
cd "$package" || exit 2
declare -A walls
for round in $(seq 1 "$rounds"); do
  for variant in "$@"; do
    key="v:$variant"
    start=$(date +%s%N)
    # shellcheck disable=SC2086
    bun ./📜️script.ts test $variant > "$out" 2>&1
    code=$?
    wall=$(( ($(date +%s%N) - start) / 1000000 ))
    walls["$key"]="${walls["$key"]:-} $wall"
    if [ "$code" -ne 0 ]; then cp "$out" "$ticket/🗑️generated/wp-q/variants-failed-$round-$(echo "$variant" | tr -c 'a-zA-Z0-9\n' '_').txt"; fi
    echo "[$variant] exit=$code wall=${wall}ms $(grep -E 'Duration|\[budget\]|AssertionError|FAIL' "$out" | sed -E 's/ +/ /g' | cut -c1-160 | tr '\n' '|')"
  done
done
for variant in "$@"; do
  key="v:$variant"
  sorted=$(echo ${walls["$key"]} | tr ' ' '\n' | sort -n | tr '\n' ' ')
  median=$(echo $sorted | awk '{ print $(int((NF + 1) / 2)) }')
  echo "median [$variant] ${median}ms of: $sorted"
done
