#!/bin/sh
# usage: run.sh <manifest> <log> <cargo-test-args...> ; one cargo test run, retried on infrastructure refusals
cd /Users/ueli/Documents/semio || exit 2
ws=$1; log=$2; shift 2
export RUST_MIN_STACK=33554432
for attempt in $(seq 1 30); do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ws" --no-fail-fast --message-format=short "$@" > "$log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q 'LeaseOrderErrorV1\|Host capacity wait cancelled\|Refusing to wait\|Insufficient physical compiler' "$log" && ! /usr/bin/grep -q 'test result\|could not compile' "$log"; then echo "retry $attempt" >> "$log.retries"; sleep 45; continue; fi
  break
done
echo "rc=$rc" > "$log.status"
