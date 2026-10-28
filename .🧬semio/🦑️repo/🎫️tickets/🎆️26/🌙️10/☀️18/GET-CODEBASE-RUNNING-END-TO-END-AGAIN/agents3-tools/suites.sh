#!/bin/sh
# 🧪 agents16: runs every host-capacity law file (framework + library + project farm) one by one in <root> and writes the FAILURE SET (file :: test) to <out>; compare sets, not counts.
# Usage: sh suites.sh <root> <out-file>
ROOT=$1; OUT=$2; HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd); C=$(dirname "$HERE")
mkdir -p "$C/🗑️generated/agents3/t"; : > "$OUT"
cd "$ROOT" || exit 2
find "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm" -name "🟦️.ts" -path "*🧪️tests*" | grep -v "📜️child\|🧫️fixtures" | sort | while read -r f; do
  log=$(SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/agents3/t" bun test "./$f" 2>&1)
  pass=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) pass$/\1/p' | tail -1); fail=$(printf '%s\n' "$log" | sed -n 's/^ *\([0-9][0-9]*\) fail$/\1/p' | tail -1); pass=${pass:-0}; fail=${fail:-0}
  echo "[DEBUG] $pass pass $fail fail $f"
  printf '%s\n' "$log" | grep '^(fail)' | sed "s|^(fail) |$f :: |; s| \[[0-9.]*m\?s\]\$||" >> "$OUT"
  if [ "$fail" != 0 ] && ! printf '%s\n' "$log" | grep -q '^(fail)'; then echo "$f :: ($fail failed, unnamed) $(printf '%s\n' "$log" | grep -m1 -i 'error' | cut -c1-160)" >> "$OUT"; fi
  if [ "$pass" = 0 ] && [ "$fail" = 0 ]; then echo "$f :: (no test ran) $(printf '%s\n' "$log" | grep -m1 -i 'error' | cut -c1-160)" >> "$OUT"; fi
done
echo "[DEBUG] failures: $(wc -l < "$OUT")"
