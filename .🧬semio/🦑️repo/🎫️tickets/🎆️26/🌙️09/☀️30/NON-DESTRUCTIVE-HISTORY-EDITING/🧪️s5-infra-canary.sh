#!/bin/zsh
# 🐤️ Activation canary (design §22.12): one foreground pass over the puzzle activation gate of the LIVE tree.
#   lock-root | lock-s | lock-hub | lock-teaching   `cargo metadata --locked --offline` per workspace (never `--no-deps`: it is blind)
#   registry                                        generator dry run: the launch/catalog render succeeds and drops no hand-added launch row
#   puzzle                                          `cargo check -p semio-hub-puzzle --target wasm32-wasip2 --lib` (hub workspace)
#   wgpu                                            `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown`
# Usage: `zsh 🧪️s5-infra-canary.sh [--stamp <stamp>] [--gate <n>] [step ...]` (default: every step, in the order above; the two
# compile steps wait for fewer than `n` cargos, default 4 — the coordinator grants a higher bound for an activation pass;
# `--frozen` runs although `COORDINATOR-ACTIVATION` holds the landing lock: the coordinator froze the tree for this very pass
# and starts the activation only on its verdict).
# The verdict names who holds the tree locks (rule 58: landing, stdio, puzzle, hub) and who released one since the last pass.
# The four lock steps are ungated (`cargo metadata` only resolves; coordinator 05:1x) and are skipped while no `Cargo.toml` / `Cargo.lock` is newer than the last green lock pass;
# a gate that stays busy for 4 minutes ends the call with exit 5 (re-issue with the same stamp).
# Every step writes `🗑️generated/s5-infra/canary-<stamp>-<step>.txt` and `.exit`; a re-issue with the same stamp skips finished
# steps (a Bash call is capped at 10 min), and one verdict line is appended to `🗑️generated/s5-infra/canary.events` once every
# step of the pass has an exit code (GREEN, AMBER = the next render would drop hand-made launch rows, RED), with free disk and
# swap. The pass obeys build gate v5 (rule 59: fewer than 4 cargo, `CARGO_BUILD_JOBS=3`; rule 55: one extra cycle behind a landing
# holder) and refuses to run while `COORDINATOR-ACTIVATION` holds the landing lock (exit 4, a SKIPPED line): it would queue on
# the cargo build lease behind the activation.
root="/Users/ueli/Documents/semio"
ticket="${0:A:h}"
out="$ticket/🗑️generated/s5-infra"
locks="$ticket/🗑️generated/coord/locks"
registry="$root/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"
events="$out/canary.events"
all=(lock-root lock-s lock-hub lock-teaching registry puzzle wgpu)
stamp="$(date '+%m%d-%H%M')"
steps=()
limit=4
frozen=0
while [ $# -gt 0 ]; do
  case "$1" in
    --stamp) stamp="$2"; shift 2 ;;
    --gate) limit="$2"; shift 2 ;;
    --frozen) frozen=1; shift ;;
    *) steps+=("$1"); shift ;;
  esac
done
[ ${#steps} -eq 0 ] && steps=($all)
mkdir -p "$out"
cd "$root" || exit 2

holder() { cat "$locks/landing/owner" 2>/dev/null || echo free }
export CARGO_BUILD_JOBS=3
quiet() { [ "$(pgrep -x cargo | wc -l | tr -d ' ')" -lt $limit ] }
waited=0
busy() { echo "[canary $stamp] GATE BUSY $(date '+%T') cargo=$(pgrep -x cargo | wc -l | tr -d ' ') — re-issue with --stamp $stamp" >&2; exit 5 }
settle() { until quiet; do [ $waited -ge 240 ] && busy; sleep 20; waited=$((waited + 20)); done }
gate() { settle; [ "$(holder)" = free ] || { sleep 20; settle } }
marker="$out/canary.locks-green"
manifests() { { git ls-files -z -- '*Cargo.toml' '*Cargo.lock'; git ls-files -z -o --exclude-standard -- '*Cargo.toml' '*Cargo.lock' } | tr '\0' '\n' }
stale_manifests() { [ -f "$marker" ] || { echo "no green lock pass yet"; return }; manifests | while read -r file; do [ "$file" -nt "$marker" ] && echo "$file"; done }

refuse_activation() {
  [ "$(holder)" = COORDINATOR-ACTIVATION ] && [ $frozen -eq 0 ] || return 0
  echo "$(date '+%F %T') canary $stamp SKIPPED landing=COORDINATOR-ACTIVATION" | tee -a "$events"
  exit 4
}
metadata() {
  [ -n "$changed" ] || { echo "manifests and lockfiles unchanged since the green lock pass of $(date -r "$marker" '+%T')"; return 0 }
  cargo metadata --locked --offline --format-version 1 --manifest-path "$1" 2>&1 >/dev/null
}
registry_dry() {
  if /usr/bin/grep -q '"reconcile-launch-seed"' "$registry/📜️script.ts"; then
    (cd "$registry" && bun ./📜️script.ts reconcile-launch-seed --check) 2>&1
    return $?
  fi
  bun "$out/stage-reconcile/run.ts" --check 2>&1
}
run() {
  case "$1" in
    lock-root) metadata Cargo.toml ;;
    lock-s) metadata "✏️s/Cargo.toml" ;;
    lock-hub) metadata "🌎️hub/Cargo.toml" ;;
    lock-teaching) metadata "🎓️teaching/Cargo.toml" ;;
    registry) registry_dry ;;
    puzzle) gate; cargo check --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-puzzle --target wasm32-wasip2 --lib --message-format=short 2>&1 ;;
    wgpu) gate; cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format=short 2>&1 ;;
    *) echo "unknown step: $1"; return 2 ;;
  esac
}

changed="$(stale_manifests | head -40)"
started="$out/canary-$stamp.started"
[ -f "$started" ] || touch "$started"
before="$(/usr/bin/grep -o ' events=[0-9]*' "$events" 2>/dev/null | tail -1 | tr -dc '0-9')"
for step in $steps; do
  file="$out/canary-$stamp-$step.txt"
  [ -f "${file%.txt}.exit" ] && continue
  refuse_activation
  echo "[canary $stamp] $step start $(date '+%T') landing=$(holder)"
  run "$step" > "$file"
  code=$?
  echo "landing-at-end=$(holder)" >> "$file"
  echo "$code" > "${file%.txt}.exit"
  echo "[canary $stamp] $step exit=$code $(date '+%T')"
done

green=1
for step in lock-root lock-s lock-hub lock-teaching; do
  [ "$(cat "$out/canary-$stamp-$step.exit" 2>/dev/null)" = 0 ] && ! /usr/bin/grep -q '^manifests and lockfiles unchanged' "$out/canary-$stamp-$step.txt" || green=0
done
verdict=GREEN
line=""
detail=""
for step in $all; do
  file="$out/canary-$stamp-$step.txt"
  [ -f "${file%.txt}.exit" ] || { echo "[canary $stamp] pass incomplete: $step has not run"; exit 0 }
  code="$(cat "${file%.txt}.exit")"
  line="$line $step=$code"
  [ "$code" = 0 ] && continue
  [ "$step" = registry ] && [ "$code" = 3 ] && { [ "$verdict" = GREEN ] && verdict=AMBER; continue }
  verdict=RED
  [ -z "$detail" ] && detail="$(/usr/bin/grep -m1 -E '(^|: )error(\[|:)|^error|Error:' "$file" | cut -c1-260)"
done
lost="$(sed -n 's/.*would move \([0-9]*\) configuration row(s) and \([0-9]*\) input(s).*/\1+\2/p' "$out/canary-$stamp-registry.txt" | head -1)"
edited="$(sed -n 's/.*; \([0-9]*\) edited seed row(s).*/\1/p' "$out/canary-$stamp-registry.txt" | head -1)"
[ -n "$lost$edited" ] && [ "$lost+${edited:-0}" != "0+0+0" ] && [ "$verdict" = GREEN ] && verdict=AMBER
disk="$(df -g /System/Volumes/Data | awk 'END { print $4 }')"
swap="$(sysctl -n vm.swapusage | sed -n 's/.*used = \([0-9.]*M\).*/\1/p')"
total="$(wc -l < "$locks/events.txt" 2>/dev/null | tr -d ' ')"
landed="$(tail -n "+$((${before:-0} + 1))" "$locks/events.txt" 2>/dev/null | awk '$3 == "release" && $4 != "serve" { printf "%s%s/%s", sep, $5, $4; sep = "," }')"
last="$(awk '$3 == "release" && $4 != "serve" { wp = $5 "/" $4 } END { print wp }' "$locks/events.txt" 2>/dev/null)"
held="$(for lock in landing stdio puzzle hub; do [ -d "$locks/$lock" ] && printf '%s=%s ' "$lock" "$(cat "$locks/$lock/owner" 2>/dev/null)"; done)"
echo "$(date '+%F %T') canary $stamp $verdict$line lost-rows=${lost:-?} edited-rows=${edited:-?} disk-free=${disk}GiB swap-used=$swap held=[${held% }] last-landed=${last:--} landed-since=[${landed}] events=${total:-0}${detail:+ first-error=\"$detail\"}" | tee -a "$events"
[ $green -eq 1 ] && touch -r "$started" "$marker"
rm -f "$started"
[ "$verdict" = RED ] && exit 1
exit 0
