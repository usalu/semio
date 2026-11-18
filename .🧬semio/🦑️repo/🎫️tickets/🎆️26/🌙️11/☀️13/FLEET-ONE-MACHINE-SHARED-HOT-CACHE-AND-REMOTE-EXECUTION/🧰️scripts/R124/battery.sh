#!/bin/zsh
# usage: battery.sh <out file> [parallel]; runs every owner-pool test and the two standing preflights through the harness
R=/Users/ueli/Documents/semio
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
O="$L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests"
SI="$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs"
OUT=$1; P=${2:-4}; : > $OUT
one(){ f=$1; r=$($BUN "$L/🧪️testing/🌱️genuine-birth/📜️script.ts" --file="$f" --birth=setup --root=$R 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s ' \n' ' '); echo "${f#$L/} :: $r" >> $OUT; }
files=("$O/🟦️.ts" "$O/🧮️lifecycle/🟦️.ts" "$O/🎚️lanes/🟦️.ts" "$O/🌊️early-wave/🟦️.ts" "$O/🏋️heavy/🟦️.ts" "$O/🌍️spread/🟦️.ts" "$O/🔁️no-record/🟦️.ts" "$O/🚪️exits/🟦️.ts" "$O/🪦️reaper/🟦️.ts" "$O/🔔️keep-alive/🟦️.ts" "$SI/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$SI/🪆️heavy-questions/🧪️tests/🟦️.ts")
[ -f "$O/🎛️one-budget/🟦️.ts" ] && files+=("$O/🎛️one-budget/🟦️.ts")
G="$L/🏗️build/🪪️graph-state/📦️project-records"
files+=("$G/🌱️fresh/🧪️tests/🟦️.ts" "$G/🧪️tests/🟦️.ts" "$G/🧪️tests/🌊️seed-stream/🟦️.ts" "$G/🧪️tests/✂️after-cut/🟦️.ts" "$G/🧪️tests/🚚️closures/🟦️.ts" "$G/🧪️tests/🏷️owner-token/🟦️.ts" "$G/🧪️tests/🗜️packed/🟦️.ts" "$G/🧪️tests/📮️retry/🟦️.ts" "$G/🧪️tests/🌍️two-platforms/🟦️.ts" "$G/🧪️tests/🤝️pool-reader/🟦️.ts")
for f in $files; do if [ -n "$R124_ONLY" ] && ! print -r -- "$f" | /usr/bin/grep -q -F -f <(print -r -- "$R124_ONLY"); then continue; fi; one "$f" & while [ $(jobs -r | wc -l) -ge $P ]; do sleep 1; done; done; wait
sort -o $OUT $OUT
