#!/usr/bin/env bash
# 🔁️ Retries tc-law.sh while a peer's transient breakage stops the build before any test ran. Usage: tc-retry.sh <filter> <tag> <package> <target>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"
for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do
  bash "$M/tc-law.sh" "$1" "$2" "$3" "$4"
  grep -aq "^error: could not compile" "$M/🗑️generated/tq/law-$2.log" || break
  cp "$M/🗑️generated/tq/law-$2.log" "$M/🗑️generated/tq/law-$2-attempt$attempt.log"
  sleep 180
done
