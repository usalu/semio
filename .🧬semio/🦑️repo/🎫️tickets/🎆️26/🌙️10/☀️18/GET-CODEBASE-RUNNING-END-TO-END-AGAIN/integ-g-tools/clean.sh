#!/bin/sh
# usage: clean.sh <id> ; ONE clean full product set, explicit run-many per product so every log names every target; rows (start, newest gate landing at start, exit) in generated/integ-g/clean.tsv
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/integ-g"; id="$1"
unset SEMIO_SCRIPT_STALL_MS; export SEMIO_LANGUAGE=en
run() { name="$1"; projects="$2"; targets="$3"; log="$G/$id.$name.log"; st=$(date +%H:%M:%S); s0=$(date +%s)
  nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$log" 2>&1; rc=$?
  printf '%s\t%s\tstart=%s\tend=%s\twall=%ss\trc=%s\tprojects=%s\ttargets=%s\n' "$id" "$name" "$st" "$(date +%H:%M:%S)" "$(( $(date +%s) - s0 ))" "$rc" "$projects" "$targets" >> "$G/clean.tsv"; }
run course "@teaching/architecture-course" "build,test"
run dash "@semio-tech/repo-dashboard-rs" "build,test"
run printrs "@semio-tech/print-rs" "build,test"
run print "@semio-tech/print" "build"
echo done > "$G/$id.finished"
