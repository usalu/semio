#!/bin/bash
T="$(cd "$(dirname "$0")/../.." && pwd)"
cd /Users/ueli/Documents/semio
O="$T/🗑️generated/1030/smoke-activate"
echo "start $(date '+%F %T')" > "$O.status"
SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/framework-os-dev:activate-puzzle3d-react-prototype > "$O.txt" 2>&1
echo "exit $? $(date '+%F %T')" >> "$O.status"
