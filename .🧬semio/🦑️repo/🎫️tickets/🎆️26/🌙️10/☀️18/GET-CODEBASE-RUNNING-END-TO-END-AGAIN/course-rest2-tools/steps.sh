#!/bin/sh
# usage: steps.sh <nx|bazel> <project:target>...   logs 🗑️generated/course-rest2/<tool>-<step>.log, rc list steps-<tool>.txt
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/course-rest2"
tool=$1; shift; export SEMIO_BUILD_GRAPH_POLICY=${SEMIO_BUILD_GRAPH_POLICY:-current}
for step in "$@"; do
  name=$(echo "$step" | tr '@/:' '___')
  s=$(date +%s)
  case "$tool" in bazel) nice -n 10 bun repo --tool=bazel run $step > "$G/$tool-$name.log" 2>&1;; *) nice -n 10 bun repo run $step > "$G/$tool-$name.log" 2>&1;; esac
  echo "$step rc=$? secs=$(( $(date +%s)-s ))" >> "$G/steps-$tool.txt"
done
