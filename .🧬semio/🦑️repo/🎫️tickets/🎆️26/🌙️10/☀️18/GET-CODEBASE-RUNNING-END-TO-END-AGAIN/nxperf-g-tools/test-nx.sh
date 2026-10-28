#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p "$C/🗑️generated/nxperf-g/ta"
D="$PWD/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts"
SEMIO_TEST_ARTIFACT_DIR="$PWD/$C/🗑️generated/nxperf-g/ta" SEMIO_NX_FIXED_STORAGE="$(dirname "$(dirname "$D")")" NX_CACHE_DIRECTORY="$D" exec bun test "$@"
