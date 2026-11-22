#!/usr/bin/env bash
# usage: l3-serve.sh <scope 2d|3d|5d>  -- foreground storybook on 6010; log in 🗑️generated/l3-run/<scope>
set -euo pipefail
ticket=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$ticket/../../../../../../.." && pwd)
run="$ticket/🗑️generated/l3-run/serve-$1"; mkdir -p "$run"; cd "$root"
export SEMIO_VITE_CACHE_DIRECTORY="$root/.cache/b/p/m/a/l3-vite" SEMIO_BUILD_MODE=prototype STORYBOOK_SCOPE="puzzle/$1"
exec node node_modules/storybook/dist/bin/dispatcher.js dev -p 6010 --ci --no-open --disable-telemetry >"$run/server.log" 2>&1
