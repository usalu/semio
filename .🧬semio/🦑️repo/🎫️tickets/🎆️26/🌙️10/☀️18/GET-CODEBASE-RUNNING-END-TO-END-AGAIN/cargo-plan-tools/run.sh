#!/bin/zsh
# 🏃 Resumable cargo window runner (ticket GET-CODEBASE-RUNNING-END-TO-END-AGAIN). Plan: cargo-plan-tools/steps.tsv (from gen.ts). Report: result-cargo-plan.md.
# Start detached from the main session (children die with a subagent turn):  nohup <C>/cargo-plan-tools/run.sh >/dev/null 2>&1 & disown
# Knobs: ONLY=<regex on "id group"> FROM=<n> UNTIL=<n> TIER_B=1 (adds lock-extra + covered) RERUN_FAILED=1 DRY=1 WASM_BUILD=1 MIN_FREE_GB=40 RETRIES=6 GATE_MAX=14400 ACCEPT_NEW_PLAN=1
unsetopt BG_NICE MONITOR 2>/dev/null
cd /Users/ueli/Documents/semio || exit 66
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
TOOLS="$C/cargo-plan-tools"; STEPS="$TOOLS/steps.tsv"; MEMBERS="$TOOLS/lock-members.tsv"
W="$C/🗑️generated/cargo-window"
RETRIES=${RETRIES:-6}; GATE_MAX=${GATE_MAX:-14400}; MIN_FREE_GB=${MIN_FREE_GB:-40}
export SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":14400000}'
export SEMIO_BUILD_BUDGET_MS=${SEMIO_BUILD_BUDGET_MS:-3600000}

RETRY_RE='System image plist required|Bounded unique parent kernel roster required|Stale loaded repository|Original Cargo workspace topology changed|topology changed|Queued cache maintenance|SyntaxError: Export named|Cannot find module|Cannot find package|too many open files'
HARD_RE='Original compiler pressure owner required|Closed Source Rustup request required|Selected newline lsof fields required|Build scheduler compilation failed|No space left on device|ENOSPC|Could not resolve host|spurious network error|Unable to update registry|Original parent Script process invocation required|ENFILE'
LOCK_RE='cannot update the lock file|lock file needs to be updated|needs to be updated but --locked|--locked was passed'
COMPILE_RE='could not compile|error\[E[0-9]+\]|aborting due to|Source refused before compiler|failed to select a version|failed to get `|failed to load manifest|failed to parse manifest|no matching package|member of the wrong workspace|believes it.s in a workspace|cyclic package dependency|does not have these features|error: no library targets|error: no bin target|did not match any packages|Unbalanced Rust source delimiter|Missing module|failed to run custom build command|error: linking with'

ts() { date '+%m-%d %H:%M:%S'; }
say() { if [[ -n "$DRY" ]]; then print -r -- "$(ts) $*"; else print -r -- "$(ts) $*" | tee -a "$W/runner.log"; fi; }
stop() { local code=$1; shift; say "STOP rc=$code step=${CUR_ID:--} $*"; print -r -- "$(ts) rc=$code step=${CUR_ID:--} $*" > "$W/STOPPED"; exit $code; }
descendants() { local p; for p in $(pgrep -P $1 2>/dev/null); do print $p; descendants $p; done; }
kill_tree() { local pids=($1 $(descendants $1)); kill -TERM $pids 2>/dev/null; sleep 5; kill -KILL $pids 2>/dev/null; }
run_alarm() { local secs=$1 out=$2; shift 2; "$@" >"$out" 2>&1 & local pid=$! t=0
  while kill -0 $pid 2>/dev/null; do sleep 5; t=$((t+5)); if (( t >= secs )); then kill_tree $pid; wait $pid 2>/dev/null; return 124; fi; done
  wait $pid; return $?; }
busy() { ps -Ao pid=,command= | awk -v me=$$ '{ if ($1==me) next; c=$2; sub(/.*\//,"",c); if (c!="bun") next;
  if ($3=="repo" && ($4=="dependencies" || $4=="show" || $4=="cargo")) print;
  else if ($3 ~ /script\.ts$/ && $4=="repo" && ($5=="dependencies" || $5=="show" || $5=="cargo")) print }'; }
wait_gate() { local waited=0 b; while b=$(busy); [[ -n "$b" ]]; do
    (( waited % 300 == 0 )) && say "gate: waiting for foreign repo process: ${${(f)b}[1]:0:160}"
    sleep 10; waited=$((waited+10)); (( waited >= GATE_MAX )) && return 1; done; return 0; }
disk_ok() { local g; g=$(df -Pk . | awk 'NR==2{print int($4/1048576)}'); (( g >= MIN_FREE_GB )) || { say "disk: only ${g} GiB free (< $MIN_FREE_GB)"; return 1; }; return 0; }
classify() { local log=$1 rc=$2
  if (( rc == 0 )); then print ok
  elif command grep -aqE "$HARD_RE" "$log"; then print hard
  elif command grep -aqE "$RETRY_RE" "$log"; then print retry
  elif command grep -aqE "$LOCK_RE" "$log"; then print lock
  elif command grep -aqE "$COMPILE_RE" "$log"; then print compile
  else print unknown; fi; }

CLS=""; ATTEMPTS=0
cargo_call() { local alarm=$1 log=$2; shift 2; local alarms=0 rc; ATTEMPTS=0
  while true; do
    ATTEMPTS=$((ATTEMPTS+1))
    wait_gate || stop 75 "collision gate: a foreign bun repo dependencies/show/cargo process ran longer than ${GATE_MAX}s"
    disk_ok || stop 74 "disk guard"
    run_alarm $alarm "$log" bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- "$@"; rc=$?
    CLS=$(classify "$log" $rc)
    if (( rc == 124 )); then alarms=$((alarms+1)); say "alarm ${alarm}s expired (attempt $ATTEMPTS, alarm #$alarms)"; (( alarms >= 2 )) && stop 124 "alarm expired twice: $*"; continue; fi
    case $CLS in
      retry) (( ATTEMPTS >= RETRIES )) && stop 75 "infrastructure error persisted after $RETRIES attempts: $(command grep -aE "$RETRY_RE" "$log" | head -1 | cut -c1-200)"
             say "infra retry $ATTEMPTS/$RETRIES: $(command grep -aE "$RETRY_RE" "$log" | head -1 | cut -c1-160)"; sleep $((60*ATTEMPTS)); continue;;
      hard) stop 75 "infrastructure error: $(command grep -aE "$HARD_RE" "$log" | head -1 | cut -c1-200)";;
    esac
    return $rc
  done; }

# 🔒 Lock refresh: skip a workspace whose Cargo.lock is newer than every member manifest; else metadata --locked, update --workspace on a lock refusal.
do_lock() { local id=$1 alarm=$2 list=$3 log="$W/$id.log" fails=0 ws lock stale m
  : > "$log"
  for ws in ${=list}; do
    lock="${ws:h}/Cargo.lock"; stale=0
    if [[ ! -e "$lock" ]]; then stale=1; else
      for m in $(awk -F'\t' -v w="$ws" '$1==w{print $2}' "$MEMBERS"); do [[ -e "$m" && "$m" -nt "$lock" ]] && { stale=1; break; }; done; fi
    if (( ! stale )); then print -r -- "$ws fresh (Cargo.lock newer than all member manifests)" >> "$log"; continue; fi
    cargo_call 1500 "$W/$id.tmp" metadata --locked --format-version 1 --manifest-path "$ws"
    if [[ $CLS == ok ]]; then : > "$W/$id.tmp"; print -r -- "$ws ok (metadata --locked)" >> "$log"; continue; fi
    if [[ $CLS == lock ]] || ( [[ $CLS != compile ]] && command grep -aqiE 'lock' "$W/$id.tmp" ); then
      cargo_call 2400 "$W/$id.upd" update --workspace --manifest-path "$ws"
      if [[ $CLS == ok ]]; then
        cargo_call 1500 "$W/$id.tmp" metadata --locked --format-version 1 --manifest-path "$ws"
        if [[ $CLS == ok ]]; then print -r -- "$ws updated" >> "$log"; continue; fi
      fi
    fi
    fails=$((fails+1)); print -r -- "$ws FAILED $CLS: $(command grep -aE '^error|Caused|failed' "$W/$id.tmp" "$W/$id.upd" 2>/dev/null | head -3 | tr '\n' ' ' | cut -c1-360)" >> "$log"
  done
  rm -f "$W/$id.tmp" "$W/$id.upd"
  LOCK_FAILS=$fails; }

[[ -z "$DRY" ]] && { mkdir -p "$W"; rm -f "$W/STOPPED"; }
[[ -s "$STEPS" ]] || { print "missing $STEPS (run: bun $TOOLS/gen.ts)"; exit 66; }
sum=$(shasum "$STEPS" | cut -c1-16)
if [[ -f "$W/plan.sum" && "$(cat "$W/plan.sum")" != "$sum" && -z "$ACCEPT_NEW_PLAN" ]] && [[ -n "$(find "$W" -maxdepth 1 -name '*.done' -print -quit)" ]]; then
  print "plan changed since markers were written (ids may differ). Remove $W/*.done or set ACCEPT_NEW_PLAN=1."; exit 64; fi
[[ -z "$DRY" ]] && print -r -- "$sum" > "$W/plan.sum"

if [[ -z "$DRY" ]]; then
  command grep -q '^components = \["rust-src", "llvm-tools-preview"\]$' rust-toolchain.toml || stop 75 "rust-toolchain.toml components must be exactly rust-src + llvm-tools-preview (Closed Source Rustup request)"
  disk_ok || stop 74 "disk guard at start"
fi

total=0; ran=0; skipped=0; failed=0; UNK_STREAK=0; LOCK_FAILS=0; t0=$(date +%s)
say "window start plan=$sum tierB=${TIER_B:-0} only=${ONLY:--} from=${FROM:--} until=${UNTIL:--}"
while IFS=$'\t' read -u3 -r id group kind alarm manifest sub flags target pkgs est label; do
  [[ -z "$id" ]] && continue
  n=${id%%-*}; n=$((10#$n))
  [[ -z "$TIER_B" && ( $group == covered || $group == lock-extra ) ]] && continue
  [[ -n "$FROM" ]] && (( n < FROM )) && continue
  [[ -n "$UNTIL" ]] && (( n > UNTIL )) && continue
  [[ -n "$ONLY" ]] && ! print -r -- "$id $group" | command grep -aqE "$ONLY" && continue
  total=$((total+1))
  if [[ -f "$W/$id.done" ]]; then
    prc=$(sed -n 's/^rc=\([0-9]*\).*/\1/p' "$W/$id.done" | head -1)
    if [[ -z "$RERUN_FAILED" || "$prc" == 0 ]]; then skipped=$((skipped+1)); continue; fi
  fi
  CUR_ID=$id
  if [[ $kind == cargo ]]; then
    pk=(); for p in ${=pkgs}; do pk+=(-p $p); done
    tg=(); [[ $target != - ]] && tg=(--target $target)
    s=$sub; fl=$flags
    [[ -n "$WASM_BUILD" && $group == wasm ]] && s=build
    args=($s --manifest-path $manifest $pk $tg ${=fl})
  fi
  if [[ -n "$DRY" ]]; then
    if [[ $kind == cargo ]]; then print -r -- "[$id] alarm=${alarm}s est=${est}m  bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- $args"
    else print -r -- "[$id] alarm=${alarm}s est=${est}m  lock refresh: $pkgs"; fi
    continue
  fi
  say "step $id ($group) $label | est ${est}m | ${#${=pkgs}} item(s)"
  s0=$(date +%s); log="$W/$id.log"
  if [[ $kind == lock ]]; then
    do_lock $id $alarm "$pkgs"; rc=$LOCK_FAILS; cls=$([[ $rc == 0 ]] && print ok || print resolve); ATTEMPTS=1
  else
    cargo_call $alarm "$log" "${args[@]}"; rc=$?; cls=$CLS
    if [[ $cls == lock ]]; then
      say "  lock refusal: update --workspace --manifest-path $manifest, then retry once"
      cargo_call 2400 "$W/$id.upd" update --workspace --manifest-path "$manifest"
      if [[ $CLS == ok ]]; then cargo_call $alarm "$log" "${args[@]}"; rc=$?; cls=$CLS; fi
      rm -f "$W/$id.upd"
    fi
  fi
  s1=$(date +%s); secs=$((s1-s0))
  errs=0
  if [[ -f "$log" ]]; then
    command grep -aE '(^|[[:space:]])error(\[E[0-9]+\])?:' "$log" | command grep -av '^\[script\]' | cut -c1-330 > "$W/$id.errs"
    errs=$(wc -l < "$W/$id.errs" | tr -d ' ')
  fi
  print -r -- "rc=$rc cls=$cls attempts=$ATTEMPTS secs=$secs errors=$errs end=$(ts)" > "$W/$id.done"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$id" "$group" "$rc" "$cls" "$ATTEMPTS" "$secs" "$errs" "$pkgs" >> "$W/results.tsv"
  ran=$((ran+1))
  if [[ $cls == ok ]]; then UNK_STREAK=0; say "  ok  ${secs}s"
  else
    failed=$((failed+1)); say "  rc=$rc cls=$cls errors=$errs ${secs}s (continuing; log $log)"
    if [[ $cls == unknown ]]; then UNK_STREAK=$((UNK_STREAK+1)); (( UNK_STREAK >= 3 )) && stop 70 "three consecutive unclassified failures (launcher or environment problem?) last log $log"; else UNK_STREAK=0; fi
  fi
done 3< "$STEPS"
[[ -n "$DRY" ]] && exit 0
el=$(( $(date +%s) - t0 ))
say "window end: selected=$total ran=$ran skipped(done)=$skipped nonzero=$failed elapsed=${el}s"
[[ $failed == 0 ]] && touch "$W/ALL.done"
exit 0
