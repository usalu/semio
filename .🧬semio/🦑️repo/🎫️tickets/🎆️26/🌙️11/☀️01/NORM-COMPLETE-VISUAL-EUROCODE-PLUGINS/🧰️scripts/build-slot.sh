#!/usr/bin/env bash
# 🎟️ Runs one `repo run <target>` inside one of SLOTS global flock slots; usage: build-slot.sh <project:target> <log>
set -u
target="$1"; log="$2"; extra=("${@:3}"); slots="${SEMIO_FLEET_SLOTS:-1}"
dir="$(dirname "$0")/../🗑️generated/build-slots"; mkdir -p "$dir" "$(dirname "$log")"
echo "[slot] waiting for one of $slots slots for $target ($(date +%T))" > "$log"
while :; do
  for i in $(seq 1 "$slots"); do
    exec {fd}>"$dir/slot-$i.lock"
    if flock -n "$fd"; then
      echo "$target $$ $(date +%T)" > "$dir/slot-$i.owner"
      echo "[slot] acquired slot $i ($(date +%T))" >> "$log"
      cd /home/deck/semio && SEMIO_LANGUAGE=en SEMIO_NX_GRAPH_CAPACITY_WAIT_MS=5400000 repo run "$target" --output=static "${extra[@]}" >> "$log" 2>&1
      code=$?; echo "exit $code" >> "$log"; rm -f "$dir/slot-$i.owner"; exit "$code"
    fi
    exec {fd}>&-
  done
  sleep 20
done
