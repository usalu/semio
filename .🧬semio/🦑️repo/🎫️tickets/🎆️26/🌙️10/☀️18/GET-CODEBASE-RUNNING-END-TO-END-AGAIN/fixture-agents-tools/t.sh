#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
export REPO_ROOT=$PWD SEMIO_TEST_ARTIFACT_DIR=$PWD/$C/🗑️generated/l/fa$$ NO_COLOR=1 SEMIO_REPO_ROOT=$PWD SEMIO_TICKET_DIR=$C SEMIO_BUILD_MODE=prototype
eval "$(bun $C/law-drift-tools/envwire.ts | python3 -c 'import json,sys,shlex;[print("export %s=%s"%(k,shlex.quote(v))) for k,v in json.load(sys.stdin).items()]')"
mkdir -p $SEMIO_TEST_ARTIFACT_DIR
f="$1"; shift
perl -e 'alarm ${ALARM:-590}; exec @ARGV' -- bun test --timeout 300000 "$@" "./$f" 2>&1
rm -rf $SEMIO_TEST_ARTIFACT_DIR
