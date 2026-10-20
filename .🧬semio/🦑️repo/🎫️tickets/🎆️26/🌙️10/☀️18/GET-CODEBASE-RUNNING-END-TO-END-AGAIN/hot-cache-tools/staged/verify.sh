#!/bin/sh
# builds the natives, applies the staged set to a scratch root (no edits of existing files) and runs every staged suite there
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${1:-/private/tmp/hot-cache-verify}
rm -rf "$OUT" && mkdir -p "$OUT/bin" "$OUT/root"
python3 "$HERE/build-native.py" "$OUT/bin" --test
python3 "$HERE/apply.py" --root "$OUT/root" --no-edits --write
python3 "$HERE/run-staged-tests.py" "$OUT/root" "$OUT/bin"
