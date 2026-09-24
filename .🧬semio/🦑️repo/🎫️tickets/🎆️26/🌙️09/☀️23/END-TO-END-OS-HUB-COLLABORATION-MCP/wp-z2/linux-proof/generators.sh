#!/usr/bin/env bash
# Z2 fresh-clone generator probe: runs each `prepare` generator's own command in dependency-free order on a checkout
# with no `🤖️generated` outputs, printing the first missing-module error per generator.
set -u
cd /src
export PATH="/root/.bun/bin:/root/.cargo/bin:/root/.local/bin:$PATH" NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4
run() {
  local label="$1" dir="$2"
  shift 2
  local start
  start="$(date +%s)"
  (cd "/src/$dir" && "$@") >"/tmp/gen-$label.log" 2>&1
  local status=$?
  echo "[z2-gen] $label status=$status elapsed=$(($(date +%s) - start))s :: $(grep -m1 -E 'Cannot find module|error' "/tmp/gen-$label.log" | cut -c1-300)"
}
for step in ${Z2_GENERATORS:-ui-rs ui-styling-tokens framework-schema framework-graph assets generator-inputs plugin-registry}; do
  case "$step" in
  ui-rs) run ui-rs "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust" bun ./📜️script.ts generate ;;
  ui-styling-tokens) run ui-styling-tokens "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust" bun ./📜️script.ts generate ;;
  framework-schema) run framework-schema "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust" bun ./📜️script.ts generate ;;
  framework-graph) run framework-graph "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust" bun ./📜️script.ts generate ;;
  assets) run assets "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript" bun ./📜️script.ts build ;;
  generator-inputs) run generator-inputs "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching" bun ./🔏️inputs/📜️script.ts registry-catalog ;;
  plugin-registry) run plugin-registry "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry" bun ./📜️script.ts generate ;;
  esac
done
