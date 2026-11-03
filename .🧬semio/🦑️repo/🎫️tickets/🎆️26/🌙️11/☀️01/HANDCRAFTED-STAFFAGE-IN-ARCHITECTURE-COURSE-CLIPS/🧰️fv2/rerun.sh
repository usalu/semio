#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/fv2"
: > "$G/rerun.tsv"
for i in 5 9 10; do
  mod=$(sed -n "${i}p" "$T/🧰️ht/modules.txt")
  log="$G/logs/rerun-m$i.en.log"
  SEMIO_TEACHING_LANGUAGE=en timeout 1500 bash "$T/m9_tests.sh" "$mod" > "$log" 2>&1 < /dev/null
  printf 'm%s\ten\t%s\t%s\t%s\n' "$i" "$?" "$(grep -E '^Ran [0-9]+ test' "$log" | tail -1 | awk '{print $2}')" "$(grep -E '^(OK|FAILED)' "$log" | tail -1)" >> "$G/rerun.tsv"
done
echo DONE >> "$G/rerun.tsv"
