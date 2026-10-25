#!/bin/sh
# usage: exec-test.sh <name> <project> [script args...]  (runs the project's script directly, no Bazel verdict)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; p=$2; shift 2
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=semio exec --projects="$p" -- repo ./📜️script.ts "$@" > "$C/🗑️generated/pres-d/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/pres-d/$n.raw"
