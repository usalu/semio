#!/bin/zsh
# T3: runs test files one at a time with repo test-file; args: <log dir> <label>=<file> ...
out=$1; shift
cd /Users/ueli/Documents/semio
for pair in "$@"; do
  label=${pair%%=*}; file=${pair#*=}
  start=$(date +%s)
  repo test-file --lang=en "$file" > "$out/$label.log" 2>&1; code=$?
  summary=$(grep -E "^ *[0-9]+ (pass|fail)$" "$out/$label.log" | tr -s ' \n' ' ')
  echo "$label exit=$code $(( $(date +%s)-start ))s $summary load=$(sysctl -n vm.loadavg | cut -d' ' -f2)" >> "$out/summary.txt"
done
echo DONE >> "$out/summary.txt"
