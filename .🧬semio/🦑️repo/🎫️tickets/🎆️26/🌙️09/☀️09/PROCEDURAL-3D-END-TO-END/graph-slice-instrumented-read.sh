#!/bin/sh
# Instrumented real slice read; retries while a peer's in-flight edit breaks the repo CLI itself.
T="$(dirname "$0")/🗑️generated"; cd "$(git rev-parse --show-toplevel)" || exit 1
i=0
while [ $i -lt 60 ]; do
  i=$((i+1)); date +%s > "$T/after-start.txt"
  SEMIO_LANGUAGE=en SEMIO_GRAPH_PHASES=1 SEMIO_BUILD_PROFILE_INPUTS=1 repo show project @semio-tech/framework-os-dev --graph-policy=fresh > "$T/after-run.txt" 2>&1
  date +%s > "$T/after-end.txt"
  if grep -q "Export named\|Restarting with the current\|Internal error" "$T/after-run.txt"; then sleep 30; else break; fi
done
