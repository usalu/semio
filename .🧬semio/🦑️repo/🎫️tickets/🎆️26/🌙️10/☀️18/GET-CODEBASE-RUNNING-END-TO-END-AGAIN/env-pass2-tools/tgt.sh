#!/bin/zsh
# usage: tgt.sh <projectDir-relative-to-repo> <target-name> <cmd words...>   (runs the authored target command under the minted process envelope)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
DIR="$1"; NAME="$2"; shift 2
OUT="$C/🗑️generated/env-pass2-tgt"; mkdir -p "$OUT"
perl -e 'alarm '"${TGT_ALARM:-540}"'; exec @ARGV' -- bun "$C/env-pass2-tools/wrapcmd.ts" "$NAME" "$DIR" "$@" > "$OUT/$NAME.log" 2>&1
RC=$?
P=$(command grep -a -E '^ *[0-9]+ pass$' "$OUT/$NAME.log" | awk '{s+=$1} END{print s+0}')
X=$(command grep -a -E '^ *[0-9]+ fail$' "$OUT/$NAME.log" | awk '{s+=$1} END{print s+0}')
echo "$NAME	rc=$RC	pass=$P	fail=$X" >> "$C/env-pass2-tools/summary-tgt.tsv"
