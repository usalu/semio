#!/usr/bin/env bash
# usage: wave-vdi-check.sh <logname>
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
cd "$R/🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust"
P=()
for c in "" -checks -diff -editor -examples -examples-failing -examples-failing-blatt-4-to-19 -inferences -io -io-text -model -mut-file -mut-geometry -mut-geometry-more -mut-products -mutations -scene -schema -viewer; do P+=(-p "semio-s-artifact-norm-vdi3805$c"); done
repo cargo --store=shared --configuration=prototype -- check --keep-going --target wasm32-wasip2 "${P[@]}" > "$T/🗑️generated/$1.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/$1.log"
