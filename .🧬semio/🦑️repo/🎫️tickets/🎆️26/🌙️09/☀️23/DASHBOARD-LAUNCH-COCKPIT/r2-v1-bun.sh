#!/usr/bin/env bash
# usage: r2-v1-bun.sh <log name> <test file (repo relative)> [bun test args...]   runs bun test outside the harness job
here="$(cd "$(dirname "$0")" && pwd)"
name="$1"; file="$2"; shift 2
cache="$(cygpath -w /c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo)"
OOB_WAIT=${OOB_WAIT:-100} "$here/r2-v1-oob.sh" "$name" /c/git/semio "[Console]::OutputEncoding=[Text.Encoding]::UTF8; \$env:SEMIO_TEST_CLI=(\$env:SEMIO_TEST_CLI ?? '$cache\target-fleet-v1\debug\semio.exe'); bun test './$file' --timeout ${BUN_TIMEOUT:-240000} $*"
