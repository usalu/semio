#!/usr/bin/env bash
# 🤝️ Slice C1b — one collaboration e2e run against a real hub and two real `s` react shells.
# Run from anywhere. Every value here is a deliberate override of a default that does not survive this
# repo's concurrent-fleet load:
#   NX_DAEMON=false             the shared nx daemon re-invalidates the 130 MB project graph on every peer
#                               file write, so a daemon-backed `nx run` never settles under load.
#   SEMIO_BUILD_BUDGET_MS       deliberately NOT set: worker H1 made a zero budget mean the 24 h
#                               fresh-component ceiling, and any capped value below ~3 h aborts two cold
#                               wasm-release component builds under this repo's fleet load.
#   S_COLLAB_TRUSTED_CATALOG    seeds the fresh per-run OS_HUB_DATA from an already-materialized trusted
#                               stdio+GIS catalog (publish it once with 📜️c1b-warm-catalog.sh) instead of
#                               rebuilding two wasm-release components per run.
set -euo pipefail
ROOT=/Users/ueli/Documents/semio
cd "$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false
# SEMIO_BUILD_BUDGET_MS deliberately UNSET: worker H1 made a zero budget mean the 24 h ceiling, and a
# capped 90 min is not enough for two cold wasm-release component builds under fleet load.
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export S_COLLAB_TRUSTED_CATALOG="$ROOT/.🧬semio/🌐hub/c1b-warm"
export COLLAB_E2E_HUB_BOOT_BUDGET_MS=900000
export COLLAB_E2E_DEV_BOOT_BUDGET_MS=1800000
export COLLAB_E2E_PREBUILD_BUDGET_MS=1800000
exec bun ./📜️script.ts verify collab
