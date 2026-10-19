#!/bin/zsh
# usage: verify.sh <changed.txt>
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$T/🗑️generated/comments-b"
L="$1"
ts=0; tsfail=0; rs=0; rsfail=0
: > "$O/verify-fail.txt"
while IFS= read -r f; do
  [ -f "$f" ] || continue
  case "$f" in
    *.rs)
      rs=$((rs+1))
      if ! rustfmt --edition 2021 --emit stdout < "$f" > /dev/null 2> "$O/verify-err.txt"; then
        if command grep -q '^error' "$O/verify-err.txt"; then rsfail=$((rsfail+1)); echo "RS $f" >> "$O/verify-fail.txt"; fi
      fi;;
    *)
      ts=$((ts+1))
      if ! bun build --no-bundle "$f" --outfile="$O/verify-out.js" > /dev/null 2> "$O/verify-err.txt"; then tsfail=$((tsfail+1)); echo "TS $f" >> "$O/verify-fail.txt"; fi;;
  esac
done < "$L"
echo "ts=$ts tsfail=$tsfail rs=$rs rsfail=$rsfail"
