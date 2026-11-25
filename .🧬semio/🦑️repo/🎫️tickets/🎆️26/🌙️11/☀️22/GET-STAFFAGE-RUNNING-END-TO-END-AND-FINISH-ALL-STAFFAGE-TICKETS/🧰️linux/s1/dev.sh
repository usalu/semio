#!/usr/bin/env bash
# 🌐️ Native fallback for `@teaching/staffage-site:dev`: the site's own Vite configuration as a development server with the pinned bun, without file watching. Usage: dev.sh [port]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
SITE="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site"
BUNS=("$HOME"/.cache/semio/runtimes/bun/*/bun)
cd "$SITE"
SEMIO_BUILD_MODE=prototype TEACHING_ARCHITECTURE_STAFFAGE_WATCH=off SEMIO_VITE_CACHE_DIRECTORY="$ROOT/.cache/b/p/m/vite-staffage-s1" \
  exec "${BUNS[0]}" "$ROOT/node_modules/vite/bin/vite.js" dev --config "$SITE/🏗️builder/🌐️vite/🟦️.ts" --mode prototype --host 127.0.0.1 --port "${1:-6272}" --strictPort
