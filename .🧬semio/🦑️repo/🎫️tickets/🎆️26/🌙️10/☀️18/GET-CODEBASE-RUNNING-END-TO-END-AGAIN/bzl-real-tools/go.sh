#!/bin/bash
# usage: go.sh <label> <comma-separated task ids> [extra bazel options]   (one Bazel build through the staged scratch-host owner; log + execution log kept per label)
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/bzl-real"
label=$1; tasks=$2; shift 2
export LANG=en_US.UTF-8
first=${tasks%%,*}
BZL_REAL_EDGES="${EDGES:-}" BZL_REAL_TASKS=$tasks BZL_BREADTH_SELECTION="run $first" BZL_REAL_OPTIONS="--execution_log_json_file=/private/tmp/bzlreal/exec-$label.json $*" perl -e 'alarm 570; exec @ARGV' -- bun "$C/bzl-real-tools/run.ts" "$C/bzl-real-tools/real.ts" > "$G/$label.log" 2>&1
echo "exit=$?"
grep -v "resource-directory-admission\|\"stage\":\"running\"\|running elapsedMs" "$G/$label.log" | grep -E "INFO|WARNING|ERROR|^\{|processes|Elapsed|cache|up-to-date" | cut -c1-400
