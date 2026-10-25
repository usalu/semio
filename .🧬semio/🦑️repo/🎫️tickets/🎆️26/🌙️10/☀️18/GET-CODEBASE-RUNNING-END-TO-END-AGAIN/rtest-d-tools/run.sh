#!/bin/sh
# usage: run.sh <run-id>   official Bazel run-many :test of the 8 repo-*-rs crates that now own cargo tests
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/rtest-d"
ID="$1"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
PROJECTS="@semio-tech/repo-codebase-rs,@semio-tech/repo-contributors-rs,@semio-tech/repo-goals-rs,@semio-tech/repo-metrics-rs,@semio-tech/repo-model-rs,@semio-tech/repo-statutes-rs,@semio-tech/repo-test-runner-rs,@semio-tech/repo-todos-rs"
date +%s > "$O/$ID.start"
nice -n 10 bun repo --tool=bazel run-many --projects="$PROJECTS" --targets=test --parallel=1 > "$O/$ID.log" 2>&1
echo $? > "$O/$ID.rc"
