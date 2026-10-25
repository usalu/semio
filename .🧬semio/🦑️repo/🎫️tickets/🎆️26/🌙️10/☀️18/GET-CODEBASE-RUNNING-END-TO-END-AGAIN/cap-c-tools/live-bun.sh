#!/bin/sh
# live proof: one Bun-bundled artifact package :test (counted bun-bundle slot)
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo --tool=semio exec --projects=@semio-tech/sequence-sequence -- repo ./📜️script.ts test 2>&1
echo "rc=$?"
