#!/usr/bin/env bash
# Starts Storybook (scope puzzle) on 6010 with the managed wasm stage; logs to 🗑️generated/p4-storybook.log
set -euo pipefail
ticket=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$ticket/../../../../../../.." && pwd)
cd "$root"
mkdir -p "$root/.cache/b/p/m/a/p4-vite-wasm"
export SEMIO_VITE_CACHE_DIRECTORY="$root/.cache/b/p/m/a/p4-vite-wasm" SEMIO_BUILD_MODE=prototype STORYBOOK_SCOPE=puzzle
exec node node_modules/storybook/dist/bin/dispatcher.js dev -p 6010 --ci --no-open --disable-telemetry
