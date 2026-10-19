#!/bin/zsh
# usage: t.sh <test file path relative to repo root> [bun test args]; mints the root Script envelope like tst3c, runs one file under alarm 300
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
ART="$PWD/$C/🗑️generated/xplat-win2-tests/run"; mkdir -p "$ART"; RUN=$(mktemp -d "$ART/r.XXXXXX")
wire=$(SEMIO_REPO_ROOT=$PWD bun "$C/tst3c-tools/envwire.ts")
f="$1"; shift
SEMIO_TEST_ARTIFACT_DIR="$RUN" NO_COLOR=1 SEMIO_REPO_ROOT="$PWD" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" SEMIO_TICKET_DIR="$PWD/$C" SEMIO_BUILD_MODE=prototype perl -e 'alarm 300; exec @ARGV' -- bun test --timeout 120000 "$@" "./$f"
rc=$?; rm -rf "$RUN"; exit $rc
