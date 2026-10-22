#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated"
tool=$1; shift; export SEMIO_BUILD_GRAPH_POLICY=current
for step in "$@"; do
  name=$(echo "$step" | tr '@/:' '___')
  s=$(date +%s)
  case "$tool" in bazel) nice -n 10 bun repo --tool=bazel run $step > "$G/course-rest-$tool-$name.log" 2>&1;; *) nice -n 10 bun repo run $step > "$G/course-rest-$tool-$name.log" 2>&1;; esac
  echo "$step rc=$? secs=$(( $(date +%s)-s ))" >> "$G/course-rest-steps-$tool.txt"
done
