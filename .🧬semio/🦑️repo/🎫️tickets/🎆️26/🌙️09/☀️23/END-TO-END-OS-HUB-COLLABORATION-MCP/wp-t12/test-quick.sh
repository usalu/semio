#!/bin/zsh
# 🧪️ Re-verifies per-plugin `test quick` (os-frontend audit P1-7) for the plugins red or timed out on 09-23, one at a time.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t12/target
for p in "$@"; do
  out=".tmp-ticket/wp-t12/generated/test-quick-${p}.txt"
  dir=$(ls -d ✏️s/🔌️plugins/*${p}/📦️packages/🦀️rust | head -1)
  start=$(date +%s)
  ( cd "$dir" && bun ./📜️script.ts test quick ) > "$out" 2>&1
  code=$?
  echo "EXIT=$code SECONDS=$(( $(date +%s) - start ))" >> "$out"
  echo "$p EXIT=$code"
done
echo ALL_DONE
