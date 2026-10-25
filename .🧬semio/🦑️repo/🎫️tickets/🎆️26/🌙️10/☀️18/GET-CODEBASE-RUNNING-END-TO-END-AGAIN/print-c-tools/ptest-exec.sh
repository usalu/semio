#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export NODE_OPTIONS= SEMIO_BUILD_DYNAMIC_OUTPUT=false SEMIO_BUILD_TERMINAL=false SEMIO_SCRIPT_STALL_MS=14400000 SEMIO_BUILD_GRAPH_POLICY=current
exec nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/print -- repo ./📜️script.ts "$@"
