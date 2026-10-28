#!/bin/sh
# usage: clean-extra.sh <id> <wait-for-file> ; waits for the main clean set to finish, then runs the two extra named targets one at a time
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/integ-g"; id="$1"
while [ ! -f "$2" ]; do sleep 20; done
unset SEMIO_SCRIPT_STALL_MS; export SEMIO_LANGUAGE=en
run() { name="$1"; projects="$2"; targets="$3"; log="$G/$id.$name.log"; st=$(date +%H:%M:%S); s0=$(date +%s)
  nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$log" 2>&1; rc=$?
  printf '%s\t%s\tstart=%s\tend=%s\twall=%ss\trc=%s\tprojects=%s\ttargets=%s\n' "$id" "$name" "$st" "$(date +%H:%M:%S)" "$(( $(date +%s) - s0 ))" "$rc" "$projects" "$targets" >> "$G/clean.tsv"; }
run tickets "@semio-tech/repo-tickets-rs" "test"
run workspacego "@semio-tech/repo-workspace-go" "test"
echo done > "$G/$id.extra.finished"
