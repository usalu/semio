#!/bin/bash
# usage: gperf-retry.sh PREFIX  (runs show projects with cpu-prof until it completes without error)
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated"
S="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"
for i in 1 2 3 4 5 6 7 8; do
  log="$G/$1-$i.log"; mkdir -p "$G/$1-prof$i"
  SEMIO_BUILD_TRACE_AUTHORITY=true "$G/gperf-run.sh" "$log" bun --cpu-prof --cpu-prof-dir="$G/$1-prof$i" "./$S" repo show projects
  if ! /usr/bin/grep -qE "error:" "$log"; then echo "$(date +%T) attempt $i ok" >> "$G/$1-status.txt"; exit 0; fi
  echo "$(date +%T) attempt $i failed: $(/usr/bin/grep -m1 'error:' "$log" | cut -c1-150)" >> "$G/$1-status.txt"
  sleep 20
done
