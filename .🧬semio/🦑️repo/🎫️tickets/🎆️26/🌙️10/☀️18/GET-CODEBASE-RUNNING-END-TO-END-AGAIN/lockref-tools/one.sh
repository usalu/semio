#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/lockref"
M="$1"; ID=$(print -rn -- "$M" | shasum | cut -c1-10)
export SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":14400000}'
gate(){ while ps -Ao command | command grep -E 'bun repo dependencies (lock|sync)' | command grep -v grep >/dev/null; do sleep 2; done; }
run(){ gate; perl -e 'alarm 3000; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- "$@" >"$G/$ID.out" 2>"$G/$ID.full"; }
try(){
  for i in {1..15}; do
    run "$@"; rc=$?
    command grep -qE 'System image plist required|Bounded unique parent kernel roster required|Stale loaded repository|topology changed|Queued cache maintenance|SyntaxError|Export named|Cannot find module' "$G/$ID.full" && [ $rc -ne 0 ] && { sleep 60; continue; }
    break
  done
}
errof(){ command grep -v '^\[script\]\|^\[DEBUG\|^[0-9]* |\|^ *at \|^\$ bun' "$G/$ID.full" | command grep -E '^error|^Caused|^  [a-z]|lock' | tr '\n\t' '  ' | cut -c1-400; }
try metadata --locked --format-version 1 --manifest-path "$M"
b=$rc; berr=$(errof); after=$rc; aerr=$berr; act=none
if [ $b -ne 0 ] && print -r -- "$berr" | command grep -qiE 'lock file|Cargo\.lock|--locked'; then
  act=update
  try update --workspace --manifest-path "$M"; ur=$rc; uerr=$(errof)
  try metadata --locked --format-version 1 --manifest-path "$M"; after=$rc; aerr=$(errof)
  [ $ur -ne 0 ] && aerr="update rc=$ur $uerr | $aerr"
fi
printf '%s\t%s\t%s\t%s\t%s\n' "$M" "$b" "$act" "$after" "${aerr:-$berr}" >> "$G/results.tsv"
