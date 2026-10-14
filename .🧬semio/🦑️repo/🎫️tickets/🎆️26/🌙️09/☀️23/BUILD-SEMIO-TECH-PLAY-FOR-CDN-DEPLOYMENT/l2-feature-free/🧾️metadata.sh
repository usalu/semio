#!/bin/bash
# 🧾️ Resolves every given Cargo workspace offline and locked (validates feature tables, optional deps and lockfiles without compiling).
# usage: 🧾️metadata.sh <workspace Cargo.toml>...
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
fail=0
for manifest in "$@"; do
  out=$(bash "$T/l2-feature-free/🦀️cargo.sh" 300 metadata --offline --locked --format-version 1 --manifest-path "$manifest" 2>&1 >/dev/null)
  if [ $? -ne 0 ]; then fail=1; echo "FAIL $manifest"; echo "$out" | tail -4; else echo "ok   $manifest"; fi
done
exit $fail
