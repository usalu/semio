#!/bin/sh
# runs the bundle law and the live probe in two separate exec envelopes (one bun process shares one build-test receiver per run)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
R="/Users/ueli/Documents/semio"
export SLOT_D_REPS=15 SLOT_D_TARGETS=3
sh "$C/slot-d-tools/run-exec.sh" "$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🧪️tests/🔬️concurrency/🟦️.ts" > "$C/🗑️generated/slot-d/exec2.log" 2>&1
sh "$C/slot-d-tools/run-exec.sh" "$C/slot-d-tools/bun-overlap-live.test.ts" > "$C/🗑️generated/slot-d/exec3.log" 2>&1
