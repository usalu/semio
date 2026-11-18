#!/usr/bin/env bash
# Native replacement of the inferred release prepare stage for the nine demonstrator lanes. Usage: native-lanes.sh <stage> [args]
set -u
ROOT="/Users/ueli/Documents/semio"
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS"
G="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI/🗑️generated/demo-native"; mkdir -p "$G/done"
export SEMIO_BUILD_MODE=prototype
export SEMIO_WORKSPACE_ROOT="$ROOT"
export SEMIO_PLUGIN_CATALOG="$ROOT/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
VARIANTS="aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen"
step() { local name="$1" cwd="$2"; shift 2
  if [ -f "$G/done/$name" ] && [ -z "${FORCE:-}" ]; then echo "[skip] $name"; return 0; fi
  echo "[run] $name"; ( cd "$ROOT/$cwd" && repo "$T/native-launch.ts" "${cwd:-.}" "$@" ) >"$G/$name.log" 2>&1; local rc=$?
  if [ $rc -eq 0 ]; then touch "$G/done/$name"; echo "[ok] $name"; else echo "[FAIL $rc] $name (see $G/$name.log)"; fi; return $rc; }
FW="🧰️framework"; OSM="$FW/🛍️products/💻️os/🔨️modules"
prereqs() {
  step wasm-opt "" repo "$FW/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts" prepare
  step plugin-web-support "$OSM/🔌️plugin/📦️packages/🟦️typescript" repo ./📜️script.ts support
  step font-assets "$FW/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust" repo ./📜️script.ts build
  step infinite-fonts "$OSM/♾️infinite/📦️packages/🦀️rust" repo ./📜️script.ts fonts
  step describe-build "$OSM/🔌️plugin/🖨️describe/📦️packages/🦀️rust" repo ./📜️script.ts build --host-platform=darwin
}
catalog() {
  step stdio-format-claims "✏️s/🧑‍💻dev" repo ./📜️script.ts stdio-format-claims
  step catalog-generate "✏️s/🧑‍💻dev" repo ./🧩️catalog/📣️publication/📜️script.ts generate
}
components() {
  for crate in "🌎️hub/🧩️compositions/🎪️demonstrator" "🌎️hub/🧩️compositions/🔋️energy" "🌎️hub/🧩️compositions/🏗️fem" "🌎️hub/🧩️compositions/🌀️procedural"; do
    step "component-$(basename "$crate")" "" repo "$OSM/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts" component --manifest "$crate/🧩️component/📦️packages/🦀️rust/Cargo.toml" || return $?
  done
}
session() { step "session-$1" "$OSM/🔌️plugin/📇️registry" repo ./📜️script.ts session "$1"; }
prepare() { step "prepare-$1" "$OSM/🧑‍💻dev/📦️packages/🟦️typescript" repo ./♻️activation/🧰️preparation/📜️script.ts prepare "$1" react release; }
"$@"
