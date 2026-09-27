# 🟢️ W4: retry-until-green for the chain's compile steps (sourced by w4-chain.sh and w4-wasm-hold.sh after OUT, PHASE, log, step).
# Peers (a Codex fleet) edit guest-linked crates while the chain runs, so a compile step may meet a transient red. On failure:
# name every `could not compile` crate, block until ONE cargo check of those crates is green again (probe every ≤ 5 min, cap
# W4_GREEN_CAP s, default 45 min), then re-run the step (≤ W4_RETRIES retries, default 3). A failure that names no crate is retried
# once after 5 min when the caller allows it (`blind`), else it fails at once. Every attempt log is kept as <step>-a<n>.txt.
#   w4_retry <name> <wasm|native> <blind|strict> <resume-fn|-> -- <cmd…>
#     resume-fn (optional): a function that prints the replacement command for the retry, given the failed attempt's log.
W4_GREEN_CAP=${W4_GREEN_CAP:-2700}
W4_RETRIES=${W4_RETRIES:-3}
W4_PROBE_EVERY=${W4_PROBE_EVERY:-300}

w4_failed_crates() { /usr/bin/grep -oE 'could not compile `[^`]+`' "$1" | sed -E 's/could not compile `([^`]+)`/\1/' | sort -u; }

w4_wait_green() {
  local kind="$1" label="$2"; shift 2
  local crates=("$@") started=$(date +%s) probe=0 args=() crate
  for crate in $crates; do args+=(-p "$crate"); done
  [ "$kind" = wasm ] && args+=(--target wasm32-wasip2)
  while true; do
    probe=$(( probe + 1 ))
    local at=$(date +%s) capture="$OUT/$PHASE-$label-green-$probe.txt"
    cargo check --lib $args --message-format short > "$capture" 2>&1
    local rc=$?
    log "GREEN-PROBE $label #$probe $kind rc=$rc crates=${(j:,:)crates} wall=$(( $(date +%s) - at ))s"
    [ $rc = 0 ] && return 0
    /usr/bin/grep -m3 -E '^[^ ]+: error|^error' "$capture" | sed 's/^/  | /'
    [ $(( $(date +%s) - started )) -ge $W4_GREEN_CAP ] && { log "GREEN-TIMEOUT $label still red after $(( $(date +%s) - started ))s"; return 1; }
    local rest=$(( W4_PROBE_EVERY - ($(date +%s) - at) ))
    [ $rest -gt 0 ] && sleep $rest
  done
}

w4_retry() {
  local name="$1" kind="$2" mode="$3" resume="$4"; shift 4; [ "$1" = "--" ] && shift
  local cmd=("$@") attempt=0 rc crates blind_used=0
  while true; do
    step "$name" $cmd; rc=$?
    [ $rc = 0 ] && return 0
    attempt=$(( attempt + 1 ))
    mv "$OUT/$PHASE-$name.txt" "$OUT/$PHASE-$name-a$attempt.txt"
    crates=($(w4_failed_crates "$OUT/$PHASE-$name-a$attempt.txt"))
    typeset -f w4_on_failure >/dev/null && w4_on_failure "$name" "$OUT/$PHASE-$name-a$attempt.txt"
    [ $attempt -gt $W4_RETRIES ] && { log "FAILED $name after $W4_RETRIES retries rc=$rc"; return $rc; }
    if [ ${#crates} -gt 0 ]; then
      log "RETRY-WAIT $name attempt=$attempt rc=$rc red=${(j:,:)crates}"
      w4_wait_green "$kind" "$name-a$attempt" $crates || { log "FAILED $name: ${(j:,:)crates} never turned green"; return $rc; }
    elif [ "$mode" = blind ] && [ $blind_used = 0 ]; then
      blind_used=1
      log "RETRY-WAIT $name attempt=$attempt rc=$rc: no crate named, one blind retry after ${W4_PROBE_EVERY}s"
      sleep $W4_PROBE_EVERY
    else
      log "FAILED $name rc=$rc (no crate named)"
      return $rc
    fi
    [ "$resume" != - ] && cmd=($($resume "$OUT/$PHASE-$name-a$attempt.txt"))
    log "RETRY $name attempt=$(( attempt + 1 )) cmd=${(j: :)cmd}"
  done
}
