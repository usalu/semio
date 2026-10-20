#!/usr/bin/env bash
# Ticket input script: the measured Bazel demonstrator + report ladder.  Heavy; run ONLY after the coordinator's GO and one phase at a time (one heavy run machine-wide).
#   cold-run.sh preflight                 load, disk, foreign cargo/rustc/bazel/vite processes, wasm mutex
#   cold-run.sh cold                      bun repo --tool=bazel run demonstrator:activate-prototype     (needs DEMO_BZL_GO=1)
#   cold-run.sh warm                      same command again: the Bazel summary must show no executed action but the two probes
#   cold-run.sh probe comment|code        edit ONE Rust component (energy hub component by default), re-run, revert, re-run
#   cold-run.sh probe brand               edit demonstrator 🪧️brand.ts (TS only): no Rust or wasm action may run
#   cold-run.sh e2e                       run-e2e.sh demonstrator --tool=bazel --skip-activate (test-e2e; serve teardown check)
#   cold-run.sh report cold|warm|probe    @semio-tech/mit-bestand-bericht:build-kompaktbericht, then build; probe edits the kompaktbericht .tex
#   cold-run.sh deck                      @semio-tech/mit-bestand-praesentation-projektetage:build twice (second must be a hit)
# Environment: DEMO_PROBE_FILE (Rust), DEMO_BZL_STAMP (log suffix).
set -u
set -o pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ticket="$(cd "$here/.." && pwd)"
root="$(cd "$ticket" && git rev-parse --show-toplevel)"
e() { printf "$@"; }
logs="$ticket/$(e '\xf0\x9f\x97\x91\xef\xb8\x8fgenerated')/demo-bzl-run"; mkdir -p "$logs"
stamp="${DEMO_BZL_STAMP:-$(date +%Y%m%dT%H%M%S)}"
demo="@semio-tech/mit-bestand-demonstrator"; bericht="@semio-tech/mit-bestand-bericht"; deck="@semio-tech/mit-bestand-praesentation-projektetage"
rust_default="$root/$(e '\xf0\x9f\x8c\x8e\xef\xb8\x8fhub')/$(e '\xf0\x9f\xa7\xa9\xef\xb8\x8fcompositions')/$(e '\xf0\x9f\x94\x8b\xef\xb8\x8fenergy')/$(e '\xf0\x9f\xa7\xa9\xef\xb8\x8fcomponent')/$(e '\xf0\x9f\xa6\x80\xef\xb8\x8f.rs')"
rust="${DEMO_PROBE_FILE:-$rust_default}"
brand="$root/$(e '\xe2\x99\xbb\xef\xb8\x8fmit-bestand')/$(e '\xf0\x9f\xa7\xba\xef\xb8\x8fdemonstrator')/$(e '\xf0\x9f\xaa\xa7\xef\xb8\x8fbrand.ts')"
tex="$root/$(e '\xe2\x99\xbb\xef\xb8\x8fmit-bestand')/$(e '\xf0\x9f\x93\x8b\xef\xb8\x8fbericht')/$(e '\xf0\x9f\x93\x9d\xef\xb8\x8fkompaktbericht')/$(e '\xf0\x9f\x93\x9d\xef\xb8\x8fkompaktbericht').tex"
log() { echo "[demo-bzl $(date +%H:%M:%S)] $*"; }
repo() { (cd "$root" && bun repo --tool=bazel "$@"); }
summary_of() { command grep -E "INFO: [0-9]+ process|Build completed|Elapsed time|executed|cache hit|\[bazel\]" "$1" | tail -10; }

timed() {
  local name="$1"; shift
  local out="$logs/$name-$stamp.log" t0=$SECONDS
  log "$name: $*  (log $out)"
  "$@" > "$out" 2>&1; local code=$?
  log "$name: exit $code in $((SECONDS - t0))s"; summary_of "$out"
  return "$code"
}

with_edit() {
  local file="$1" text="$2" name="$3"; shift 3
  [ -f "$file" ] || { log "probe file missing: $file"; return 2; }
  local backup="$logs/probe-backup-$name-$stamp"; cp -p "$file" "$backup"
  trap 'cp -p "$backup" "$file"; log "probe reverted"' EXIT
  printf '%s\n' "$text" >> "$file"
  timed "probe-$name" "$@"; local code=$?
  cp -p "$backup" "$file"; log "reverted; the rerun expects disk-cache hits only"
  timed "probe-$name-revert" "$@" || code=$?
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
  local foreign; foreign="$(pgrep -fl 'cargo|rustc|wasm-opt|jco|vite|bazel|tectonic' | command grep -v "pgrep\|cold-run" || true)"
  [ -z "$foreign" ] || { log "FAIL foreign heavy processes:"; echo "$foreign"; return 1; }
  [ ! -e /tmp/semio-wasm-build.lock ] || log "WARNING: fleet wasm mutex lock exists"
  repo --help > /dev/null 2>&1 || { log "FAIL bun repo --help"; return 1; }
  log "preflight ok"
}

case "${1:-}" in
  preflight) preflight ;;
  cold)
    [ "${DEMO_BZL_GO:-0}" = 1 ] || { log "refused: the cold activation needs the coordinator's go (DEMO_BZL_GO=1)"; exit 2; }
    preflight && timed cold repo run "$demo:activate-prototype" ;;
  warm) timed warm repo run "$demo:activate-prototype" ;;
  probe)
    case "${2:-comment}" in
      comment) with_edit "$rust" "// [DEBUG] demo-bzl probe $stamp" comment repo run "$demo:activate-prototype" ;;
      code) with_edit "$rust" "#[allow(dead_code)] const DEMO_BZL_PROBE: &str = \"$stamp\";" code repo run "$demo:activate-prototype" ;;
      brand) with_edit "$brand" "// [DEBUG] demo-bzl probe $stamp" brand repo run "$demo:activate-prototype" ;;
      *) log "kind comment|code|brand"; exit 2 ;;
    esac ;;
  e2e) exec "$ticket/e2e-tools/run-e2e.sh" demonstrator --tool=bazel --skip-activate ;;
  report)
    case "${2:-warm}" in
      cold) [ "${DEMO_BZL_GO:-0}" = 1 ] || { log "refused: needs DEMO_BZL_GO=1"; exit 2; }
            preflight && timed report-compact-cold repo run "$bericht:build-kompaktbericht" && timed report-all-cold repo run "$bericht:build" ;;
      warm) timed report-compact-warm repo run "$bericht:build-kompaktbericht" && timed report-all-warm repo run "$bericht:build" ;;
      probe) with_edit "$tex" "% [DEBUG] demo-bzl probe $stamp" report repo run "$bericht:build-kompaktbericht" ;;
      *) log "kind cold|warm|probe"; exit 2 ;;
    esac ;;
  deck) timed deck-first repo run "$deck:build" && timed deck-second repo run "$deck:build" ;;
  *) sed -n 2,13p "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 2 ;;
esac
