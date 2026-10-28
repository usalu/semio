#!/bin/sh
# Laws of `vouch` (graph-free single-file tests, run AFTER apply). Real-run proof: `SEMIO_REPO_PROFILE=1 bun repo dependencies ensure` twice: the second prints a `dependencies.vouch` mark `journal` and dependencies.verify + dependencies.recheck shrink from seconds to well under one second.
R=$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)
cd "$R" || exit 1
N="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes"
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
status=0
binary=$(ls "$R"/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
[ -n "$binary" ] && export SEMIO_REPO_WATCH_BINARY="$binary" || echo "no compiled watcher: the REAL journal law is skipped"
for file in "$N/🪪️vouch/🧪️tests/🟦️.ts" "$N/🧪️tests/🟦️.ts" "$G/🧪️tests/🟦️.ts"; do
  out=$(bun test --timeout 180000 "./$file" 2>&1); echo "$out" | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"
  echo "$out" | grep -qE "^ *0 fail" || status=1
done
exit $status
