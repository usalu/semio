#!/bin/sh
# usage: chk.sh <manifest> <log> <cargo args...> : one `cargo check` (any subcommand args) via the repo ingress, retried on infra refusals; writes <log>.status last
cd /Users/ueli/Documents/semio || exit 2
ws=$1; log=$2; sub=$3; shift 3
export RUST_MIN_STACK=33554432
for attempt in $(seq 1 30); do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$sub" --manifest-path "$ws" "$@" > "$log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q 'LeaseOrderErrorV1\|Host capacity wait cancelled\|Refusing to wait\|Insufficient physical compiler\|Uncaptured implementation import refused\|ENOENT: no such file\|Cannot find module' "$log" && ! /usr/bin/grep -q 'test result\|could not compile' "$log"; then echo "retry $attempt" >> "$log.retries"; sleep 45; continue; fi
  break
done
echo "rc=$rc" > "$log.status"
