#!/bin/zsh
# usage: tw.sh OVERLAY TESTFILE(repo-relative) [bun test args]; mints the root Script envelope and runs ONE test file in the copy: nice, 290 s alarm
OV="$1"; f="$2"; shift 2
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/storage-seam-tools"
cd "$OV" || exit 9
cp "$T/envwire.ts" "$OV/.envwire.ts"
wire=$(SEMIO_REPO_ROOT="$OV" bun "$OV/.envwire.ts")
mkdir -p /private/tmp/storage-seam/art; RUN=$(mktemp -d /private/tmp/storage-seam/art/r.XXXXXX)
SEMIO_TEST_ARTIFACT_DIR="$RUN" NO_COLOR=1 SEMIO_REPO_ROOT="$OV" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" SEMIO_BUILD_MODE=prototype perl -e 'alarm 290; exec @ARGV' -- nice -n 10 bun test --timeout 120000 "$@" "./$f"
rc=$?; rm -rf "$RUN"; exit $rc
