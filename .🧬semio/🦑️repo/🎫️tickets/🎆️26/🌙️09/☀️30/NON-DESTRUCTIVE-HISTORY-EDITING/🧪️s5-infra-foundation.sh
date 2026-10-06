#!/bin/zsh
# 🧱️ Foundation status (rules 56, 63): one foreground pass that keeps ONE line current in `🗑️generated/coord/foundation.status`:
# `GREEN <HH:MM:SS> …`, `RED <HH:MM:SS> <crate> <file:line> …` or, while a cold closure is being warmed, `BUILDING <HH:MM:SS> <what>`
# (= no cargo by anyone else). Every verdict ends with `disk-free=<GiB> swap-used=<M>`.
#   (no argument)  native `cargo check --lib` of pack + replication + kernel + pixels (the crates peers outside the fleet
#                  refactor; all four sit in the activation closure), then the kernel for `wasm32-wasip2`
#                  (a red there alone is marked `wasm32-wasip2-only`)
#   --cold         the same, announced as BUILDING first (after a build-unit prune everything recompiles)
#   --warm         the same, then `semio-framework-plugin --lib` and its `--lib --tests --features artifact-app-testing` check; the
#                  status stays BUILDING with the running step and flips once at the end. The colour is the foundation's; the plugin
#                  verdicts follow as `plugin-lib=…` / `plugin-tests=…` (`RED:<file:line>(<n>)` names the first of n errors)
# Build gate v5 for this pass (rule 59: fewer than 5 cargo, `CARGO_BUILD_JOBS=3`) before each cargo; a gate that stays busy for
# 4 minutes ends the pass with exit 5. A cargo that ends without a compile error (killed, deadlock breaker) is no verdict: exit 6.
# Neither changes a GREEN/RED line. Step output `🗑️generated/s5-infra/foundation-<step>.txt` (overwritten per pass), history
# `🗑️generated/s5-infra/foundation.events`. The last output line says `TRANSITION` when the colour or the red location changed —
# the one case `main` gets a message. A pass can outlast one Bash call (10 minutes): let it finish or re-issue it.
root="/Users/ueli/Documents/semio"
ticket="${0:A:h}"
out="$ticket/🗑️generated/s5-infra"
state="$ticket/🗑️generated/coord/foundation.status"
locks="$ticket/🗑️generated/coord/locks"
mode="$1"
mkdir -p "$out"
cd "$root" || exit 2
export CARGO_BUILD_JOBS=3
holder() { cat "$locks/landing/owner" 2>/dev/null || echo free }
quiet() { [ "$(pgrep -x cargo | wc -l | tr -d ' ')" -lt 5 ] }
waited=0
busy() { echo "GATE BUSY $(date '+%T') cargo=$(pgrep -x cargo | wc -l | tr -d ' ') rustc=$(pgrep -x rustc | wc -l | tr -d ' ') — no pass, status unchanged: $(cat "$state" 2>/dev/null)"; exit 5 }
gate() { until quiet; do [ $waited -ge 240 ] && busy; sleep 20; waited=$((waited + 20)); done }
verdictless() { echo "NO VERDICT $(date '+%T') cargo exit=$code without a compile error (killed or lock-starved) — status unchanged: $(cat "$state" 2>/dev/null)"; exit 6 }
building() { [ -n "$mode" ] && print -r -- "BUILDING $(date '+%T') $1 — S5-INFRA alone (rule 63: no cargo by anyone else) (last verdict: $last)" > "$state" }
# 🏃️ check <step> <cargo check arguments…>: one gated cargo, output in `foundation-<step>.txt`, exit code in `code`.
check() {
  file="$out/foundation-$1.txt"
  shift
  gate
  cargo check "$@" --message-format=short > "$file" 2>&1
  code=$?
  [ $code -ne 0 ] && ! /usr/bin/grep -q -E '^error|: error(\[|:)' "$file" && verdictless
  return $code
}
# 📍️ The first compile error of the last step as `<file:line>` (repo-relative), or the first `error` line.
place() {
  local first="$(/usr/bin/grep -m1 -E '^[^ ].*:[0-9]+:[0-9]+: error' "$file")" at
  [ -n "$first" ] || { /usr/bin/grep -m1 -E '^error' "$file" | cut -c1-160; return }
  at="${${first%%: error*}%:*}"
  echo "${${${at%:*}:a}#$root/}:${at##*:}"
}
errors() { /usr/bin/grep -c -E ': error(\[|:)' "$file" }

previous="$(cat "$state" 2>/dev/null)"
last="${previous%% disk-free*}"
[ "${previous%% *}" = BUILDING ] && last="${${previous#*last verdict: }%\)}"
steps=2
[ "$mode" = --warm ] && steps=4
extra=""
building "warm-up 1/$steps pack + replication + kernel + pixels native lib"
if ! check native -p semio-framework-pack -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-pixels --lib; then
  line="RED $(date '+%T') $(sed -n 's/^error: could not compile `\([^`]*\)`.*/\1/p' "$file" | head -1) $(place)"
else
  building "warm-up 2/$steps kernel wasm32-wasip2 lib"
  if ! check wasip2 -p semio-framework-os-kernel --lib --target wasm32-wasip2; then
    line="RED $(date '+%T') $(sed -n 's/^error: could not compile `\([^`]*\)`.*/\1/p' "$file" | head -1) $(place) wasm32-wasip2-only"
  else
    if [ "$mode" = --warm ]; then
      building "warm-up 3/4 plugin lib (foundation is green)"
      if check plugin-lib -p semio-framework-plugin --lib; then
        extra=" plugin-lib=GREEN"
        building "warm-up 4/4 plugin lib + tests check, artifact-app-testing (foundation and plugin lib are green)"
        check plugin-tests -p semio-framework-plugin --lib --tests --features artifact-app-testing && extra="$extra plugin-tests=GREEN" || extra="$extra plugin-tests=RED:$(place)($(errors))"
      else
        extra=" plugin-lib=RED:$(place)($(errors))"
      fi
    fi
    line="GREEN $(date '+%T')$extra"
  fi
fi
line="$line disk-free=$(df -g /System/Volumes/Data | awk 'END { print $4 }')GiB swap-used=$(sysctl -n vm.swapusage | sed -n 's/.*used = \([0-9]*\).*/\1/p')M"
print -r -- "$line" > "$state.tmp" && mv "$state.tmp" "$state"
echo "$(date '+%F') $line landing=$(holder)" >> "$out/foundation.events"
was=(${=last})
now=(${=line})
echo "$line"
key() { [ "$1" = RED ] && echo "RED $3 $4" || echo "$1" }
[ "$(key $was)" = "$(key $now)" ] && echo "same as before" || echo "TRANSITION from: $last"
