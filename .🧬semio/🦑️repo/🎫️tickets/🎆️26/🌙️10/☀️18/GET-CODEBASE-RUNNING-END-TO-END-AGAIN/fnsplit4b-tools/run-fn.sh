#!/bin/zsh
# usage: run-fn.sh <tag> <realFile> <snapshot> <fnName> old|new '<jsonArgs>' [scopeDir]: calls an exported function of the old snapshot or the live tree and records its outcome and console output; writes C/🗑️generated/fnsplit4b-fn-<tag>-<mode>.json
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
mkdir -p "$C/🗑️generated"
export REG_MODE="$5" REG_REAL_FILE="/Users/ueli/Documents/semio/$2" REG_OLD_FILE="$C/fnsplit4b-tools/orig/$3" REG_FN="$4" FD_ARGS="${6:-[]}"
[ -n "$7" ] && export REG_SCOPE="/Users/ueli/Documents/semio/$7"
export REG_OUT="$C/🗑️generated/fnsplit4b-fn-$1-$5.json"
perl -e 'alarm 300; exec @ARGV' -- bun test "$C/fnsplit4b-tools/fn-diff.test.ts" 2>&1 | command grep -E "RESULT|error:|rror" | head -6
