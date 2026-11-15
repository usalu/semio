#!/bin/bash
# usage: 1113-run-suites.sh <repo-or-temp-root> [suite-substring]  -- runs every shared-worktree suite, prints pass/fail per file
ROOT="$1"; ONLY="$2"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
W="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
run(){
  local dir="$1" file="$2"
  [ -n "$ONLY" ] && [[ "$dir/$file" != *"$ONLY"* ]] && return
  [ -f "$W/$dir/$file" ] || return
  cd "$W/$dir" || return
  out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE "$BUN" test "./$file" --timeout 60000 2>&1)
  echo "$dir/$file: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)$' | tr -s ' ' | tr '\n' ' ')"
  echo "$out" | grep -E "^\(fail\)|error:" | head -10
}
run "📒️journal/🧪️tests" "🟦️.ts"
run "📒️journal/🧪️tests" "💾️store.ts"
run "🚧️boundary/🧪️tests" "🟦️.ts"
run "🌿️git/🧪️tests" "🟦️.ts"
run "🩺️attest/🧪️tests" "🟦️.ts"
run "🔁️replica/🧪️tests" "🟦️.ts"
run "🔁️replica/🧪️tests" "🏁️race.ts"
run "🔁️replica/🧪️tests" "🌿️git.ts"
run "🔁️replica/🧪️tests" "💾️store.ts"
run "🔁️replica/🧪️tests" "🌪️chaos.ts"
run "🔁️replica/🥾️bootstrap/🧪️tests" "🟦️.ts"
run "🗺️topology/🧪️tests" "🟦️.ts"
