#!/usr/bin/env bash
# usage: l3-storybook.sh <scope 2d|3d|5d> <test-dir> <grep>
set -euo pipefail
ticket=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$ticket/../../../../../../.." && pwd)
scope="$1"; dir="$2"; grep="$3"
run="$ticket/🗑️generated/l3-run/$scope"; mkdir -p "$run"
cd "$root"
runtime=(/home/deck/.cache/semio/runtimes/bun/*/bun)
export SEMIO_VITE_CACHE_DIRECTORY="$root/.cache/b/p/m/a/l3-vite" SEMIO_BUILD_MODE=prototype STORYBOOK_SCOPE="puzzle/$scope" L3_TEST_DIR="$dir"
node node_modules/storybook/dist/bin/dispatcher.js dev -p 6010 --ci --no-open --disable-telemetry >"$run/server.log" 2>&1 &
server=$!
trap 'kill "$server" 2>/dev/null || true' EXIT
for _ in {1..300}; do
  if curl --max-time 2 -fsS http://localhost:6010/index.json >"$run/index.json" 2>/dev/null; then break; fi
  kill -0 "$server"; sleep 1
done
test -s "$run/index.json"
"${runtime[0]}" node_modules/playwright/cli.js test -c "$ticket/l3-pw.config.ts" -g "$grep" --output "$run/test-results" >"$run/playwright.log" 2>&1 || echo "PW EXIT $?" >>"$run/playwright.log"
