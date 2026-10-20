#!/bin/zsh
# usage: tw.sh OVERLAY TESTFILE [bun args]; mints the root Script envelope (like win2/tst3c) and runs one test file in the overlay
OV="$1"; f="$2"; shift 2
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
wire=$(cd /Users/ueli/Documents/semio && SEMIO_REPO_ROOT=/Users/ueli/Documents/semio bun "$C/tst3c-tools/envwire.ts")
cd "$OV" || exit 9
mkdir -p /private/tmp/xplat-linux-fix/art; RUN=$(mktemp -d /private/tmp/xplat-linux-fix/art/r.XXXXXX)
SEMIO_TEST_ARTIFACT_DIR="$RUN" NO_COLOR=1 SEMIO_REPO_ROOT="$OV" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" SEMIO_TICKET_DIR="$C" SEMIO_BUILD_MODE=prototype perl -e 'alarm 290; exec @ARGV' -- nice -n 10 bun test --timeout 120000 "$@" "./$f"
rc=$?; rm -rf "$RUN"; exit $rc
