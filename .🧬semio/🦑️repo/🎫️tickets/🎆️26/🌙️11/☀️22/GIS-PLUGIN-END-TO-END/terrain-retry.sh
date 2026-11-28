#!/bin/bash
# usage: terrain-retry.sh NAME cargo-args... ; retries on infrastructure failures, waits while the exclusive planner window file exists
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
N="$1"; shift
gate() {
  while [ -e "$T/🗑️generated/exclusive-plan.active" ]; do sleep 30; done
}
for attempt in 1 2 3 4 5 6 7 8 9 10; do
  gate
  "$T/terrain-run.sh" "$N-$attempt" "$@"
  grep -q -E "error\[E|test result:|exceeds its source boundary|Source boundary exceeded" "$T/🗑️generated/terrain-$N-$attempt.log" && break
  sleep 60
done
