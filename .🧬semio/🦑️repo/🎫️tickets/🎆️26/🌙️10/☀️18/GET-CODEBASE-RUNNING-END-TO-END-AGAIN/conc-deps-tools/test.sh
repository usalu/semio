#!/bin/sh
# usage: sh test.sh [MIRROR_DIR] [bun test args...]   builds the scratch mirror with the set applied and runs the new suite (one process, explicit file path)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/conc-deps/m}
[ $# -gt 0 ] && shift
python3 "$HERE/apply.py" mirror "$M" --apply --group ${GROUP:-3} | tail -1
cd "$M"
mkdir -p /private/tmp/conc-deps/art
ENVELOPE=$(cd "$HERE/../../../../../../../.." && bun "$HERE/mint.ts")
SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/conc-deps/art exec bun test --timeout 300000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🧪️tests/🟦️.ts" "$@"
