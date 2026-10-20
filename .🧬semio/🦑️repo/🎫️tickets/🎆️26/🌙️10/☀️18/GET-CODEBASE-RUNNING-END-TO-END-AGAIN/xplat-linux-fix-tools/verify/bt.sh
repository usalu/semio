#!/bin/zsh
# usage: bt.sh OVERLAY TESTFILE(repo-relative) [bun test args]; one test file, nice, alarm
OV="$1"; f="$2"; shift 2
cd "$OV" || exit 9
mkdir -p /private/tmp/xplat-linux-fix/art; RUN=$(mktemp -d /private/tmp/xplat-linux-fix/art/r.XXXXXX)
SEMIO_TEST_ARTIFACT_DIR="$RUN" NO_COLOR=1 perl -e 'alarm 290; exec @ARGV' -- nice -n 10 bun test --timeout 120000 "$@" "./$f"
rc=$?; rm -rf "$RUN"; exit $rc
