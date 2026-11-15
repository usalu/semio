#!/bin/sh
# usage: run-all-tests.sh <module root> <log file>   runs every bun test file below every 🧪️tests directory of the module, sequentially
ROOT="$1"; LOG="$2"; BT=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/D/bt.sh
: > "$LOG"
find "$ROOT" -type d -name "🧪️tests" -not -path "*/node_modules/*" | sort | while read -r dir; do
  for f in "$dir"/*.ts; do
    [ -f "$f" ] || continue
    grep -q 'from "bun:test"' "$f" || continue
    name=$(basename "$f")
    start=$(date +%s)
    out=$("$BT" "$dir" "$name" 2>&1); code=$?
    end=$(date +%s)
    summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' ')
    printf '%s | %s | exit %s | %ss | %s\n' "${dir#$ROOT/}" "$name" "$code" "$((end-start))" "$summary" >> "$LOG"
    [ "$code" = 0 ] || printf '%s\n' "$out" | grep -E "^\(fail\)|error:" | head -8 >> "$LOG"
  done
done
echo DONE >> "$LOG"
