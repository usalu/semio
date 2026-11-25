#!/usr/bin/env bash
# 🏗️ Native fallback for `@teaching/staffage-site:build`: the site's own Vite configuration with the pinned bun, under a capacity grant, into the package's `dist/<mode>`, through `build-traced.mjs`, which reports each checked catalog asset. Usage: build.sh [release|prototype]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
SITE="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site"
MODE="${1:-release}"
BUNS=("$HOME"/.cache/semio/runtimes/bun/*/bun)
cd "$ROOT"
SEMIO_BUILD_MODE="$MODE" SEMIO_VITE_CACHE_DIRECTORY="$ROOT/.cache/b/p/m/vite-staffage-s1" \
  repo capacity run --memory="${BUILD_MEMORY_MB:-3500}" -- timeout "${BUILD_TIMEOUT_S:-3600}" \
  "${BUNS[0]}" "$HERE/build-traced.mjs" "$MODE"
