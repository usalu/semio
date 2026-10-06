#!/bin/zsh
# 🛬️ S5-AGNOSTIC P1 (fleet rule 51, LANDING LOCK): lands the two staged harness changes — `🧪️s4-agnostic-harness-leafless.py` and
# `🧪️s4-agnostic-child-reload-law.py` — in ONE lock hold with their verifying gated check
# `cargo check -p semio-framework-plugin --lib --features artifact-app-testing --message-format=short` (the harness module compiles only
# under `test` or `artifact-app-testing`, so the activation closure never sees it). Green: releases `landing`. Red: KEEPS the lock and
# prints the errors (fix within the 20-min hold, then `--recheck`; or `--revert` restores the pre-acquire copy and releases).
# Single-flight and re-issuable like the acceptance runner: a call that finds the landing running waits for it; `LAND_WAIT=<seconds>` bounds
# the wait for the lock (default 480; re-issue on exit 3 within 90 s to keep the FIFO place, rule 60). One cargo per agent: an acceptance
# build of mine still running when the lock arrives is stopped at once (`run/landing-hold` keeps the batch from starting another). Usage: [--recheck|--revert]
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
G="$T/🗑️generated/s5-agnostic"
H="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
WP=S5-AGNOSTIC
STAGED=("🧪️s4-agnostic-harness-leafless.py" "🧪️s4-agnostic-child-reload-law.py")
mkdir -p "$G/run"
pidf="$G/run/land.pid"; log="$G/land-check.txt"
alive() { [ -f "$pidf" ] && kill -0 "$(cat "$pidf")" 2>/dev/null }
if alive; then
  waited=0
  while alive && [ "$waited" -lt 540 ]; do sleep 10; waited=$((waited + 10)); done
  if alive; then echo "LANDING STILL RUNNING: $(tail -1 "$log" 2>/dev/null | cut -c1-240)"; exit 3; fi
  echo "landing finished:"; tail -5 "$log"; zsh "$T/🔐️lock.sh" status; exit 0
fi
foundation_red() {
  local line="$(cat "$T/🗑️generated/coord/foundation.status" 2>/dev/null)"
  [[ "$line" == BUILDING* ]] && return 0
  [[ "$line" == RED* ]] || return 1
  local since="$(date -j -f '%T' "${${=line}[2]}" +%s 2>/dev/null)"
  [ -n "$since" ] || return 1
  local age=$(( $(date +%s) - since ))
  [ "$age" -ge 0 ] && [ "$age" -lt 360 ]
}
echo $$ > "$pidf"
if [ "$1" != "--revert" ] && [ -e "$T/🗑️generated/coord/activation.flag" ]; then echo "ACTIVATION WINDOW (rule 68): activation.flag exists — no save under the framework tree"; rm -f "$pidf"; exit 6; fi
if [ "$1" != "--revert" ] && foundation_red; then echo "FOUNDATION RED (rule 56): $(cat "$T/🗑️generated/coord/foundation.status") — not acquiring"; rm -f "$pidf"; exit 6; fi
if [ "$1" = "--revert" ]; then
  [ -f "$G/harness-before-landing.rs" ] && cp "$G/harness-before-landing.rs" "$H" && echo "harness restored to its pre-acquire state"
  zsh "$T/🔐️lock.sh" release landing "$WP"; rm -f "$pidf" "$G/run/landing-hold"; exit 0
fi
zsh "$T/🔐️lock.sh" acquire landing "$WP" "${LAND_WAIT:-480}" || { rm -f "$pidf"; exit 3; }
touch "$G/run/landing-hold"
mine() { pgrep -f "history_edit_inputs_resolve documents_reload_identically" > /dev/null }
if mine; then
  echo "stopping my own acceptance build (one cargo per agent; its finished units stay)"
  pkill -f "history_edit_inputs_resolve documents_reload_identically"
  waited=0
  while mine && [ "$waited" -lt 60 ]; do sleep 3; waited=$((waited + 3)); done
fi
if [ "$1" != "--recheck" ]; then
  cp "$H" "$G/harness-before-landing.rs"
  for staged in "${STAGED[@]}"; do
    python3 "$T/$staged" --apply || {
      echo "$staged did not apply — harness restored, lock released"; cp "$G/harness-before-landing.rs" "$H"; zsh "$T/🔐️lock.sh" release landing "$WP"; rm -f "$pidf" "$G/run/landing-hold"; exit 5; }
  done
fi
until [ "$(pgrep -x cargo | wc -l | tr -d ' ')" -lt 4 ]; do sleep 20; done
export CARGO_BUILD_JOBS=3
echo "check start $(date '+%F %T')" > "$log"
cargo check -p semio-framework-plugin --lib --features artifact-app-testing --message-format=short >> "$log" 2>&1
code=$?
echo "exit=$code $(date '+%F %T')" >> "$log"
if [ "$code" = 0 ]; then
  zsh "$T/🔐️lock.sh" release landing "$WP"
  rm -f "$G/run/landing-hold"
  echo "LANDED GREEN"; tail -3 "$log"
else
  echo "CHECK RED (lock kept, hold ≤ 20 min):"; /usr/bin/grep -E "error|could not compile" "$log" | head -30
fi
rm -f "$pidf"
exit $code
