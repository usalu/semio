#!/usr/bin/env bash
# 🔗️ Runs the fem3d React prototype activation chain natively: `bash native-chain.sh [first-step [last-step]]`.
set -uo pipefail
T="$(cd "$(dirname "$0")" && pwd)" || exit 1
W="$(cd "$T/../../../../../../.." && pwd)" || exit 1
platform="$(node -p 'process.platform')" || exit 1
G="$T/🗑️generated/chain"
mkdir -p "$G"
export SEMIO_BUILD_MODE=prototype SEMIO_PLUGIN=fem3d SEMIO_RENDERER=react
SEMIO_PLUGIN_CATALOG="$(cd "$W" && node -p "require('node:path').resolve('✏️s/🧑‍💻dev/🧩️catalog/🤖️generated')")" || exit 1
export SEMIO_PLUGIN_CATALOG
FEM="🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml"
DRIVER="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts"
WASM="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts"
P="🧰️framework/🛍️products/💻️os/🔨️modules"
steps=(
"01|@semio-tech/s-dev-composition:generate-stdio-format-claims|PLAN"
"02|@semio-tech/s-dev-composition:generate|PLAN"
"03|@semio-tech/plugin-registry:session-fem3d|PLAN"
"04|@semio-tech/framework-plugin-web:support|PLAN"
"05|semio-framework-canvas-font-assets:build|PLAN"
"06|semio-os-infinite:fonts|PLAN"
"06a|workspace:deps-wasm-opt|PLAN"
"06b|workspace:deps-trunk|PLAN"
"06c|workspace:deps-wasm|PLAN"
"06d|@semio-tech/assets:build|PLAN"
"07|@semio-tech/framework-surface-rs:wasm|PLAN"
"08|@semio-tech/framework-editor-rs:wasm|PLAN"
"09|@semio-tech/framework-os-node-graph-rs:wasm|PLAN"
"10|@semio-tech/os-plugin-describe-rs:build|PLAN"
"11|@semio-tech/os-plugin-native-transpile-rs:build|PLAN"
"12|workspace:deps-wasm-opt|.|repo $WASM prepare"
"13|semio-hub-fem-component:component|.|repo $DRIVER component --manifest $FEM"
"14|@semio-tech/framework-os-dev:prepare-fem3d-react-prototype|PLAN"
"15|@semio-tech/framework-os-dev:activate-fem3d-react-prototype|PLAN"
)
first="${1:-01}"
last="${2:-15}"
for row in "${steps[@]}"; do
  IFS='|' read -r n id cwd cmd <<<"$row"
  [[ "$n" < "$first" ]] && continue
  [[ "$n" > "$last" ]] && break
  if [ "$cwd" = PLAN ]; then
    line=$(node "$T/native-plan.mjs" "$id" | tail -1) || exit $?
    cwd=$(printf '%s' "$line" | cut -f2); cmd=$(printf '%s' "$line" | cut -f3)
  fi
  cmd="${cmd//\{hostPlatform\}/$platform}"; cmd="${cmd//\"/}"
  read -r -a argv <<<"${cmd#repo }"
  echo "[$(date +%H:%M:%S)] $n $id :: (cd $cwd) repo ${argv[*]}"
  start=$(date +%s)
  for attempt in $(seq 1 30); do
    bun "$T/native-run.ts" "$W/$cwd" "$id" "${argv[@]}" > "$G/$n.txt" 2>&1
    code=$?
    [ $code -eq 75 ] || break
    echo "[$(date +%H:%M:%S)] $n restartable exit 75, attempt $attempt"
  done
  echo "[$(date +%H:%M:%S)] $n exit=$code ($(( $(date +%s) - start )) s)"
  if [ $code -ne 0 ]; then grep -v '^\s*at ' "$G/$n.txt" | tail -25 | cut -c1-400; exit $code; fi
done
echo CHAIN-DONE
