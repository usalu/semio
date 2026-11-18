#!/usr/bin/env bash
set -u
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../../../../../../.." && pwd)"
logs="$here/🗑️generated"; mkdir -p "$logs"
export SEMIO_BUILD_MODE="${SEMIO_BUILD_MODE:-prototype}"
os="🧰️framework/🛍️products/💻️os/🔨️modules"
step() { local name="$1" cwd="$2"; shift 2; echo "[$(date +%T)] $name"; timeout "${STEP_TIMEOUT:-3000}" repo "$here/native-exec.ts" "$cwd" "$@" >"$logs/native-$name.log" 2>&1; local rc=$?; echo "[$(date +%T)] $name rc=$rc"; return $rc; }
phase="${1:-all}"
if [ "$phase" = all ] || [ "$phase" = tools ]; then
  step wasm-opt . "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts" prepare
  step describe "$os/🔌️plugin/🖨️describe/📦️packages/🦀️rust" ./📜️script.ts build "--host-platform=$(uname -s | tr A-Z a-z | sed 's/darwin/darwin/')"
  step support "$os/🔌️plugin/📦️packages/🟦️typescript" ./📜️script.ts support
  step fonts-assets "🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust" ./📜️script.ts build
  step fonts "$os/♾️infinite/📦️packages/🦀️rust" ./📜️script.ts fonts
fi
if [ "$phase" = describe ]; then
  step describe "$os/🔌️plugin/🖨️describe/📦️packages/🦀️rust" ./📜️script.ts build "--host-platform=$(uname -s | tr A-Z a-z)"
fi
if [ "$phase" = all ] || [ "$phase" = engines ]; then
  for e in "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust" "🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust" "$os/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust"; do
    step "wasm-$(basename "$(dirname "$(dirname "$e")")")" "$e" ./📜️script.ts wasm
  done
fi
if [ "$phase" = all ] || [ "$phase" = component ]; then
  N="🌎️hub/🧩️compositions/📕️norm"
  for c in "norm:$N/🧩️component" "norm-eurocode:$N/🧩️extensions/📗️eurocode/🧩️component" "norm-eurocode-materials:$N/🧩️extensions/📙️eurocode-materials/🧩️component" "norm-eurocode-structural:$N/🧩️extensions/📓️eurocode-structural/🧩️component" "norm-energy:$N/🧩️extensions/📒️energy/🧩️component"; do
    case " ${ONLY_COMPONENTS:-${c%%:*}} " in *" ${c%%:*} "*) ;; *) continue ;; esac
    step "component-${c%%:*}" . "$os/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts" component --manifest "${c#*:}/📦️packages/🦀️rust/Cargo.toml" || echo "FAILED component-${c%%:*}"
  done
fi
if [ "$phase" = all ] || [ "$phase" = lane ]; then
  lane="$os/🧑‍💻dev/📦️packages/🟦️typescript"
  for v in ${ONLY_LANES:-din4108 din18599 en1990 en1992 en1995}; do
    SEMIO_PLUGIN_CATALOG="$root/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated" step "session-$v" "$os/🔌️plugin/📇️registry" ./📜️script.ts session "$v" &&
    SEMIO_PLUGIN_CATALOG="$root/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated" step "prepare-$v" "$lane" ./📜️script.ts prepare "$v" react prototype &&
    SEMIO_PLUGIN_CATALOG="$root/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated" step "activate-$v" "$lane" ./📜️script.ts activate "$v" react prototype || echo "FAILED lane-$v"
  done
fi
if [ "$phase" = serve ]; then
  mkdir -p "$root/.cache/b/p/m/vite/play-${PLAY_SCOPE:-standards}" && cd "$root/🏢️semio-tech/🎡️play" && SEMIO_VITE_CACHE_DIRECTORY="$root/.cache/b/p/m/vite/play-${PLAY_SCOPE:-standards}" SEMIO_TECH_PLAY_FROZEN="${SEMIO_TECH_PLAY_FROZEN:-true}" SEMIO_TECH_PLAY_SCOPE="${PLAY_SCOPE:-standards}" SEMIO_TECH_PLAY_PORT="${PLAY_PORT:-6033}" SEMIO_BUILD_MODE=prototype NODE_OPTIONS= exec repo "$here/native-exec.ts" "🏢️semio-tech/🎡️play" ./🔨️modules/🧩️runtime/📜️script.ts serve
fi
