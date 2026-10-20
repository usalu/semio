#!/bin/sh
# usage: run-tests.sh real | mirror <mirror-dir>
# Runs the deps5 suites (explicit file paths only, sequentially, with a minted caller envelope) against the real tree or a mirror built by `apply.py mirror`.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
REAL="$(cd "$HERE/../../../../../../../.." && pwd)"
MODE="${1:-real}"
if [ "$MODE" = mirror ]; then BASE="$(cd "$2" && pwd)"; else BASE="$REAL"; fi
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage"
ART="${SEMIO_TEST_ARTIFACT_DIR:-/private/tmp/deps5-art}"; mkdir -p "$ART"; export SEMIO_TEST_ARTIFACT_DIR="$(cd "$ART" && pwd -P)"
cd "$REAL" || exit 2
ENVELOPE="$(bun "$HERE/bench/mint.ts")"
cd "$BASE" || exit 2
run() { SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" perl -e 'alarm 560; exec @ARGV' -- bun test --timeout 300000 "./$D/🧪️tests/$1/🟦️.ts" "${@:2}" 2>&1 | grep -E '^ *[0-9]+ (pass|fail)|^\(fail\)|^error:' | sed 's/ \[[0-9.]*ms\]//' | head -${LINES_MAX:-10}; }
echo "== installed path scan: mutation table + progress frames"; run "📏️paths"
echo "== endpoint fence + full comparison + producer law"; run "🚧️fence"
echo "== provider walk memo vs mutations"; run "🔭️providers"
echo "== regression: producer/reader laws (baseline failures are stale tests, compare with the real tree before apply)"; LINES_MAX=3 run "📤️diagnostics"
