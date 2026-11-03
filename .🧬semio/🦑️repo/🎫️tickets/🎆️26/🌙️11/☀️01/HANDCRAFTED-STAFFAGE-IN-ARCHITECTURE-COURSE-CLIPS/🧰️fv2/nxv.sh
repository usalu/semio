#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/fv2"
until grep -q DONE "$G/nx.tsv"; do sleep 30; done
: > "$G/nxv.tsv"
cd "$W/🎓️teaching"
F="🏛️architecture/🏢️nakagin/🧪️tests/🎨️visuals/🐍️.py"
mod="🎓️teaching.🏛️architecture.🏢️nakagin.🧪️tests.🎨️visuals.🐍️.VisualContracts"
cd "$W"
for m in $(grep -oE "^    def test_[A-Za-z0-9_]+" "$W/🎓️teaching/$F" | awk '{print $2}'); do
  log="$G/logs/nxv-$m.log"
  start=$(date +%s)
  SEMIO_TEACHING_LANGUAGE=en timeout 1500 bash "$T/m9_tests.sh" "$mod.$m" > "$log" 2>&1 < /dev/null
  rc=$?
  printf '%s\t%s\t%s\t%s\t%ss\n' "$m" "$rc" "$(grep -E '^Ran [0-9]+ test' "$log" | tail -1 | awk '{print $2}')" "$(grep -E '^(OK|FAILED)' "$log" | tail -1)" "$(( $(date +%s) - start ))" >> "$G/nxv.tsv"
done
echo DONE >> "$G/nxv.tsv"
