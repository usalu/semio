#!/usr/bin/env bash
# 🪁️ Native fallback for `@teaching/staffage-react:typecheck` and `@teaching/staffage-site:typecheck`: the package's own `tsconfig.json` with the workspace's TypeScript compiler under a capacity grant, no emit, no build info. Usage: typecheck.sh <react|site> [tsc options...]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
STAFFAGE="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage"
case "$1" in
  react) PACKAGE="$STAFFAGE/🎯️targets/⚛️react/📦️packages/🟦️typescript" ;;
  site) PACKAGE="$STAFFAGE/🌐️site/📦️packages/🟦️typescript" ;;
  *) echo "typecheck.sh <react|site>" >&2; exit 2 ;;
esac
shift
cd "$ROOT"
repo capacity run --memory="${TYPECHECK_MEMORY_MB:-3000}" -- timeout "${TYPECHECK_TIMEOUT_S:-1800}" \
  node --max-old-space-size="${TYPECHECK_HEAP_MB:-2800}" "$ROOT/node_modules/typescript/lib/tsc.js" --noEmit --incremental false --pretty false -p "$PACKAGE/tsconfig.json" "$@"
