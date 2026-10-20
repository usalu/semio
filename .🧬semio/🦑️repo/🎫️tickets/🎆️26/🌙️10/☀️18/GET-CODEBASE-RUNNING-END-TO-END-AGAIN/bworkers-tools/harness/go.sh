#!/bin/bash
# usage: go.sh <run-dir> <tasks> [VAR=value ...]   one Bazel build through the live-root scratch host; BW_GRAPH names the graph
cd /Users/ueli/Documents/semio || exit 2
H=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/bworkers-tools/harness"
base=$1; tasks=$2; shift 2
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 BW_BASE=$base BW_TASKS=$tasks BW_BAZEL=/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel
mkdir -p "$base"
for kv in "$@"; do export "$kv"; done
echo "$base/time.log" > /private/tmp/bworkers/current-log
exec perl -e 'alarm 560; exec @ARGV' -- bun "$H/run.ts" "$H/${BW_DRIVER:-go.ts}"
