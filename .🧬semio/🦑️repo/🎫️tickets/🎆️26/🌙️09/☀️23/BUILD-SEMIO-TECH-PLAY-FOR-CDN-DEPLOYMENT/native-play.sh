#!/usr/bin/env bash
# Native fallback of `repo run @semio-tech/semio-tech-play:build` when the graph service stalls. Usage: native-play.sh <prereqs|site|all|routing|browsers|verify|step NAME CWD CMD...>
set -u
ROOT="/Users/ueli/Documents/semio"
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
G="$T/🗑️generated/native"; mkdir -p "$G/done"
export SEMIO_BUILD_MODE=release SEMIO_WORKSPACE_ROOT="$ROOT" SEMIO_TICKET_DIR="$T" SEMIO_UNIT_CACHE="${SEMIO_UNIT_CACHE:-off}"
step() { local name="$1" cwd="$2"; shift 2
  if [ -f "$G/done/$name" ] && [ -z "${FORCE:-}" ]; then echo "[skip] $name"; return 0; fi
  echo "[run] $name $(date +%T)"; ( cd "$ROOT/$cwd" && repo capacity run $(if [ -n "${OP:-}" ]; then echo "--operation=$OP --jobs=${JOBS:-1}"; else echo "--memory=${MEM:-3000}"; fi) -- repo "$T/${LAUNCH:-native-launch.ts}" ${TASK:+"$TASK"} "${cwd:-.}" "$@" ) >"$G/$name.log" 2>&1; local rc=$?
  if [ $rc -eq 0 ]; then touch "$G/done/$name"; echo "[ok] $name $(date +%T)"; else echo "[FAIL $rc] $name (see $G/$name.log)"; tail -25 "$G/$name.log" | cut -c1-300; fi; return $rc; }
prereqs() {
  step wasm-opt "" repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts" prepare &
  step assets "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript" repo ./📜️script.ts build &
  step registry-generate "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry" repo ./📜️script.ts generate &
  wait
}
catalog() {
  step stdio-format-claims "✏️s/🧑‍💻dev" repo ./📜️script.ts stdio-format-claims && step catalog-generate "✏️s/🧑‍💻dev" repo ./🧩️catalog/📣️publication/📜️script.ts generate
}
generators() {
  python3 "$T/native-generators.py" "$ROOT" | while IFS=$'\t' read -r name cwd cmd; do eval "set -- $cmd"; MEM=1500 step "gen-$name" "$cwd" "$@" </dev/null; done
}
site() { for attempt in 1 2 3 4 5 6 7 8; do FORCE=1 catalog || return 1; [ -f "$G/site-build.log" ] && mv "$G/site-build.log" "$G/site-build-$(date +%H%M%S).log"; LAUNCH=native-build-launch.ts TASK=@semio-tech/semio-tech-play:build OP=${OP:-cargo} JOBS=${JOBS:-6} step site-build "🏢️semio-tech/🎡️play" repo ./🔨️modules/📦️site/📜️script.ts build && return 0; echo "[retry] site-build attempt $attempt failed"; done; return 1; }
PLAY="🏢️semio-tech/🎡️play"
routing() { FORCE=1 step release-routing "" "$(ls -d "$ROOT"/.cache/b/t/m/bun/*/bun | head -1)" test "./$PLAY/🧪️tests/🧪️playrelease/🟦️.ts"; }
browsers() { ls "$ROOT"/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-* >/dev/null 2>&1 && ls "$ROOT"/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium_headless_shell-* >/dev/null 2>&1 || FORCE=1 step browsers "" repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync browsers; }
verify() { routing && browsers && FORCE=1 step release-verify "$PLAY" repo ./🔨️modules/📦️site/📜️script.ts verify && MEM=${MEM:-4000} FORCE=1 step test-release "$PLAY" repo ./🔨️modules/🧪️e2e/📜️script.ts test-release; }
all() { prereqs; catalog && generators && site; }
"$@"
