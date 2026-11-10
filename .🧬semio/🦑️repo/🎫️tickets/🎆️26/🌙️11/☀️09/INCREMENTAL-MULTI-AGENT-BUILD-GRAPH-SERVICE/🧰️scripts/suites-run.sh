#!/usr/bin/env bash
cd /home/deck/semio; G="$1"
grep -v '👷️child\|📥️holder\|📥️worker\|flat-reference' "$G/list.txt" | tail -n +2 | grep -v '^/' > "$G/files.txt"
grep -v '👷️child\|📥️holder\|📥️worker\|flat-reference' "$G/list.txt" | grep -v '^/' | while read -r f; do
  out=$(timeout 300 repo tools exec --light bun -- test "./$f" 2>&1); 
  p=$(printf '%s' "$out" | grep -E '^ *[0-9]+ pass' | tail -1 | tr -s ' '); fl=$(printf '%s' "$out" | grep -E '^ *[0-9]+ fail' | tail -1 | tr -s ' ')
  echo "${p:-nopass}|${fl:-nofail}|${f#*build/}"; printf '%s\n' "$out" > "$G/out-$(echo "$f" | md5sum | cut -c1-8).txt"
done
