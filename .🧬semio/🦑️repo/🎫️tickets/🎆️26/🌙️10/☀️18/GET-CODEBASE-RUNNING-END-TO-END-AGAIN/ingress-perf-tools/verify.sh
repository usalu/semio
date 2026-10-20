#!/bin/sh
# ✅ Runs every staged test against the live checkout after apply (explicit file paths only, one runner at a time).
cd /Users/ueli/Documents/semio || exit 2
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:-/private/tmp/ingress-perf-art}"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
S=🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build
L=🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
rc=0
for f in \
  "$S/💾️storage/🍎️image/📡️roster/🧪️tests/🟦️.ts" \
  "$S/💾️storage/📜️journal/🔖️revision/🧪️tests/🟦️.ts" \
  "$S/🎟️admission/🔖️scheduler/🧪️tests/🟦️.ts" \
  "$L/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/💾️storage/🔒️enforcement/🧵️session/🏊️pool/🧪️tests/🟦️.ts" \
  "$L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🧹️stamp/🟦️.ts" \
  "$S/💾️storage/🍎️image/📥️information/🧪️tests/🟦️.ts"; do
  echo "== $f"; perl -e 'alarm 280; exec @ARGV' -- bun test "./$f" 2>&1 | tail -6 || rc=1
done
exit $rc
