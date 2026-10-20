#!/bin/zsh
# usage: cmp2.sh BASE_OVERLAY PATCHED_OVERLAY FILE... ; pass/fail tallies side by side (names diffed when tallies differ)
A="$1"; B="$2"; shift 2
for f in "$@"; do
  a=$(/private/tmp/xplat-linux-fix/tw.sh "$A" "$f" 2>&1 | /usr/bin/grep -E '^\((pass|fail)\)' | sed -E 's/ \[[0-9.]+m?s\]//' | sort)
  b=$(/private/tmp/xplat-linux-fix/tw.sh "$B" "$f" 2>&1 | /usr/bin/grep -E '^\((pass|fail)\)' | sed -E 's/ \[[0-9.]+m?s\]//' | sort)
  pa=$(echo "$a" | /usr/bin/grep -c '^(pass)'); fa=$(echo "$a" | /usr/bin/grep -c '^(fail)'); pb=$(echo "$b" | /usr/bin/grep -c '^(pass)'); fb=$(echo "$b" | /usr/bin/grep -c '^(fail)')
  same="same names"; [ "$a" = "$b" ] || same="NAMES DIFFER"
  echo "${f: -62} | base ${pa}p/${fa}f | patched ${pb}p/${fb}f | $same"
  [ "$a" = "$b" ] || diff <(echo "$a") <(echo "$b") | head -6
done
