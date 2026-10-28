#!/bin/sh
# Retries the real `repo cache gc --area=nx-tooling` with the longest idle poll until a run retired something (or 4 attempts, about 40 minutes). Output per attempt in 🗑️generated/nxret-g/loop-<n>.txt, state before/after in loop-state.txt. Kill only this script's own pid (written to loop.pid).
R="/Users/ueli/Documents/semio"
C="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/nxret-g"
mkdir -p "$G"
cd "$R" || exit 1
echo $$ > "$G/loop.pid"
{ echo "start $(date +%H:%M:%S)"; df -k .cache/b/t/m | tail -1; echo "identities $(ls .cache/b/t/m/nx | wc -l)"; } > "$G/loop-state.txt"
n=1
while [ $n -le 4 ]; do
  SEMIO_NX_TOOLING_IDLE_WAIT_MS=600000 nice -n 10 bun repo cache gc --area=nx-tooling --lang=en > "$G/loop-$n.txt" 2>&1
  if grep -q "^Retired [1-9]" "$G/loop-$n.txt"; then break; fi
  n=$((n+1))
done
{ echo "end $(date +%H:%M:%S) attempts $n"; df -k .cache/b/t/m | tail -1; echo "identities $(ls .cache/b/t/m/nx | wc -l)"; } >> "$G/loop-state.txt"
rm -f "$G/loop.pid"
