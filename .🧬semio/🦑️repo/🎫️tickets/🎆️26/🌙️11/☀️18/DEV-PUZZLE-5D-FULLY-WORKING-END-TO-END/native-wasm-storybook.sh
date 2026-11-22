#!/usr/bin/env bash
set -euo pipefail

ticket=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$ticket/../../../../../../.." && pwd)
output="$ticket/🗑️generated"
label="${1:?A verified or interim WASM candidate directory name is required}"
[[ "$label" =~ ^wasm-(candidate|interim)-[a-z0-9-]+$ ]]
candidate="$output/$label"
run=$(mktemp -d "$output/storybook-$label-XXXXXX")
printf '%s\n' "$run"

unshare --user --map-root-user --mount --propagation private bash -s -- "$root" "$ticket" "$candidate" "$run" <<'SEMIO_5D_STORYBOOK'
set -euo pipefail
root="$1"
ticket="$2"
candidate="$3"
run="$4"
cd "$root"
stage="$root/.cache/b/p/m/browser/prototype/wasm-packages/d30e56a055ea2d5bde08382c"
mount --bind "$candidate" "$stage"
vite_cache="$root/.cache/b/p/m/a/native-vite-5d"
mkdir "$run/cache"
mount --bind "$run/cache" "$vite_cache"
runtime=(/home/deck/.cache/semio/runtimes/bun/*/bun)
export SEMIO_VITE_CACHE_DIRECTORY="$vite_cache" SEMIO_BUILD_MODE=prototype STORYBOOK_SCOPE=puzzle/5d
node node_modules/storybook/dist/bin/dispatcher.js dev -p 6010 --ci --no-open --disable-telemetry >"$run/server.log" 2>&1 &
server=$!
trap 'kill "$server" 2>/dev/null || true' EXIT
for _ in {1..180}; do
    if curl --max-time 2 -fsS http://localhost:6010/index.json >"$run/index.json" 2>"$run/probe.log"; then break; fi
    kill -0 "$server"
    sleep 1
done
test -s "$run/index.json"
"${runtime[0]}" node_modules/playwright/cli.js test -c "$ticket/native-pw.config.ts" -g puzzle5d --output "$run/test-results" >"$run/playwright.log" 2>&1
SEMIO_5D_STORYBOOK
