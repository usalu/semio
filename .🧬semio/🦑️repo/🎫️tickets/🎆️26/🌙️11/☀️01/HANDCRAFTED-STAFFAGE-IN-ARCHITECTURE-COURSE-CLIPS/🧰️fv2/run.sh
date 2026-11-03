#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/fv2"
M="$T/🧰️ht/modules.txt"
V="🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests"
mkdir -p "$G/logs"
: > "$G/counts.tsv"

run_one() {
  local label="$1" lang="$2" mod="$3" log start rc dur ran res
  log="$G/logs/$label.$lang.log"
  start=$(date +%s)
  SEMIO_TEACHING_LANGUAGE="$lang" timeout 1500 bash "$T/m9_tests.sh" "$mod" > "$log" 2>&1 < /dev/null
  rc=$?
  dur=$(( $(date +%s) - start ))
  ran=$(grep -E "^Ran [0-9]+ test" "$log" | tail -1 | awk '{print $2}')
  res=$(grep -E "^(OK|FAILED)" "$log" | tail -1)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%ss\n' "$label" "$lang" "$rc" "${ran:-none}" "${res:-no-result}" "$mod" "$dur" >> "$G/counts.tsv"
}

for lang in en de; do
  run_one guard "$lang" "$V.🐍️staffage_only"
  run_one labels "$lang" "$V.🐍️labels"
done
for lang in en de; do
  for i in $(seq 1 35); do
    mod=$(sed -n "${i}p" "$M")
    [ "$lang" = de ] && [ "$i" -gt 15 ] && [ "$i" -ne 18 ] && continue
    run_one "m$(printf %02d "$i")" "$lang" "$mod"
  done
  run_one hydronic "$lang" "🎓️teaching.🏛️architecture.🧍️staffage.🧪️tests.🚰️hydronic-coupling.🐍️"
done
echo DONE >> "$G/counts.tsv"
