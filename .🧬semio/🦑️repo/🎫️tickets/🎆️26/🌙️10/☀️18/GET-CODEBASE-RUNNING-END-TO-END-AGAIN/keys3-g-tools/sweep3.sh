#!/bin/bash
# keys3-g: after the earlier queues, ONE oracle run of architecture-course:build with the simulated key of the batch (old key + undeclared-g's simulated fixes, minus the batch's negations, plus the declared Tailwind scan region). A denial inside an excluded directory would mean the build reads a file the negations drop from the key.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; U="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"; G3="$C/🗑️generated/keys3-g"
for _ in $(seq 1 720); do /usr/bin/grep -q "queue2 done" "$G3/sweep.log" 2>/dev/null && break; sleep 20; done
echo "$(date +%H:%M:%S) start k3-ac-build" >> "$G3/sweep.log"
nice -n 10 bun "$U/target-run.mjs" k3-ac-build "$G/closure-roots.jsonl" "@teaching/architecture-course:build" "$G3/ac-build-sim.files" --rounds 6 --timeout 1500 > "$G3/sweep-k3-ac-build.out" 2> "$G3/sweep-k3-ac-build.err"
echo "$(date +%H:%M:%S) end k3-ac-build rc=$?" >> "$G3/sweep.log"
