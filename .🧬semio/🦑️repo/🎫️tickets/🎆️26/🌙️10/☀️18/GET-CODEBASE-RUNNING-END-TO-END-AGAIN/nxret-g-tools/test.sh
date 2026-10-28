#!/bin/sh
# runs bun test with the bounded-namespace environment the Nx SDK laws demand; cwd stays where the caller is
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
R="/Users/ueli/Documents/semio"
mkdir -p "$C/🗑️generated/nxret-g/ta"
D="$R/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts"
SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/nxret-g/ta" SEMIO_NX_FIXED_STORAGE="$(dirname "$(dirname "$D")")" NX_CACHE_DIRECTORY="$D" exec nice -n 10 bun test "$@"
