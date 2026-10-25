#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export PRAESENTATION_PROJEKTETAGE_PORT=6050 NODE_OPTIONS= SEMIO_BUILD_DYNAMIC_OUTPUT=false SEMIO_BUILD_TERMINAL=false SEMIO_BUILD_GRAPH_POLICY=current
exec nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/mit-bestand-praesentation-projektetage -- repo ./📜️script.ts dev
