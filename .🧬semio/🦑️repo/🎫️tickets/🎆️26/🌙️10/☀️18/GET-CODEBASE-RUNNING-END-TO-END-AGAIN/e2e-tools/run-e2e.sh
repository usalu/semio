#!/usr/bin/env bash
# Ticket input script (not permanent tooling): runs the exact end-to-end chain of one product through the neutral entry.
#
#   run-e2e.sh <play|demonstrator|course> [--tool=nx|bazel] [options] [-- <extra test-e2e arguments>]
#
# Chain (play, demonstrator):  activate-prototype -> serve (detached, port polled) -> test-e2e -> serve stopped
# Chain (course, dev topology): test-e2e --topology dev   (the gate boots its own throw-away proctor + dev site)
#
# Options:
#   --tool=nx|bazel        passed to `bun repo` (default: the repo default, nx)
#   --install-browsers     run `bun repo run workspace:deps-browsers` first when Playwright's browsers are missing (DOWNLOADS ~300 MB)
#   --skip-activate        play/demonstrator: assume activate-prototype is already green
#   --no-serve             play/demonstrator: skip the detached dev serve (test-e2e owns its own port-0 server either way)
#   --serve                course: also start `course:dev` detached (port 6061) as a smoke before the gate; off by default
#   --topology=<dev|rehearsal|both>   course topology (default dev; rehearsal/both need the release mode and a Linux-free host)
#   --proctor=<executable> course: run this proctor instead of building one (skips the cargo build)
#   --release              export SEMIO_BUILD_MODE=release for the test-e2e step
#   --warm-proctor         course: first run `@teaching/proctor:build` (cached cargo closure) so the gate's own proctor build is incremental
#   --keep                 course: keep the gate's run directory after success
#   --serial               course: with --topology=both, run the topologies one after the other instead of at once
#   --check                run only the preflight (dependencies, disk, port, Playwright browsers) and exit
# Environment: E2E_SERVE_TIMEOUT (s, default 1800), E2E_MIN_FREE_GB (default 15), E2E_STAMP (log suffix).
set -u
set -o pipefail
unset PLAYWRIGHT_BROWSERS_PATH

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ticket="$(cd "$here/.." && pwd)"
root="$(cd "$ticket" && git rev-parse --show-toplevel)"
generated="$ticket/$(printf '\xf0\x9f\x97\x91\xef\xb8\x8fgenerated')"
[ -d "$generated" ] || mkdir -p "$generated"
stamp="${E2E_STAMP:-$(date +%Y%m%dT%H%M%S)}"

usage() { sed -n '2,/^set -u/{/^set -u/!p;}' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit "${1:-2}"; }
[ $# -ge 1 ] || usage
case "$1" in -h|--help) usage 0 ;; esac
product="$1"; shift

tool=()
warm=0 check=0 install_browsers=0 skip_activate=0 serve=default release=0 keep=0 serial=0 topology=dev proctor=""
extra=()
while [ $# -gt 0 ]; do
  case "$1" in
    --tool=nx|--tool=bazel) tool=("$1") ;;
    --tool=*) echo "unknown tool: $1" >&2; exit 2 ;;
    --install-browsers) install_browsers=1 ;;
    --skip-activate) skip_activate=1 ;;
    --no-serve) serve=0 ;;
    --serve) serve=1 ;;
    --topology=dev|--topology=rehearsal|--topology=both) topology="${1#--topology=}" ;;
    --proctor=*) proctor="${1#--proctor=}" ;;
    --release) release=1 ;;
    --keep) keep=1 ;;
    --serial) serial=1 ;;
    --check) check=1 ;;
    --warm-proctor) warm=1 ;;
    -h|--help) usage 0 ;;
    --) shift; extra=("$@"); break ;;
    *) echo "unknown option: $1" >&2; usage ;;
  esac
  shift
done

case "$product" in
  play)         pkg="@semio-tech/semio-tech-play"; port=6033; activate=1; dev_target=serve ;;
  demonstrator) pkg="@semio-tech/mit-bestand-demonstrator"; port=6029; activate=1; dev_target=serve ;;
  course)       pkg="@teaching/architecture-course"; port=6061; activate=0; dev_target=dev ;;
  *) echo "unknown product: $product (play|demonstrator|course)" >&2; exit 2 ;;
esac
[ "$serve" = default ] && { if [ "$product" = course ]; then serve=0; else serve=1; fi; }

log() { echo "[run-e2e $(date +%H:%M:%S)] $*"; }
repo() { (cd "$root" && bun repo ${tool[@]+"${tool[@]}"} "$@"); }

declare -a steps=()
serve_pid=""
serve_log="$generated/e2e-$product-$stamp-serve.log"

port_open() { (exec 3<>"/dev/tcp/127.0.0.1/$1") 2>/dev/null; }

stop_serve() {
  [ -n "$serve_pid" ] || return 0
  if kill -0 "$serve_pid" 2>/dev/null; then
    log "stopping serve (process group $serve_pid)"
    kill -TERM -- "-$serve_pid" 2>/dev/null || kill -TERM "$serve_pid" 2>/dev/null
    for _ in $(seq 1 60); do kill -0 "$serve_pid" 2>/dev/null || break; sleep 0.5; done
    if kill -0 "$serve_pid" 2>/dev/null; then
      log "serve ignored SIGTERM for 30 s; sending SIGKILL"
      kill -KILL -- "-$serve_pid" 2>/dev/null || kill -KILL "$serve_pid" 2>/dev/null
    fi
  fi
  wait "$serve_pid" 2>/dev/null
  serve_pid=""
  sleep 1
  if port_open "$port"; then log "WARNING: port $port still answers after stopping serve (orphan?); inspect with: lsof -nP -iTCP:$port -sTCP:LISTEN"; fi
}
cleanup() { local code=$?; trap - EXIT INT TERM; stop_serve; summary; exit "$code"; }
summary() {
  echo; log "summary for $product ($pkg) tool=${tool[*]:-default}"
  local entry; for entry in ${steps[@]+"${steps[@]}"}; do echo "  $entry"; done
  echo "  logs: $generated/e2e-$product-$stamp-*.log"
}
trap cleanup EXIT INT TERM

run_step() {
  local name="$1"; shift
  local out="$generated/e2e-$product-$stamp-$name.log" started=$SECONDS
  log "$name: $*  (log $out)"
  "$@" 2>&1 | tee "$out"
  local code=${PIPESTATUS[0]}
  steps+=("$(printf '%-18s exit %-3s %4ss' "$name" "$code" "$((SECONDS - started))")")
  return "$code"
}

fail() { log "FAILED: $*"; exit "${2:-1}"; }

preflight() {
  cd "$root" || fail "cannot enter $root"
  command -v bun >/dev/null || fail "bun not on PATH" 3
  [ -f node_modules/playwright/cli.js ] || fail "node_modules/playwright/cli.js missing: the dependency sync is not green yet (bun repo dependencies sync)" 3
  local free_gb; free_gb=$(df -Pk "$root" | awk 'NR==2{printf "%d", $4/1048576}')
  [ "$free_gb" -ge "${E2E_MIN_FREE_GB:-15}" ] || fail "only ${free_gb} GB free (need ${E2E_MIN_FREE_GB:-15}); free disk first" 3
  if [ "$serve" = 1 ] && port_open "$port"; then fail "port $port is already in use; stop the other serve first (lsof -nP -iTCP:$port -sTCP:LISTEN)" 3; fi
  check_browsers
}

browser_report() {
  bun -e '
    const fs = require("node:fs"), path = require("node:path"), { createRequire } = require("node:module");
    const cli = fs.realpathSync("node_modules/playwright/cli.js");
    const core = path.dirname(createRequire(cli).resolve("playwright-core/package.json"));
    const manifest = JSON.parse(fs.readFileSync(path.join(core, "browsers.json"), "utf8"));
    const cache = path.join(process.cwd(), ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
    const wanted = ["chromium", "chromium-headless-shell", "ffmpeg"];
    const missing = [];
    for (const name of wanted) {
      const revision = manifest.browsers.find((browser) => browser.name === name)?.revision;
      const directory = path.join(cache, `${name.replaceAll("-", "_")}-${revision}`);
      const ok = fs.existsSync(path.join(directory, "INSTALLATION_COMPLETE"));
      console.log(`${ok ? "ok     " : "MISSING"} ${directory}`);
      if (!ok) missing.push(name);
    }
    process.exit(missing.length ? 10 : 0);
  '
}

check_browsers() {
  local report code
  report=$(browser_report 2>&1); code=$?
  echo "$report" | sed 's/^/  /'
  [ "$code" = 0 ] && return 0
  if [ "$install_browsers" = 1 ]; then
    run_step browsers repo run workspace:deps-browsers || fail "workspace:deps-browsers failed"
    report=$(browser_report 2>&1); code=$?
    echo "$report" | sed 's/^/  /'
    [ "$code" = 0 ] || fail "Playwright browsers still missing after deps-browsers"
    return 0
  fi
  fail "Playwright browsers missing (download needs a go-ahead). Install with: bun repo ${tool[*]:+${tool[*]} }run workspace:deps-browsers   (or re-run with --install-browsers)" 4
}

start_serve() {
  [ "$serve" = 1 ] || return 0
  log "starting detached $dev_target on port $port (log $serve_log)"
  : > "$serve_log"
  (cd "$root" && exec perl -MPOSIX -e 'setsid(); exec @ARGV' -- bun repo ${tool[@]+"${tool[@]}"} run "$pkg:$dev_target" ) > "$serve_log" 2>&1 < /dev/null &
  serve_pid=$!
  local waited=0 limit="${E2E_SERVE_TIMEOUT:-1800}"
  while [ "$waited" -lt "$limit" ]; do
    if ! kill -0 "$serve_pid" 2>/dev/null; then
      tail -n 40 "$serve_log" | sed 's/^/  | /'
      steps+=("$(printf '%-18s exit %-3s %4ss' serve-ready "died" "$waited")")
      fail "serve exited before port $port answered" 5
    fi
    if port_open "$port" && curl -fsS -o /dev/null --max-time 15 "http://127.0.0.1:$port/" 2>/dev/null; then
      steps+=("$(printf '%-18s exit %-3s %4ss' serve-ready 0 "$waited")")
      log "serve answers on http://127.0.0.1:$port/ after ${waited}s"
      return 0
    fi
    sleep 3; waited=$((waited + 3))
  done
  tail -n 40 "$serve_log" | sed 's/^/  | /'
  steps+=("$(printf '%-18s exit %-3s %4ss' serve-ready timeout "$waited")")
  fail "port $port did not answer within ${limit}s" 6
}

main() {
  log "root $root; ticket $ticket"
  preflight
  [ "$check" = 1 ] && { log "preflight ok"; return 0; }
  export SEMIO_TICKET_DIR="$ticket"
  [ "$release" = 1 ] && export SEMIO_BUILD_MODE=release

  if [ "$activate" = 1 ] && [ "$skip_activate" = 0 ]; then
    run_step activate repo run "$pkg:activate-prototype" || fail "activate-prototype failed" 10
  fi

  if [ "$product" = course ] && [ "$warm" = 1 ] && [ -z "$proctor" ]; then
    run_step warm-proctor repo run @teaching/proctor:build || fail "@teaching/proctor:build failed" 10
  fi

  start_serve

  local args=(run "$pkg:test-e2e")
  if [ "$product" = course ]; then
    local gate=(--topology "$topology")
    [ "$keep" = 1 ] && gate+=(--keep)
    [ "$serial" = 1 ] && gate+=(--serial)
    [ -n "$proctor" ] && gate+=(--proctor "$proctor")
    args+=(-- "${gate[@]}" --)
    [ ${#extra[@]} -gt 0 ] && args+=("${extra[@]}")
  elif [ ${#extra[@]} -gt 0 ]; then
    log "note: play/demonstrator test-e2e accept no free arguments; ignoring: ${extra[*]}"
  fi
  run_step test-e2e repo "${args[@]}"
  local code=$?
  stop_serve
  return "$code"
}

main
