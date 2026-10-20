#!/bin/sh
# usage: sh test.sh [MIRROR_DIR] [bun test args...]   runs the new suite in the scratch mirror (one process, explicit file path)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/si/m}
[ $# -gt 0 ] && shift
python3 "$HERE/apply.py" mirror "$M" --apply | tail -1
cd "$M"
mkdir -p /private/tmp/si/art
ENVELOPE=$(cd "$HERE/../../../../../../../.." && bun "$HERE/mint.ts")
SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/si/art exec bun test --timeout 300000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts" "$@"
