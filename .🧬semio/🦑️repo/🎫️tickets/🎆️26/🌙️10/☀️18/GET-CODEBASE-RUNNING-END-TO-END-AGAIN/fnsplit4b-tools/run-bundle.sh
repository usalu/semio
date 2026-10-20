#!/bin/zsh
# usage: run-bundle.sh <tag> <realFile> <snapshot> <fnName> old|new [argc] [vitestArg] [sourceArg]: bundled registration trace with stubs for unresolvable imports; writes C/🗑️generated/fnsplit4b-reg-<tag>-<mode>.json
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
mkdir -p "$C/🗑️generated"
export REG_MODE="$5" REG_REAL_FILE="/Users/ueli/Documents/semio/$2" REG_OLD_FILE="$C/fnsplit4b-tools/orig/$3" REG_FN="$4" REG_ARGC="${6:-3}" REG_VITEST_ARG="${7:-0}" REG_SOURCE_ARG="${8:-$(( ${6:-3} - 1 ))}"
export REG_OUT="$C/🗑️generated/fnsplit4b-reg-$1-$5.json"
perl -e 'alarm 300; exec @ARGV' -- bun $C/fnsplit4b-tools/reg-bundle.ts 2>&1 | tail -12
