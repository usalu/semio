#!/bin/bash
# 🧪 [audit12] runs the G46 batch laws listed in laws.tsv on <root> (read-only bun tests with the pinned bun, one suite at a time, from the root): bash laws.sh <root> [batch-regex]  → one summary row per line in 🗑️generated/audit12/laws.txt, logs in 🗑️generated/audit12/laws/
ROOT=$(cd "${1:?root}" && pwd -P); HERE=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$HERE"); G="$C/🗑️generated/audit12"; only="${2:-.}"
BUN=${BUN:-$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
export PATH="$(dirname "$BUN"):$PATH"; mkdir -p "$G/laws" "$G/a"
n=0
while IFS="$(printf '\t')" read -r batch mode want args; do
  n=$((n+1)); echo "$batch" | grep -qE "$only" || continue
  log="$G/laws/$(printf '%03d' $n)-$batch.log"
  pat=""; files=""; for a in $args; do case "$a" in -t) pat=next;; *) if [ "$pat" = next ]; then pat=$(printf '%s' "$a" | tr '.' ' '); else files="$files $a"; fi;; esac; done
  case "$mode" in
    art) env=(env -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ROOT/.cache/test-sandbox/ac/$batch"); mkdir -p "$ROOT/.cache/test-sandbox/ac/$batch";;
    optcache) mkdir -p "$ROOT/.cache/test-sandbox/ac-ocv" "$ROOT/.cache/test-sandbox/ac-pw"; env=(env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ROOT/.cache/test-sandbox/ac-ocv" SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT="$ROOT/.cache/test-sandbox/ac-pw");;
    watch) env=(env -u SEMIO_TEST_ARTIFACT_DIR -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_REPO_WATCH_BINARY="$G/watch/new/semio-repo-watch" SEMIO_REPO_WATCH_STALL_BINARY="$G/watch/stall");;
    wire) wire=$(cd "$ROOT" && "$BUN" "$C/cmdid-tools/wire.ts"); mkdir -p "$G/a/$batch"; env=(env SEMIO_TEST_ARTIFACT_DIR="$G/a/$batch" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire");;
    *) env=(env -u SEMIO_TEST_ARTIFACT_DIR -u SEMIO_SCRIPT_PROCESS_INVOCATION);;
  esac
  start=$(date +%s)
  if [ "$mode" = dashenv ]; then mkdir -p "$G/a/dash"; (cd "$ROOT" && "${env[@]}" nice -n 10 "$BUN" "$C/dashhot-tools/env-run.ts" "$ROOT" "@semio-tech/repo-dashboard-rs:test-source-freshness" "$G/a/dash" -- test --timeout 300000 ${pat:+-t "$pat"} $files) > "$log" 2>&1 </dev/null
  else (cd "$ROOT" && "${env[@]}" nice -n 10 "$BUN" test --timeout 170000 ${pat:+-t "$pat"} $files) > "$log" 2>&1 </dev/null; fi
  rc=$?; p=$(grep -E '^ *[0-9]+ pass$' "$log" | tail -1 | tr -dc 0-9); f=$(grep -E '^ *[0-9]+ fail$' "$log" | tail -1 | tr -dc 0-9); e=$(grep -cE '^error:|^# Unhandled' "$log")
  got="${p:-0}/${f:-0}"; verdict=OK; { [ "$want" = - ] && [ "${f:-0}" != 0 ]; } && verdict=FAIL; { [ "$want" != - ] && [ "$want" != "$got" ]; } && verdict=DIFF; [ -z "$p$f" ] && verdict=NORUN
  row="$verdict $batch $mode want=$want got=$got rc=$rc errs=$e $(( $(date +%s)-start ))s $(basename "$log") ::$(printf '%s' "$files" | sed 's#\./🧰️framework/##g' | cut -c1-160)"
  echo "$row" | tee -a "$G/laws.txt"; grep -E '^\(fail\)' "$log" | sed 's/ \[[0-9.]*m\?s\]$//' | sort -u | sed 's/^/      /' | cut -c1-230 | tee -a "$G/laws.txt"
done < "$HERE/laws.tsv"
