#!/usr/bin/env bash
# usage: b2df-retry.sh <name> <cargo args...>  (EXTRA passes through); retries while the build fails to compile (others' in-flight edits)
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
N="$1"; shift
for attempt in 1 2 3 4 5 6; do
  bash "$T/b2df-run.sh" "$N" "$@"
  if ! grep -qE "could not compile|omitted observed native effects" "$T/🗑️generated/b2df-$N.log"; then exit 0; fi
  cp "$T/🗑️generated/b2df-$N.log" "$T/🗑️generated/b2df-$N-attempt$attempt.log"
  sleep 600
done
