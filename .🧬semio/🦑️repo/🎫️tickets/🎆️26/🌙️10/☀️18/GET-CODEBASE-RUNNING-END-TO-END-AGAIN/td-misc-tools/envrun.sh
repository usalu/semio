#!/bin/sh
# usage: envrun.sh <mirror> <project:target> <repo-relative test file> [bun test args]  (NOART=1: no artifacts capability, as a suite without one grants; ONLYART=1: capabilities are only {artifacts}, the declared birth of td-leases) -> full output of the file run inside the official envelope of that target (ver-f proc-run.ts rewritten to the mirror and the target)
C="$(cd "$(dirname "$0")/.." && pwd)"; m="$1"; t="$2"; f="$3"; shift 3
R="$(cd "${TMPDIR%/}/sm/$m" && pwd -P)"; T="$R/.td-misc-run"; mkdir -p "$T"; n="$T/run-$(printf %s "$t" | tr -c 'A-Za-z0-9' _).ts"
sed "s#/Users/ueli/Documents/semio#$R#g; s#@semio-tech/framework-process:test-testing#$t#g$([ -n "$NOART" ] && echo '; s#...(mode==="file"?{artifacts:art}:{})##; s#if(mode==="file")base.SEMIO_TEST_ARTIFACT_DIR=art;##')$([ -n "$ONLYART" ] && echo '; s#const capabilities={#const capabilities={artifacts:art},unusedCapabilities={#')" "$C/ver-f-tools/proc-run.ts" > "$n"
cd "$R" && VF_WALL=300 bun "$n" file "$R/$f" "$@" 2>&1
