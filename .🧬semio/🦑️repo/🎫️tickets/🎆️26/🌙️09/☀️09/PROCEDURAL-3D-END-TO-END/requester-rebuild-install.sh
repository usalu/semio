#!/bin/sh
# 🌀️ Rebuilds the procedural requester component and installs it into the React runtime; retries on transient failures.
cd "$(dirname "$0")/../../../../../../.." || exit 1
W="$(pwd)"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
export SEMIO_LANGUAGE=en
n=0
until [ $n -ge 6 ]; do
  n=$((n+1))
  repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies > "$T/🗑️generated/requester-build-$n.log" 2>&1 && { echo "requester ok attempt $n"; break; }
  echo "requester failed attempt $n: $(sed 's/\x1b\[[0-9;]*m//g' "$T/🗑️generated/requester-build-$n.log" | grep -E '^error|Internal error' | head -2 | cut -c1-250)"
  grep -q "cannot update the lock file" "$T/🗑️generated/requester-build-$n.log" && repo dependencies lock --scope 🌎️hub > /dev/null 2>&1
  grep -q "ENOSPC" "$T/🗑️generated/requester-build-$n.log" && repo cache gc --reclaim=15GiB > /dev/null 2>&1
  [ $n -ge 6 ] && exit 1
  sleep 150
done
cd "$T" && repo tools exec --light bun -- install-local-procedural.ts react "$W/.cache/b/p/m/browser/prototype/modules/🌀️procedural" "$W/.cache/b/p/m/browser/prototype/runtime/react/generation3d" 2>&1 | tail -1 | cut -c1-200
ls -la "$W/.cache/b/p/m/browser/prototype/runtime/react/generation3d/components/🌀️procedural/semio_hub_procedural_component.core.wasm" | awk '{print $5,$6,$7,$8}'
