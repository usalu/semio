#!/bin/sh
f="$1"
[ -f "$f" ] || exit 0
out=$(rustc +nightly-2026-07-20 --edition 2021 -Zunpretty=normal --error-format=short --crate-type=lib "$f" 2>&1 >/dev/null)
rc=$?
if [ $rc -ne 0 ]; then
  printf 'FILE\t%s\n%s\n' "$f" "$out"
fi
