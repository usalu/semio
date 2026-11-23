#!/bin/sh
# usage: story-playwright.sh <playwright args...>; runs the ui Storybook browser suites natively against a served Storybook (default 127.0.0.1:6090)
R=/Users/ueli/Documents/semio
export PLAYWRIGHT_BROWSERS_PATH="$R/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
export PLAYWRIGHT_BASE_URL="${PLAYWRIGHT_BASE_URL:-http://127.0.0.1:6090/}"
export PLAYWRIGHT_OUTPUT_DIR=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS/🗑️generated/pw-out"
cd "$R" && exec "$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)" --bun node_modules/@playwright/test/cli.js test --config .storybook/🧪️tests/🧪️browser-runner/🟦️.ts "$@"
