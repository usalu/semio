#!/usr/bin/env bash
# run_py.sh <module list file> <tag>: one unittest module per uv run, counts in generated/r3-valid/<tag>/counts.tsv
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/r3-valid/$2"
mkdir -p "$G/logs"
: > "$G/counts.tsv"
i=0
while IFS= read -r mod; do
  [ -z "$mod" ] && continue
  i=$((i+1))
  log="$G/logs/$(printf %02d "$i").log"
  start=$(date +%s)
  bash "$T/m9_tests.sh" "$mod" > "$log" 2>&1 < /dev/null
  rc=$?
  dur=$(( $(date +%s) - start ))
  ran=$(grep -E "^Ran [0-9]+ test" "$log" | tail -1 | awk '{print $2}')
  res=$(grep -E "^(OK|FAILED)" "$log" | tail -1)
  printf '%s\t%s\t%s\t%s\t%s\t%ss\n' "$i" "$mod" "$rc" "${ran:-none}" "${res:-no-result}" "$dur" >> "$G/counts.tsv"
done < "$1"
echo DONE >> "$G/counts.tsv"
