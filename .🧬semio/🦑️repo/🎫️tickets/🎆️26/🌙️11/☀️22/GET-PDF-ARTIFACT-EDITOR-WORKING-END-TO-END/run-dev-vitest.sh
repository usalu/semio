#!/bin/sh
# usage: run-dev-vitest.sh <suite path relative to the dev package root> <absolute log> [extra vitest args]
S=/Users/ueli/Documents/semio
R="$S/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
T="$S/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END"
for B in "$HOME"/Library/Caches/semio/runtimes/bun/*/ "$HOME"/.cache/semio/runtimes/bun/*/ "$HOME"/.bun/bin/; do [ -d "$B" ] && PATH="$B:$PATH"; done
export PATH
cd "$S" || exit 2
SEMIO_SCRIPT_PROVIDERS="$(bun "$T/issue-script-providers.ts")" || exit 2
export SEMIO_SCRIPT_PROVIDERS
cd "$R" || exit 2
SUITE="$1"; LOG="$2"; shift 2
export NODE_OPTIONS=--max-old-space-size=2048 SEMIO_TEST_LEVEL=long
exec "$(command -v node)" "$S/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native --pool=forks --maxWorkers=1 --no-file-parallelism --reporter=verbose "$SUITE" "$@" > "$LOG" 2>&1
