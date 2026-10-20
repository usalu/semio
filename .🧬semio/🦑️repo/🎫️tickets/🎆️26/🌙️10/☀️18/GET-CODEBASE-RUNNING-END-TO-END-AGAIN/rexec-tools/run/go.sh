#!/bin/bash
# usage: go.sh <label> <comma-separated task ids> [extra bazel options]   (one Bazel build through the scratch-host owner; ASCII base /private/tmp/rexec/<label>; REXEC_* env knobs)
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/rexec"
label=$1; tasks=$2; shift 2
export LANG=en_US.UTF-8
export REXEC_BASE=/private/tmp/rexec/$label
export REXEC_GRAPH=${REXEC_GRAPH:-$C/🗑️generated/bzl-breadth/graph.json}
export REXEC_TASKS=$tasks
export REXEC_SELECTION="run ${tasks%%,*}"
export REXEC_OPTIONS="--execution_log_json_file=/private/tmp/rexec/exec-$label.json $*"
mkdir -p "$G" /private/tmp/rexec
perl -e 'alarm 570; exec @ARGV' -- bun "$C/rexec-tools/run/run.ts" "$C/rexec-tools/run/drive.ts" > "$G/$label.log" 2>&1
echo "exit=$?"
grep -v "resource-directory-admission\|\"stage\":\"running\"\|running elapsedMs" "$G/$label.log" | grep -E "INFO|WARNING|ERROR|^\{|processes|Elapsed|cache|up-to-date|rexec|error" | cut -c1-400
