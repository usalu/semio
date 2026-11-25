#!/bin/sh
# 🔁️ Locked rebuild, restage and host restart of the sourcing hub component for the history/import-export journey.
# Usage: [ONLY="pkg"] [PLUGINS="sourcing ..."] [NOBUILD=1] sh rebuild-history.sh   (run from the workspace root)
# @see ./native-wasm-build.sh
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
lock="$T/🗑️generated/build.lock"
until mkdir "$lock" 2>/dev/null; do sleep 30; done
trap 'rmdir "$lock"' EXIT
BUN=$(ls -d ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
export SEMIO_NATIVE_TRANSPILE_BIN="${SEMIO_NATIVE_TRANSPILE_BIN:-$PWD/.cache/b/p/m/browser/prototype/tools/native-transpile/darwin/semio-native-transpile}"
if [ -z "${NOBUILD:-}" ]; then
  ONLY="${ONLY:-semio-hub-sourcing-component}" ONLY_HUB=1 sh "$T/native-wasm-build.sh" || exit 1
  for p in ${PLUGINS:-sourcing}; do "$BUN" "$T/native-stage-modules.ts" "$p" || exit 1; done
fi
pkill -f serve-sourcing-native.ts; lsof -ti tcp:6081 | xargs kill 2>/dev/null; sleep 2
nohup "$BUN" "$T/serve-sourcing-native.ts" > "$T/🗑️generated/serve-native.txt" 2>&1 &
i=0; until grep -q ready "$T/🗑️generated/serve-native.txt" 2>/dev/null || [ $i -gt 60 ]; do sleep 2; i=$((i+1)); done
cat "$T/🗑️generated/serve-native.txt"
