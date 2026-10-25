#!/bin/sh
# usage: run.sh LOG cargo-args...   (retries on lease/loader refusals; last attempt kept)
LOG=$1; shift
n=0
while [ $n -lt 20 ]; do
  n=$((n+1))
  : > "$LOG"
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$LOG" 2>&1
  rc=$?
  if /usr/bin/grep -qE 'SEMIO_LEASE_ORDER|LeaseOrderErrorV1|Authenticated captured build command entry required|Insufficient physical compiler|"await" can only be used|Expected ";" but found|Uncaptured implementation|Original Cargo workspace topology changed|Original capture invocation changed|ENOENT: no such file or directory|Refusing to wait|Stale loaded repository Store|Host capacity wait cancelled|Relative filesystem [a-z]* failed' "$LOG"; then
    echo "attempt $n refused (rc=$rc), retry" >> "$LOG.attempts"
    sleep 90
    continue
  fi
  echo "attempt $n done rc=$rc" >> "$LOG.attempts"
  exit $rc
done
exit 99
