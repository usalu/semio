#!/bin/sh
# usage: run.sh <run-id>   official Bazel run-many :test of the metrics Rust crate and its Go twin
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/metrics-d"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
nice -n 10 bun repo --tool=bazel run-many --projects=@semio-tech/repo-metrics-rs,@semio-tech/repo-metrics-go --targets=test --parallel=1 > "$O/$1.log" 2>&1
echo $? > "$O/$1.rc"
