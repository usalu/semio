#!/usr/bin/env bash
# Native fallback of `repo run @semio-tech/semio-tech-play:build` when the graph service stalls. Usage: native-play.sh <prereqs|site|all|routing|browsers|verify|step NAME CWD CMD...>
set -uo pipefail
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" || exit 1
ROOT="$T"
while [ ! -d "$ROOT/.🧬semio" ]; do
  parent="$(dirname "$ROOT")"
  [ "$parent" != "$ROOT" ] || exit 1
  ROOT="$parent"
done
G="$T/🗑️generated/native"; mkdir -p "$G/done"
export SEMIO_BUILD_MODE=release SEMIO_WORKSPACE_ROOT="$ROOT" SEMIO_TICKET_DIR="$T" SEMIO_UNIT_CACHE="${SEMIO_UNIT_CACHE:-off}"
step() { local name="$1" cwd="$2"; shift 2
  if [ -f "$G/done/$name" ] && [ -z "${FORCE:-}" ]; then echo "[skip] $name"; return 0; fi
  echo "[run] $name $(date +%T)"; ( cd "$ROOT/$cwd" && repo capacity run $(if [ -n "${OP:-}" ]; then echo "--operation=$OP --jobs=${JOBS:-1}"; else echo "--memory=${MEM:-3000}"; fi) -- repo "$T/${LAUNCH:-native-launch.ts}" ${TASK:+"$TASK"} "${cwd:-.}" "$@" ) >"$G/$name.log" 2>&1; local rc=$?
  if [ $rc -eq 0 ]; then touch "$G/done/$name"; echo "[ok] $name $(date +%T)"; else echo "[FAIL $rc] $name (see $G/$name.log)"; tail -25 "$G/$name.log" | cut -c1-300; fi; return $rc; }
prereqs() {
  step wasm-opt "" repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts" prepare &
  local wasm_pid=$!
  step assets "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript" repo ./📜️script.ts build &
  local assets_pid=$!
  step registry-generate "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry" repo ./📜️script.ts generate &
  local registry_pid=$! failed=0
  wait "$wasm_pid" || failed=1
  wait "$assets_pid" || failed=1
  wait "$registry_pid" || failed=1
  return "$failed"
}
catalog() {
  MEM=${MEM:-512} step stdio-format-claims "✏️s/🧑‍💻dev" repo ./📜️script.ts stdio-format-claims &&
    MEM=${MEM:-1280} step catalog-generate "✏️s/🧑‍💻dev" repo ./🧩️catalog/📣️publication/📜️script.ts generate
}
generators() {
  python3 "$T/native-generators.py" "$ROOT" | while IFS=$'\t' read -r name cwd cmd; do eval "set -- $cmd"; MEM=1500 step "gen-$name" "$cwd" "$@" </dev/null || exit $?; done
}
boundaries() { BUN=$(ls -d "$ROOT"/.cache/b/t/m/bun/*/bun | head -1); "$BUN" "$T/native-source-boundary-preflight.ts" "$ROOT" "✏️s" "🌎️hub" "🧰️framework" > "$G/boundaries.tsv" 2>&1 || return $?; if grep -q "^OVER" "$G/boundaries.tsv"; then echo "[FAIL] source boundaries"; grep "^OVER" "$G/boundaries.tsv"; return 1; fi; echo "[ok] source boundaries"; }
site() { for attempt in 1 2 3 4 5 6 7 8; do FORCE=1 catalog || { echo "[retry] catalog attempt $attempt failed"; sleep 180; continue; }; boundaries || return 1; [ -f "$G/site-build.log" ] && mv "$G/site-build.log" "$G/site-build-$(date +%H%M%S).log"; FORCE=1 LAUNCH=native-build-launch.ts TASK=@semio-tech/semio-tech-play:build OP=${OP:-cargo} JOBS=${JOBS:-6} step site-build "🏢️semio-tech/🎡️play" repo ./🔨️modules/📦️site/📜️script.ts build && return 0; echo "[retry] site-build attempt $attempt failed"; done; return 1; }
PLAY="🏢️semio-tech/🎡️play"
routing() { FORCE=1 step release-routing "" "$(ls -d "$ROOT"/.cache/b/t/m/bun/*/bun | head -1)" test "./$PLAY/🧪️tests/🧪️playrelease/🟦️.ts"; }
browsers() { ls "$ROOT"/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-* >/dev/null 2>&1 && ls "$ROOT"/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium_headless_shell-* >/dev/null 2>&1 || FORCE=1 step browsers "" repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync browsers; }
verify() {
  local gpu="${PLAY_E2E_GPU:-}"
  routing && browsers || return $?
  if ! PLAY_E2E_GPU="$gpu" FORCE=1 step verify-browser "" repo "$T/📜️script.ts" browser-capabilities "$ROOT"; then
    [ -z "$gpu" ] || return 1
    if [[ "$OSTYPE" == darwin* ]] && PLAY_E2E_GPU=windowed FORCE=1 step verify-browser-windowed "" repo "$T/📜️script.ts" browser-capabilities "$ROOT"; then
      gpu=windowed
    else
      gpu=swiftshader
      PLAY_E2E_GPU="$gpu" FORCE=1 step verify-browser-swiftshader "" repo "$T/📜️script.ts" browser-capabilities "$ROOT" || return $?
    fi
  fi
  FORCE=1 step release-verify "$PLAY" repo ./🔨️modules/📦️site/📜️script.ts verify &&
    PLAY_E2E_GPU="$gpu" MEM=${MEM:-4000} FORCE=1 step test-release "$PLAY" repo ./🔨️modules/🧪️e2e/📜️script.ts test-release
}
all() { prereqs && catalog && generators && site; }
"$@"
