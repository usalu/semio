#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/mcp-b"
while kill -0 "$1" 2>/dev/null; do sleep 10; done
run() { name="$1"; shift; echo "== $name $(date +%H:%M:%S)" >> "$G/chain.log"; "$@" > "$G/$name.log" 2>&1; echo "rc=$? $name $(date +%H:%M:%S)" >> "$G/chain.log"; }
run repo-mcp-test nice -n 10 repo --tool=bazel run repo-mcp:test
run coordinator-rs-test nice -n 10 repo --tool=bazel run @semio-tech/repo-coordinator-rs:test
echo "chain done $(date +%H:%M:%S)" >> "$G/chain.log"
