#!/bin/zsh
# usage: run.sh <root> <file-relative-to-root> [log-dir]   runs one bun suite with the minted launcher envelope against <root> (VERIFY_TSCONFIG: scratch tsconfig `paths` for workspace packages whose install is pending)
ROOT=$1; F=$2; LOGS=${3:-}
TICKET=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
ART=/private/tmp/claude-501/lda/r$$
mkdir -p $ART
export REPO_ROOT=$ROOT SEMIO_WORKSPACE_ROOT=$ROOT SEMIO_REPO_ROOT=$ROOT SEMIO_TICKET_DIR=$TICKET SEMIO_TEST_ARTIFACT_DIR=$ART NO_COLOR=1 SEMIO_BUILD_MODE=prototype
eval "$(bun $TICKET/lib-defects-tools/envwire.ts $ROOT | python3 -c 'import json,sys,shlex;[print("export %s=%s"%(k,shlex.quote(v))) for k,v in json.load(sys.stdin).items()]')"
cd $ROOT
if [ -n "$LOGS" ]; then OUT=$LOGS/$(echo "$F" | python3 -c 'import re,sys;print(re.sub(r"[^A-Za-z0-9]+","_",sys.stdin.read().strip())[-110:])').log; else OUT=/dev/stdout; fi
TS=(); [ -n "$VERIFY_TSCONFIG" ] && TS=(--tsconfig-override "$VERIFY_TSCONFIG")
perl -e 'alarm 280; exec @ARGV' -- bun test $TS --timeout 120000 "./$F" > $OUT 2>&1
rc=$?
rm -rf $ART
[ -n "$LOGS" ] && echo "rc=$rc $F" 
exit $rc
