#!/bin/sh
# 🧪 Runs the memo law suites (plus the invocation neighbours) inside a root: sh memotests.sh <root> [extra test files...]
root=$1; shift
cd "$root" || exit 2
M=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo
I=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests
[ -n "${SEMIO_REPO_WATCH_BINARY:-}" ] || SEMIO_REPO_WATCH_BINARY=$(ls -t /Users/ueli/Documents/semio/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
[ -n "$SEMIO_REPO_WATCH_BINARY" ] && export SEMIO_REPO_WATCH_BINARY
bun test --timeout 300000 "$@" "$M/🧪️tests/🟦️.ts" "$M/🧪️tests/🎯️slice/🟦️.ts" "$M/🧪️tests/🔂️reseal/🟦️.ts" "$M/🧪️tests/🕰️verified/🟦️.ts" "$M/🧪️tests/🧳️carry/🟦️.ts" "$M/🧪️tests/🧹️retention/🟦️.ts" "$I/🧊️isolated-plan/🟦️.ts" "$I/📝️plan-file/🟦️.ts"
