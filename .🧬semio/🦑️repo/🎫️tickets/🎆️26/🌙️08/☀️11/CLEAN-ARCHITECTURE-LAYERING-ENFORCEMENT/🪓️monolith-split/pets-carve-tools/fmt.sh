#!/bin/bash
cd /Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets
MODE=$1
for f in 🔨️modules/*/🦀️.rs 🔨️modules/*/🧪️tests/🔬️unit/🦀️.rs 🧬️schema/🦀️.rs 🧬️schema/🧪️tests/🧰️testkit/🦀️.rs 🦀️.rs; do
  case "$f" in "🔨️modules/🎪️stage/🧪️tests/🔬️unit/🦀️.rs"|"🧬️schema/🧪️tests/🔬️unit/🦀️.rs") continue;; esac
  if [ "$MODE" = check ]; then
    r=$(rustfmt +nightly-2026-07-20 --edition 2021 --check --config skip_children=true,unstable_features=true "$f" 2>&1 | head -5)
    [ -n "$r" ] && echo "DIRTY $f" && echo "$r"
  else
    rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true,unstable_features=true "$f" 2>&1 | head -3
  fi
done
echo done-$MODE
