#!/bin/bash
# usage: go.sh <label> <comma-separated fixture task ids> [extra bazel options]   one phase through the overlay's scratch-host owner; logs under /private/tmp/bzlcov/logs
cd /private/tmp/bzlcov-ov || exit 2
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
mkdir -p /private/tmp/bzlcov/logs
label=$1; tasks=$2; shift 2
export LANG=en_US.UTF-8 BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
BZL_COV_TASKS=$tasks BZL_COV_OPTIONS="--action_env=BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 --test_env=BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 --execution_log_json_file=/private/tmp/bzlcov/logs/exec-$label.json $*" perl -e 'alarm 570; exec @ARGV' -- bun "$C/bzl-coverage-tools/scratch-run.ts" > "/private/tmp/bzlcov/logs/$label.log" 2>&1
echo "exit=$?"
command grep -v "resource-directory-admission\|\"stage\":\"running\"\|running elapsedMs\|\[script\]" "/private/tmp/bzlcov/logs/$label.log" | command grep -E "INFO|WARNING|ERROR|^\{|processes|Elapsed|cache|up-to-date|PASSED|FAILED|NO STATUS|error|Error|\[bazel\]|Executed|Test" | cut -c1-300 | head -${LINES_MAX:-40}
