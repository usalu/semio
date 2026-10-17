#!/bin/zsh
out="$1"; f="$2"
cd /Users/ueli/Documents/semio || exit 2
key=$(printf '%s' "$f" | shasum | cut -c1-12)
log="$out/$key.log"
perl -e 'alarm 240; exec @ARGV' -- bun test --reporter=junit --reporter-outfile="$out/$key.xml" "./$f" > "$log" 2>&1
code=$?
pass=$(/usr/bin/grep -E '^ *[0-9]+ pass$' "$log" | tail -1 | tr -d ' ')
fail=$(/usr/bin/grep -E '^ *[0-9]+ fail$' "$log" | tail -1 | tr -d ' ')
print -r -- "$code|$f|$key|${pass:-?}|${fail:-?}" >> "$out/summary.txt"
