#!/usr/bin/env bash
# Ticket input script: the measured Bazel play activation ladder.  Heavy; run ONLY after the coordinator's GO and one phase at a time (one heavy run machine-wide).
#   cold-run.sh preflight            load, disk, foreign cargo/rustc/bazel/vite processes, wasm mutex, bazel provisioned
#   cold-run.sh cold                 bun repo --tool=bazel run play:activate-prototype            (needs PLAY_BZL_GO=1)
#   cold-run.sh warm                 same command again: expect 0 executed actions
#   cold-run.sh probe comment|code   edit ONE lane's component source, re-run, then revert and re-run (expects only that lane's chain, then disk hits)
#   cold-run.sh e2e                  run-e2e.sh play --tool=bazel --skip-activate
# Environment: PLAY_PROBE_FILE (default: the lowpoly hub component crate), PLAY_BZL_STAMP (log suffix).
set -u
set -o pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ticket="$(cd "$here/.." && pwd)"
root="$(cd "$ticket" && git rev-parse --show-toplevel)"
logs="$ticket/$(printf '\xf0\x9f\x97\x91\xef\xb8\x8fgenerated')/play-bzl-run"; mkdir -p "$logs"
stamp="${PLAY_BZL_STAMP:-$(date +%Y%m%dT%H%M%S)}"
target="@semio-tech/semio-tech-play:activate-prototype"
probe_default="$root/$(printf '\xf0\x9f\x8c\x8e\xef\xb8\x8fhub')/$(printf '\xf0\x9f\xa7\xa9\xef\xb8\x8fcompositions')/$(printf '\xf0\x9f\x92\xa0\xef\xb8\x8flowpoly')/$(printf '\xf0\x9f\xa7\xa9\xef\xb8\x8fcomponent')/$(printf '\xf0\x9f\xa6\x80\xef\xb8\x8f.rs')"
probe="${PLAY_PROBE_FILE:-$probe_default}"
log() { echo "[play-bzl $(date +%H:%M:%S)] $*"; }
repo() { (cd "$root" && bun repo --tool=bazel "$@"); }

summary_of() { command grep -E "INFO: [0-9]+ process|Build completed|Elapsed time|executed|cache hit" "$1" | tail -8; }

timed() {
  local name="$1"; shift
  local out="$logs/$name-$stamp.log" t0=$SECONDS
  log "$name: $*  (log $out)"
  "$@" > "$out" 2>&1; local code=$?
  log "$name: exit $code in $((SECONDS - t0))s"; summary_of "$out"
  return "$code"
}

preflight() {
  cd "$root" || exit 3
  local load free_gb
  load="$(sysctl -n vm.loadavg | tr -d '{}' | awk '{print $1}')"
  free_gb="$(df -g "$root" | awk 'NR==2{print $4}')"
  log "load $load  free ${free_gb} GB  (need load < 8 and >= 60 GB free)"
  [ "${free_gb:-0}" -ge 60 ] || { log "FAIL disk"; return 1; }
  awk "BEGIN{exit !($load < 8)}" || { log "FAIL load"; return 1; }
  local foreign; foreign="$(pgrep -fl 'cargo|rustc|wasm-opt|jco|vite|bazel' | command grep -v "pgrep\|cold-run" || true)"
  [ -z "$foreign" ] || { log "FAIL foreign heavy processes:"; echo "$foreign"; return 1; }
  [ ! -e /tmp/semio-wasm-build.lock ] || log "WARNING: fleet wasm mutex lock exists: $(ls /tmp/semio-wasm-build.lock 2>/dev/null)"
  repo --help > /dev/null 2>&1 || { log "FAIL bun repo --help"; return 1; }
  log "preflight ok"
}

case "${1:-}" in
  preflight) preflight ;;
  cold)
    [ "${PLAY_BZL_GO:-0}" = 1 ] || { log "refused: cold activation needs the coordinator's go (PLAY_BZL_GO=1)"; exit 2; }
    preflight && timed cold repo run "$target" ;;
  warm) timed warm repo run "$target" ;;
  probe)
    kind="${2:-comment}"; [ -f "$probe" ] || { log "probe file missing: $probe"; exit 2; }
    backup="$logs/probe-backup-$stamp"; cp -p "$probe" "$backup"
    trap 'cp -p "$backup" "$probe"; log "probe reverted"' EXIT
    case "$kind" in
      comment) printf '\n// [DEBUG] play-bzl probe %s\n' "$stamp" >> "$probe" ;;
      code) printf '\n#[allow(dead_code)]\nconst PLAY_BZL_PROBE: &str = "%s";\n' "$stamp" >> "$probe" ;;
      *) log "kind comment|code"; exit 2 ;;
    esac
    timed probe-$kind repo run "$target"; code=$?
    cp -p "$backup" "$probe"; log "reverted; rerun expects disk-cache hits only"
    timed probe-revert repo run "$target" || code=$?
    exit "$code" ;;
  e2e) exec "$ticket/e2e-tools/run-e2e.sh" play --tool=bazel --skip-activate ;;
  *) sed -n 2,8p "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 2 ;;
esac
