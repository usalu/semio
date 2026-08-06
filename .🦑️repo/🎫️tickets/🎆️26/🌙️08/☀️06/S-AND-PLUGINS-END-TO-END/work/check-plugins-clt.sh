#!/usr/bin/env bash
set -uo pipefail
export DEVELOPER_DIR=/Library/Developer/CommandLineTools
export SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk
ROOT="/Users/ueli/Documents/semio"
TICKET=$(find "$ROOT/.🦑️repo/🎫️tickets" -type d -name 'S-AND-PLUGINS-END-TO-END' | head -1)
OUT="$TICKET/work/cargo-check-clt"
mkdir -p "$OUT"
cd "$ROOT"
PKGS=$(rg -l '^name\s*=\s*"semio-s' ✏️s --glob 'Cargo.toml' | while read f; do
  rg -o '^name\s*=\s*"([^"]+)"' -r '$1' "$f" | head -1
done | sort -u)
echo "packages: $(echo "$PKGS" | wc -l)" | tee "$OUT/summary.txt"
OK=0; FAIL=0
while IFS= read -r pkg; do
  [ -z "$pkg" ] && continue
  safe=$(echo "$pkg" | tr '/:' '__')
  echo "=== $pkg ===" | tee -a "$OUT/summary.txt"
  if cargo check -p "$pkg" --message-format=short >"$OUT/$safe.out" 2>"$OUT/$safe.err"; then
    echo "OK $pkg" | tee -a "$OUT/summary.txt"
    OK=$((OK+1))
  else
    echo "FAIL $pkg" | tee -a "$OUT/summary.txt"
    FAIL=$((FAIL+1))
    # keep real rustc errors, drop xcode noise
    rg -n 'error(\[E[0-9]+\])?:' "$OUT/$safe.err" | head -40 >> "$OUT/summary.txt" || head -40 "$OUT/$safe.err" >> "$OUT/summary.txt"
  fi
done <<< "$PKGS"
echo "DONE ok=$OK fail=$FAIL" | tee -a "$OUT/summary.txt"
