#!/bin/bash
HERE=$(cd "$(dirname "$0")" && pwd)
for t in "${@:-editor node-graph puzzle flow canvas}"; do for x in $t; do "$HERE/native-wasm.sh" $x; done; done
