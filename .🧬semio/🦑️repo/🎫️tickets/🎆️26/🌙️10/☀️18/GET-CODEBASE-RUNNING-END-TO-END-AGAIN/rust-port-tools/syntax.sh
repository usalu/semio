#!/bin/sh
# syntax-only check (rustfmt parse, nothing is written): syntax.sh <file>...
for f in "$@"; do
  out=$(rustfmt --edition 2021 --emit stdout "$f" 2>&1 >/dev/null | /usr/bin/grep -E '^error' | head -3)
  [ -n "$out" ] && printf '%s\n%s\n' "$f" "$out"
done
exit 0
