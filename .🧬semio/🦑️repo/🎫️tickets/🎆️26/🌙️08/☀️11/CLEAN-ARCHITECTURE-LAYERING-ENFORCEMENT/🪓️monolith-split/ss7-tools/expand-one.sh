#!/bin/sh
f="$1"
[ -f "$f" ] || exit 0
out=$(rustc +nightly-2026-07-20 --edition 2021 --test -Zunpretty=expanded --error-format=short --crate-type=lib "$f" 2>&1 >/dev/null | /usr/bin/grep -E ': error' | /usr/bin/grep -v -E 'error\[E[0-9]+\]|cannot find (macro|derive macro|attribute macro)|proc.macro|aborting due')
if [ -n "$out" ]; then
  printf 'FILE\t%s\n%s\n' "$f" "$out"
fi
