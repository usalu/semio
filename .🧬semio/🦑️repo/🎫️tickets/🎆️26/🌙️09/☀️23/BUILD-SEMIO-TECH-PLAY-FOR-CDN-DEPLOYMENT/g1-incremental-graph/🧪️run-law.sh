#!/usr/bin/env bash
# 🧪️ Runs one bun test file from the repository root under a locally issued Script invocation envelope while `bun repo run` cannot
# provision its Nx SDK snapshot (tools volume full). argv: <test file relative to repo root> <nx tooling identity> <artifact dir> [bun test args...]
set -eu
root="/Users/ueli/Documents/semio"; file="$1"; identity="$2"; artifacts="$3"; shift 3
mkdir -p "$artifacts"
envelope="$(node -e 'const n=Date.now();console.log(JSON.stringify({version:1,policy:{version:1,owner:"g1-incremental-law",maximumElapsedMilliseconds:900000},deadlineEpochMilliseconds:n+900000,capabilities:{}}))')"
cd "$root"
SEMIO_SCRIPT_PROCESS_INVOCATION="$envelope" SEMIO_NX_TOOLING_IDENTITY="$identity" SEMIO_TEST_ARTIFACT_DIR="$artifacts" bun test --timeout 600000 "./$file" "$@"
