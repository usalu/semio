#!/usr/bin/env bash
N="$1"
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
S="$W/🎓️teaching/🏛️architecture/🧍️staffage"
G="$T/🗑️generated/cr/round-$N"
M="$T/🧰️ht/modules.txt"
mkdir -p "$G/logs"
: > "$G/counts.tsv"

run_mod() {
  local label="$1" idx="$2" mod log start rc dur ran res
  mod=$(sed -n "${idx}p" "$M")
  log="$G/logs/$label.log"
  start=$(date +%s)
  bash "$T/m9_tests.sh" "$mod" > "$log" 2>&1 < /dev/null
  rc=$?
  dur=$(( $(date +%s) - start ))
  ran=$(grep -E "^Ran [0-9]+ test" "$log" | tail -1 | awk '{print $2}')
  res=$(grep -E "^(OK|FAILED)" "$log" | tail -1)
  printf '%s\t%s\t%s\t%s\t%s\t%ss\n' "$label" "$idx" "$rc" "${ran:-none}" "${res:-no-result}" "$dur" >> "$G/counts.tsv"
}

# (a) visuals tests, incl. staffage_only and integration (list lines 6-15)
for i in $(seq 6 15); do run_mod "a$(printf %02d "$i")" "$i"; done
# (b) staffage Python package tests (list line 5)
run_mod "b05" 5
# (c) vitest, staffage library then React, ticket-local alias configs
cd "$S/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-alias.config.ts" > "$G/vitest-lib.log" 2>&1 < /dev/null
echo "lib rc=$?" >> "$G/vitest-status.txt"
cd "$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" && repo tools exec bun -- x vitest run --config "$T/q2-vitest-react-alias.config.ts" > "$G/vitest-react.log" 2>&1 < /dev/null
echo "react rc=$?" >> "$G/vitest-status.txt"
# (d) full catalog check, no arguments
cd "$S" && repo tools exec bun -- "./🔨️modules/🗂️catalog/🩺️check/🟦️.ts" > "$G/catalog.log" 2>&1 < /dev/null
echo "catalog rc=$?" >> "$G/vitest-status.txt"
# (e) remaining energy Python test modules, minus the four without tests (27, 33, 34, 35) and minus (a)/(b) already run
for i in 1 2 3 4 $(seq 16 26) 28 29 30 31 32; do run_mod "e$(printf %02d "$i")" "$i"; done
echo DONE >> "$G/counts.tsv"
echo DONE >> "$G/vitest-status.txt"
