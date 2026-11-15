#!/usr/bin/env bash
# usage: normcomp-check.sh <check|build> <logname>
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
R="$(git -C "$T" rev-parse --show-toplevel)"
cd "$R/🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust/../../../../../.." 2>/dev/null
cd "$R/🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust"
repo cargo --store=shared --configuration=prototype -- "$1" --keep-going $EXTRA --target wasm32-wasip2 -p semio-hub-norm-component > "$T/🗑️generated/normcomp-$2.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/normcomp-$2.log"
