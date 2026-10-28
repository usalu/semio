#!/bin/sh
# keys5-g batch schema-index-key: after `apply`, the laws (graph-free, read-only on the live tree). usage: sh verify.sh
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && git rev-parse --show-toplevel)
cd "$ROOT"
nice bun test --timeout 900000 "$HERE/laws/schema-index-key.test.ts"
nice bun test --timeout 900000 "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🟦️.ts"
