#!/bin/sh
# usage: run2.sh LOG cargo-args...  json diagnostics filtered to errors; retries on infra refusals
LOG=$1; shift
T=$(dirname "$0")
n=0
while [ $n -lt 20 ]; do
  n=$((n+1))
  : > "$LOG"
  ( nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" --message-format=json 2>&1; echo "RC=$?" >> "$LOG.rc" ) | python3 "$T/jsonkeep.py" > "$LOG"
  rc=$(tail -n1 "$LOG.rc" | sed 's/RC=//')
  if /usr/bin/grep -qE 'SEMIO_LEASE_ORDER|Authenticated captured build command entry required|Insufficient physical compiler|"await" can only be used|Expected ";" but found|Uncaptured implementation|Original Cargo workspace topology changed|Original capture invocation changed|ENOENT: no such file or directory' "$LOG"; then
    echo "attempt $n refused (rc=$rc), retry" >> "$LOG.attempts"; sleep 90; continue
  fi
  echo "attempt $n done rc=$rc" >> "$LOG.attempts"
  exit 0
done
exit 99
