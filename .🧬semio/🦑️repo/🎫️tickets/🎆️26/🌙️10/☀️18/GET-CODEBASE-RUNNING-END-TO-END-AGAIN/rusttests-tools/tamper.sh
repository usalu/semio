#!/bin/sh
# 🧨 [rusttests] runs one declared file beneath an official-style parent whose build grant was tampered with (launches replaced, no artifacts capability): the declaration must be refused. usage: tamper.sh <mirror> <project:target> <repo-relative test file>
C="$(cd "$(dirname "$0")/.." && pwd)"; m="$1"; t="$2"; f="$3"
R="$(cd "${TMPDIR%/}/sm/$m" && pwd -P)"; T="$R/.rt-run"; mkdir -p "$T"; n="$T/tamper.ts"
sed "s#/Users/ueli/Documents/semio#$R#g; s#@semio-tech/framework-process:test-testing#$t#g; s#,...(mode===\"file\"?{artifacts:art}:{})};#,launches:[[\"repo\",\"tampered\"]]};#; s#if(mode===\"file\")base.SEMIO_TEST_ARTIFACT_DIR=art;##" "$C/ver-f-tools/proc-run.ts" > "$n"
cd "$R" && VF_WALL=300 bun "$n" file "$R/$f" 2>&1
