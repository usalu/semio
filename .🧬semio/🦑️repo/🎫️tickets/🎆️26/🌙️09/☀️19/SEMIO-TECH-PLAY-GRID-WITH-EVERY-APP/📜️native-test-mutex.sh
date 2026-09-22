#!/bin/zsh
# 🔒 Play fleet NATIVE cargo mutex: at most SLOTS (default 2) cargo test/check invocations of the play fleet at a time
# (shared build dir). FIFO by ticket name. usage: zsh 📜️native-test-mutex.sh <topic> -- <cmd …>
slice="$1"; shift; [ "$1" = "--" ] && shift
SLOTS="${PLAY_NATIVE_SLOTS:-2}"
lockbase="/tmp/semio-play-native-test.lock"
queue="/tmp/semio-play-native-test.queue"
mkdir -p "$queue"
ticket="$queue/$(date '+%Y%m%d%H%M%S')-$$-$slice"
echo $$ > "$ticket"
trap 'rm -f "$ticket"' EXIT INT TERM
slot_dir() { if [ "$1" = 1 ]; then echo "$lockbase"; else echo "$lockbase.$1"; fi; }
held=""
while true; do
  for t in "$queue"/*(N); do
    tp=$(cat "$t" 2>/dev/null)
    if [ -z "$tp" ] || ! kill -0 "$tp" 2>/dev/null; then rm -f "$t"; fi
  done
  rank=$(ls "$queue" 2>/dev/null | sort | grep -n -F "$(basename "$ticket")" | cut -d: -f1)
  if [ -n "$rank" ] && [ "$rank" -le "$SLOTS" ]; then
    for s in $(seq 1 "$SLOTS"); do
      lock=$(slot_dir "$s")
      if mkdir "$lock" 2>/dev/null; then held="$lock"; break; fi
      holder_pid=$(cat "$lock/pid" 2>/dev/null)
      if [ -n "$holder_pid" ] && ! kill -0 "$holder_pid" 2>/dev/null; then rm -rf "$lock"; continue; fi
      if [ -z "$holder_pid" ] && [ -n "$(find "$lock" -maxdepth 0 -mmin +3 2>/dev/null)" ]; then rm -rf "$lock"; continue; fi
    done
    [ -n "$held" ] && break
  fi
  sleep 15
done
echo $$ > "$held/pid"; echo "$slice $(date '+%H:%M:%S')" > "$held/owner"
trap 'rm -rf "$held"; rm -f "$ticket"' EXIT INT TERM
"$@"
