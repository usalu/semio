#!/bin/zsh
# 🧾️ Parse check: rustfmt (skip_children) on every changed .rs file passed; prints files with parse errors.
cd /Users/ueli/Documents/semio
bad=0
for f in "$@"; do
  out=$(rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --check "$f" 2>&1 >/dev/null | /usr/bin/grep -E "^error" | head -3)
  if [ -n "$out" ]; then echo "PARSE-ERROR $f: $out"; bad=1; fi
done
[ $bad = 0 ] && echo "rustfmt parse: $# files ok"
