#!/bin/sh
# usage: sh test.sh [MIRROR_DIR] [bun test args...]   mirrors the live tree with the round-2 set applied and runs the adopt suite (one process)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/si/m2}
[ $# -gt 0 ] && shift
python3 "$HERE/apply.py" mirror "$M" --apply | tail -1
cd "$M"
mkdir -p /private/tmp/si/art
ENVELOPE=$(cd "$HERE/../../../../../../../.." && bun "$HERE/../mint.ts")
SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/si/art exec bun test --timeout 300000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🪞️adopt/🧪️tests/🟦️.ts" "$@"
