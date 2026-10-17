#!/bin/zsh
# usage: checkapplied.sh <label> <tag:plugin/🗿️artifacts/artifact>...   runs the layout gate (new crates --check-applied --base=f856dff0813) per package; retries while the TS library fails to load (peer edits in flight)
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SC=/Users/ueli/.cache/tc2/ca
export LC_ALL=en_US.UTF-8
ulimit -n 65536
L=$1; shift
cd $R
run_one() {
  n=$1; d=$2
  for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do
    rm -rf "$SC-$n-$L"
    nice -n 10 perl -e 'alarm 3400; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/$d/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SC-$n-$L" > "$F/🗑️generated/tc2-checkapplied-$n-$L.log" 2>&1
    code=$?
    if ! /usr/bin/grep -q "SyntaxError\|Cannot find module\|error: Cannot" "$F/🗑️generated/tc2-checkapplied-$n-$L.log"; then break; fi
    sleep 150
  done
  echo "exit $code attempts $attempt" >> "$F/🗑️generated/tc2-checkapplied-$n-$L.log"
}
for a in "$@"; do
  n=${a%%:*}; d=${a##*:}
  run_one "$n" "$d" &
done
wait
echo CHECKAPPLIED-DONE > "$F/🗑️generated/tc2-checkapplied-$L.done"
