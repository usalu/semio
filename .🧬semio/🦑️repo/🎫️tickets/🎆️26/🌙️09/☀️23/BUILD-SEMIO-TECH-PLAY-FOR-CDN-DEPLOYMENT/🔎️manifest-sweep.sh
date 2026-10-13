#!/bin/bash
# Checks every plugin artifact and extension manifest natively (lib) and records failing manifests.
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
OUT="$T/🗑️generated/manifest-sweep.tsv"; : > "$OUT"
export CARGO_TARGET_DIR=".🧬semio/🦑️repo/⚡️cache/sweep/target"
find ✏️s/🔌️plugins -path '*/📦️packages/🦀️rust/Cargo.toml' -not -path '*/target/*' | sort | while read -r m; do
  f=""; grep -q '^component-app-assembly' "$m" && f="--features component-app-assembly"
  log=$(bash "$T/🚦️cargo-slot.sh" sweep -- cargo check --manifest-path "$m" --lib $f --message-format short 2>&1)
  if echo "$log" | grep -q "^error\|: error"; then printf "FAIL\t%s\t%s\n" "$m" "$(echo "$log" | grep -m1 ': error' | cut -c1-200)" >> "$OUT"; else printf "ok\t%s\n" "$m" >> "$OUT"; fi
done
echo done >> "$OUT"
