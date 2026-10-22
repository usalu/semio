#!/bin/sh
# usage: t.sh <log-name> <crate> [extra cargo test args after --]; one cargo test of the root workspace
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log="$C/🗑️generated/rust-reds-b-logs/$1.log"; crate=$2; shift 2
ws=${WS:-Cargo.toml}
export RUST_MIN_STACK=33554432
for attempt in $(seq 1 30); do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ws" --no-fail-fast --message-format=short -p "$crate" "$@" > "$log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q 'LeaseOrderErrorV1\|Host capacity wait cancelled\|Refusing to wait\|Insufficient physical compiler' "$log" && ! /usr/bin/grep -q 'test result\|could not compile' "$log"; then echo "retry $attempt" >> "$log.retries"; sleep 45; continue; fi
  break
done
echo "rc=$rc" > "$log.status"
