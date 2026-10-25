#!/bin/sh
# usage: live-cargo.sh <agent-id> <package> : one repo cargo check in its own agent lane (counted compile units, default parallelism)
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_AGENT_ID="$1" SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check --manifest-path Cargo.toml -p "$2" 2>&1
echo "rc=$?"
