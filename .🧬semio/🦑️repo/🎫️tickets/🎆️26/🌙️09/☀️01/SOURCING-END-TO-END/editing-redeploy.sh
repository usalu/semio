#!/bin/sh
# 🚢️ Serialized rebuild, restage and host restart of the sourcing hub component for the editing journey (shares `🗑️generated/build.lock` with the other agents).
# Usage: editing-redeploy.sh   (from the workspace root; log lines go to stdout)
set -u
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
until mkdir "$T/🗑️generated/build.lock" 2>/dev/null; do sleep 30; done
echo "lock acquired $(date)"
status=0
ONLY="semio-hub-sourcing-component" ONLY_HUB=1 sh "$T/native-wasm-build.sh" || status=$?
if [ "$status" -eq 0 ]; then
  BUN=$(ls -d ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
  SEMIO_NATIVE_TRANSPILE_BIN="${SEMIO_NATIVE_TRANSPILE_BIN:-$PWD/.cache/b/p/m/browser/prototype/tools/native-transpile/darwin/semio-native-transpile}" "$BUN" "$T/native-stage-modules.ts" sourcing || status=$?
  if [ "$status" -eq 0 ]; then
    pkill -f serve-sourcing-native.ts; lsof -ti tcp:6081 | xargs kill 2>/dev/null
    nohup "$BUN" "$T/serve-sourcing-native.ts" > "$T/🗑️generated/serve-native.txt" 2>&1 &
    until grep -q ready "$T/🗑️generated/serve-native.txt" 2>/dev/null; do sleep 3; done
  fi
fi
rmdir "$T/🗑️generated/build.lock"
echo "redeploy exit $status"
