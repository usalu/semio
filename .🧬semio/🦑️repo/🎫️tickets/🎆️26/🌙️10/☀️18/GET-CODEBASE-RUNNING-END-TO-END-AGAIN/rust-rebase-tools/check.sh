#!/bin/sh
# usage: check.sh <manifest> <log> <cargo-check-args...> ; one cargo check run, retried on infrastructure refusals
cd /Users/ueli/Documents/semio || exit 2
ws=$1; log=$2; shift 2
for attempt in $(seq 1 30); do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ws" --keep-going --message-format=short "$@" > "$log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q 'LeaseOrderErrorV1\|Host capacity wait cancelled\|Refusing to wait\|Insufficient physical compiler' "$log" && ! /usr/bin/grep -q 'could not compile' "$log"; then echo "retry $attempt" >> "$log.retries"; sleep 45; continue; fi
  break
done
echo "rc=$rc" > "$log.status"
