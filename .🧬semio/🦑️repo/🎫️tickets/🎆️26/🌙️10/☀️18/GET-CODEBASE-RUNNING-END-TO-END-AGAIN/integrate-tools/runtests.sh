#!/bin/bash
# usage: runtests.sh <overlay-root> <list-file> <log-dir> [match]   one bun test process at a time, 280 s each; the summary of every file lands in <log-dir>/summary.txt
ov="$1"; list="$2"; log="$3"; match="${4:-}"
mkdir -p "$log"; : > "$log/summary.txt"; cd "$ov" || exit 2
n=0
while IFS= read -r rel; do
  [ -z "$rel" ] && continue
  case "$rel" in *"$match"*) ;; *) continue;; esac
  n=$((n+1)); out="$log/$(printf %02d $n).txt"
  if [ ! -f "$rel" ]; then echo "MISSING $rel" >> "$log/summary.txt"; continue; fi
  perl -e 'alarm 280; exec @ARGV' -- bun test "./$rel" > "$out" 2>&1; rc=$?
  line=$(grep -E "^ *[0-9]+ (pass|fail|skip)" "$out" | tr -s ' ' | tr '\n' ' ')
  echo "rc=$rc $(printf %02d $n) ${rel##*library/} :: $line" >> "$log/summary.txt"
done < "$list"
echo done >> "$log/summary.txt"
