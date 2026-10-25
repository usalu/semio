#!/bin/sh
# usage: cargo.sh <cargo args...>
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000 SEMIO_BUILD_GRAPH_POLICY=current
exec nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@"
