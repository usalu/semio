#!/usr/bin/env bash
# 🎪️ Builds every demonstrator plugin component, boots :6029 and runs the 7-test acceptance suite.
# Budget is raised well past BUILD_BUDGET_MS (20 min), which silently kills a cold wasm plugin build.
# RUSTC_WRAPPER is cleared because sccache serializes concurrent builds in this repo.
set -uo pipefail
cd /Users/ueli/Documents/semio || exit 1
export RUSTC_WRAPPER=""
export SEMIO_BUILD_BUDGET_MS="${SEMIO_BUILD_BUDGET_MS:-7200000}"
export SEMIO_CMD_BUDGET_MS="${SEMIO_CMD_BUDGET_MS:-7200000}"
export MIT_BESTAND_DEMONSTRATOR_PORT="${MIT_BESTAND_DEMONSTRATOR_PORT:-6029}"
export PLAYWRIGHT_BROWSERS_PATH="$PWD/node_modules/.cache/ms-playwright"
cd "♻️mit-bestand/🧺️demonstrator" || exit 1
exec bun ./📜️script.ts test e2e "$@"
