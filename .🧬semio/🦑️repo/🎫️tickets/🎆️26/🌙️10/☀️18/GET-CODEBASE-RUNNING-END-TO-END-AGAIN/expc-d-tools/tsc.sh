#!/bin/sh
# usage: tsc.sh <tag> <process|course|print>   (repo root)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
AMB="🧰️framework/📦️packages/🟦️typescript/🌿️ambient"
out="$C/🗑️generated/expc-d/tsc-$1-$2.log"
case "$2" in
  process) set -- --scope 🧰️framework/🔨️modules/🏃️process ;;
  course) set -- --scope 🎓️teaching --scope 🧰️framework/🛍️products/📚️course ;;
  print) set -- --scope 🧰️framework/🛍️products/📓️print --scope 🧰️framework/🛍️products/🎤️presentation --scope ♻️mit-bestand/🎤️präsentation ;;
esac
SEMIO_LANGUAGE=en nice -n 10 bun ./📜️script.ts verify typescript check "$@" --scope "$AMB" --timeout-minutes 25 > "$out" 2>&1
echo "$out rc=$?"
