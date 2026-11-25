#!/bin/bash
# usage: watch.sh <label> <budget-seconds> <test-file>
label=$1 budget=$2 file=$3
out="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🗑️generated/T5"
mkdir -p "$out"; cd /Users/ueli/Documents/semio
LC_ALL=C repo test-file --lang=en "$file" >"$out/$label.out" 2>"$out/$label.err" &
pid=$!; start=$(date +%s)
tree(){ local p; for p in $(pgrep -P "$1"); do echo "$p"; tree "$p"; done; }
while kill -0 $pid 2>/dev/null; do
  if (( $(date +%s) - start > budget )); then
    echo "[watchdog] budget $budget s exceeded at $(date)" >>"$out/$label.err"
    pids=$(tree $pid)
    for b in $pids; do ps -o args= -p $b | grep -q -- "bun test" && sample $b 5 -file "$out/$label.sample.$b.txt" >/dev/null 2>&1; done
    kill -TERM $pid; sleep 10; kill -KILL $pid $pids 2>/dev/null
    break
  fi
  sleep 3
done
wait $pid; echo "exit=$? seconds=$(( $(date +%s) - start ))" >>"$out/$label.out"
cat "$out/$label.out"
grep -aE "\(pass\)|\(fail\)|timed out|pass$|fail$| pass| fail|error:|Expected|Received" "$out/$label.err" | grep -av "remains active" | cut -c1-300 | tail -60
