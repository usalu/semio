#!/bin/sh
# usage: sh one.sh MIRROR SUITE-RELATIVE-TO-LIBRARY NAME-FILTER [lines]   one test (or filter) of one suite, error lines only
HERE=$(cd "$(dirname "$0")" && pwd)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
ENVELOPE=$(cd "$HERE/../../../../../../../.." && bun "$HERE/mint.ts")
mkdir -p /private/tmp/pm-seam/art
cd "$1"
SEMIO_SCRIPT_PROCESS_INVOCATION="$ENVELOPE" SEMIO_TEST_ARTIFACT_DIR=/private/tmp/pm-seam/art nice -n 10 bun test --timeout 240000 -t "$3" "./$L/$2" 2>&1 | /usr/bin/grep -E "^error|Expected|Received|^\(fail\)|^ *at |Error" | head -${4:-8} | cut -c1-420
