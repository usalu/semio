#!/bin/sh
# usage: lane.sh <name> <verify args...>  (run from repo root); retries up to 4 times when the dependency gate fails or the host runs out of file descriptors
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
name="$1"; shift
out="$C/🗑️generated/purity-d/$name.log"
start=$(date +%s)
attempt=0
while :; do
  attempt=$((attempt+1))
  SEMIO_LANGUAGE=en nice -n 10 bun ./📜️script.ts verify "$@" >"$out" 2>&1
  rc=$?
  if [ "$rc" -ne 0 ] && [ "$attempt" -lt 4 ] && { /usr/bin/grep -q "repo dependencies ensure. failed\|Too many open files\|ENFILE" "$out" || [ "$rc" -eq 134 ]; }; then
    cp "$out" "$out.attempt$attempt"
    sleep 120
    continue
  fi
  break
done
end=$(date +%s)
printf '%s\t%s\t%ss\tattempts=%s\tverify %s\n' "$name" "$rc" "$((end-start))" "$attempt" "$*" >> "$C/🗑️generated/purity-d/verdicts.tsv"
echo "$name rc=$rc $((end-start))s"
