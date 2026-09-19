#!/bin/zsh
# Runs each crate's backbone/convergence/idempotence tests sequentially; one summary line per crate and filter.
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/EVENT-SOURCED-FRAMEWORK-VERSION-CONTROL-END-TO-END"
OUT="$T/🗑️generated/m-final-results.txt"
: > "$OUT"
while read -r crate; do
  manifest=$(rg -l "^name = \"$crate\"" -g 'Cargo.toml' ✏️s | head -1)
  features=""
  if grep -q '^component-app-assembly' "$manifest" 2>/dev/null; then features="--features component-app-assembly"; fi
  for filter in converge idempotent; do
    log="$T/🗑️generated/m-$crate-$filter.txt"
    cargo test -p "$crate" $=features --lib "$filter" > "$log" 2>&1
    summary=$(grep -E "^test result" "$log" | tail -1)
    [ -z "$summary" ] && summary="NO RESULT: $(grep -E '^error' "$log" | head -1)"
    echo "$crate $filter :: $summary" >> "$OUT"
    grep -E "^test .* FAILED$" "$log" | sed "s/^/    /" >> "$OUT"
  done
done < "$T/🗑️generated/m-final-crates.txt"
echo DONE >> "$OUT"
