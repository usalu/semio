#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/wsr"
M="$1"; ID=$(print -rn -- "$M" | shasum | cut -c1-10)
export SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":14400000}'
gate(){ while ps -Ao command | command grep -E 'bun repo dependencies (lock|sync)' | command grep -v grep >/dev/null; do sleep 60; done; }
run(){ gate; perl -e 'alarm 550; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- "$@" >"$G/$ID.out" 2>"$G/$ID.full"; }
try(){
  for i in 1 2 3 4 5 6; do
    run metadata --format-version 1 --manifest-path "$M"; rc=$?
    command grep -qE 'System image plist required|Bounded unique parent kernel roster required|Stale loaded repository|topology changed|Queued cache maintenance' "$G/$ID.full" && { sleep 20; continue; }
    break
  done
}
try
if [ $rc -ne 0 ] && command grep -qE 'cannot (update|create) the lock file' "$G/$ID.full" && [ -n "$UPDATE" ]; then
  run update --workspace --manifest-path "$M"; try
fi
echo "$rc" > "$G/$ID.rc"
err=$(command grep -v '^\[script\]\|^\[DEBUG\|^[0-9]* |\|^ *at \|^\$ bun' "$G/$ID.full" | command grep -E '^error|^Caused|^  [a-z]' | tr '\n' ' ' | cut -c1-600)
printf '%s\t%s\t%s\n' "$rc" "$M" "$err" >> "$G/results.tsv"
