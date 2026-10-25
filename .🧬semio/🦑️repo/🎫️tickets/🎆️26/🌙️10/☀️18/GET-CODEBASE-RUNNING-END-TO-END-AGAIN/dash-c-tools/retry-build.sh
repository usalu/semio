#!/bin/sh
# usage: retry-build.sh <tries>  Rebuilds the dashboard binary with cargo, retrying when the lease-order guard refuses the compiler-writer wait.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/dash-c"; export SEMIO_SCRIPT_STALL_MS=14400000
i=0
while [ $i -lt ${1:-10} ]; do
  i=$((i+1))
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- build --manifest-path Cargo.toml --message-format=short -p semio-framework-repo-dashboard --bin semio > "$G/cargo-build-retry-$i.log" 2>&1
  /usr/bin/grep -q "Finished" "$G/cargo-build-retry-$i.log" && { echo "built on try $i" > "$G/cargo-build-retry.done"; exit 0; }
  /usr/bin/grep -q "LeaseOrderErrorV1" "$G/cargo-build-retry-$i.log" || { echo "failed on try $i (not lease order)" > "$G/cargo-build-retry.done"; exit 1; }
  sleep 45
done
echo "gave up" > "$G/cargo-build-retry.done"; exit 1
