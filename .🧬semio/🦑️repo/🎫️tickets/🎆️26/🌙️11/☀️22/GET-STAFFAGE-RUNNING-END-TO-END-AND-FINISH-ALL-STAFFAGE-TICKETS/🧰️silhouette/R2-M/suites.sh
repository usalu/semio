#!/bin/zsh
# 🧾️ The asset's tests of the further suites by name filter, one process after the other, one line per suite: suites.sh <tag> <id>…
source "${0:A:h}/env.sh"
tag="$1"; shift
for a in "$@"; do
  for suite in 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog ✅️asset-validation ⚡️asset-power; do
    echo "$a | $suite | $(SUFFIX= ./vt.sh "$tag-$suite-$a" "$suite" "$a")"
  done
done
