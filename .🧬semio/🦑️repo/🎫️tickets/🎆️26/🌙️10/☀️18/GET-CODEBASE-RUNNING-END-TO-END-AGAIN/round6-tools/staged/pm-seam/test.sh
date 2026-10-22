#!/bin/sh
# usage: sh test.sh [MIRROR_DIR] [bun test args...]   builds the scratch mirror (round-5 closure batch + this set applied) and runs the new suite (one process, explicit file path, nice 10)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/pm-seam/m}
[ $# -gt 0 ] && shift
[ -d "$M/node_modules" ] || python3 "$HERE/apply.py" mirror "$M" --lock-sync --apply | tail -2
cd "$M"
mkdir -p /private/tmp/pm-seam/art
ENVELOPE=$(cd "$HERE/../../../../../../../.." && nice -n 10 bun "$HERE/mint.ts")
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies"
SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/pm-seam/art exec nice -n 10 bun test --timeout 300000 "./$D/🔌️managers/🧪️tests/🟦️.ts" "$@"
