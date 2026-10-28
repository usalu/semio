#!/bin/sh
# usage: conc.sh <tag> : two cargo builds of DIFFERENT crates (labels, preferences), each with a one-line edit: sequential in one lane (what a whole-directory custody forces) vs concurrent in two lanes. Lanes rustg-<tag>-1 / -2 are warmed first.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/rust-g"; T="$C/rust-g-tools"; TAG="$1"
unset SEMIO_SCRIPT_STALL_MS; export SEMIO_CARGO_TIMING=0
cargo_check() { sh "$T/healthy.sh"; CLAUDE_CODE_SESSION_ID="rustg-$TAG-$1" nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p "$2" --lib >> "$G/$TAG-conc.log" 2>&1; }
measure() { sh "$T/healthy.sh"; python3 "$T/measure.py" "$TAG-$1" "$G/runs.jsonl" -- sh -c "$2" >> "$G/$TAG-conc.out" 2>&1; }
revert() { python3 "$T/probe.py" revert labels; python3 "$T/probe.py" revert prefs; }
trap revert EXIT INT TERM
A=semio-framework-repo-dashboard-labels; B=semio-framework-repo-dashboard-preferences
for lane in 1 2; do cargo_check $lane $A; cargo_check $lane $B; done
for round in 1 2; do
  python3 "$T/probe.py" apply labels; python3 "$T/probe.py" apply prefs
  measure "seq$round" "CLAUDE_CODE_SESSION_ID=rustg-$TAG-1 nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p $A --lib >/dev/null 2>&1; CLAUDE_CODE_SESSION_ID=rustg-$TAG-1 nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p $B --lib >/dev/null 2>&1"
  revert
  python3 "$T/probe.py" apply labels; python3 "$T/probe.py" apply prefs
  measure "par$round" "CLAUDE_CODE_SESSION_ID=rustg-$TAG-1 nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p $A --lib >/dev/null 2>&1 & CLAUDE_CODE_SESSION_ID=rustg-$TAG-2 nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p $B --lib >/dev/null 2>&1 & wait"
  revert
done
echo done
