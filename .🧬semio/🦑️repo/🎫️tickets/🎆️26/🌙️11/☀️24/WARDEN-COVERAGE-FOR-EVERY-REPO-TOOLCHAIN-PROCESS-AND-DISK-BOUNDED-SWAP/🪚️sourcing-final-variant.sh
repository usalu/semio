#!/bin/sh
# 🪚️ Rebuilds only the sourcing hub final through the admitted entry with extra rustc flags for the final (variant measurement of its peak resident memory; the peak lands in the `miss` event of the lane log).
# Usage: sourcing-final-variant.sh <label> <rustc flags...>
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
label=$1; shift
mkdir -p "$here/🗑️generated"
cd "$root"
status=0
repo cargo --store=shared --configuration=prototype -- rustc --manifest-path 🌎️hub/Cargo.toml -p semio-hub-sourcing-component --lib --target wasm32-wasip2 -- "$@" > "$here/🗑️generated/variant-$label.txt" 2>&1 || status=$?
echo "exit $status -> $here/🗑️generated/variant-$label.txt"
