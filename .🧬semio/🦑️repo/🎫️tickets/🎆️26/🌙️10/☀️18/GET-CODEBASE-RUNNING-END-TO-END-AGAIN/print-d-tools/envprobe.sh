#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
export NODE_OPTIONS= SEMIO_BUILD_DYNAMIC_OUTPUT=false SEMIO_BUILD_TERMINAL=false SEMIO_BUILD_GRAPH_POLICY=current
exec nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/print -- repo "/Users/ueli/Documents/semio/$C/print-d-tools/envprobe.ts"
