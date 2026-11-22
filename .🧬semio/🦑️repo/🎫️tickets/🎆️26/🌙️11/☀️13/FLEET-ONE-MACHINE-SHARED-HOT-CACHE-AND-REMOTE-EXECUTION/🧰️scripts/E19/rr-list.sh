#!/bin/zsh
S="$(dirname "$0")"
while read f; do [ -z "$f" ] && continue; out=$("$S/rr.sh" "/Users/ueli/Documents/semio/$f"); rc=$?; echo "rc=$rc $(echo "$out" | grep -E '^\s*[0-9]+ (pass|fail)$' | tr -s ' \n' ' ') :: ${f#*library/}"; done
