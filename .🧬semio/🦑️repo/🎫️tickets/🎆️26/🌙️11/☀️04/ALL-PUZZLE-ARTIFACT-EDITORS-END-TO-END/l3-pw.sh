#!/usr/bin/env bash
# usage: l3-pw.sh <test-dir> <grep> <outname>  (storybook already on :6010)
set -euo pipefail
ticket=$(cd "$(dirname "$0")" && pwd); root=$(cd "$ticket/../../../../../../.." && pwd); cd "$root"
runtime=(/home/deck/.cache/semio/runtimes/bun/*/bun)
mkdir -p "$ticket/🗑️generated/l3-run"
L3_TEST_DIR="$1" "${runtime[0]}" node_modules/playwright/cli.js test -c "$ticket/l3-pw.config.ts" -g "$2" --output "$ticket/🗑️generated/l3-run/$3-results" >"$ticket/🗑️generated/l3-run/$3.log" 2>&1 || echo "PW EXIT $?" >>"$ticket/🗑️generated/l3-run/$3.log"
