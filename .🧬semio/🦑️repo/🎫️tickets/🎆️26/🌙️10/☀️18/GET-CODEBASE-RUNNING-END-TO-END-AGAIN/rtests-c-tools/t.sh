#!/bin/sh
# usage: t.sh <test dir name under library tests | full path> -> prints summary + first errors
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
f="$1"
case "$f" in
  /*|🧰*) ;;
  *) f="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/$f/🟦️.ts";;
esac
out="$C/🗑️generated/rtests-c/t.log"
sh "$C/rtests-c-tools/one.sh" "$f" > "$out" 2>&1
/usr/bin/grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' "$out" | head -${2:-12}
