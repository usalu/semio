#!/usr/bin/env bash
# 🏗️ Builds the site (release or prototype) with its own Vite configuration under a capacity grant, into the package's dist/<mode>. Usage: build.sh [release|prototype]
set -euo pipefail
source "$(dirname "$0")/env.sh"
MODE="${1:-release}"
cd "$ROOT"
SEMIO_BUILD_MODE="$MODE" SEMIO_VITE_CACHE_DIRECTORY="$ROOT/.cache/b/p/m/vite-staffage-mac" \
  repo capacity run --memory="${BUILD_MEMORY_MB:-3500}" -- timeout "${BUILD_TIMEOUT_S:-3600}" /usr/bin/time -l \
  "$BUN" "$HERE/build-traced.mjs" "$MODE"
