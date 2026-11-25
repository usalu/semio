#!/bin/sh
# 🎟️ Holds ONE capacity grant and runs the jobs dropped into a folder one after the other, so that a series of short measurements waits in the queue once: grant.sh <memory MB> <jobs folder>
# A job is a file <name>.sh; its output goes to <name>.log (the last line `job ended <code>`), the file becomes <name>.done. The grant is given back when the folder holds a file `stop` or after fifteen idle minutes.
set -u
if [ "${1:-}" != "--inside" ]; then
  cd /Users/ueli/Documents/semio || exit 1
  exec repo capacity run --memory="$1" -- sh "$0" --inside "$1" "$2"
fi
dir=$3; idle=0
touch "$dir/admitted"
while [ ! -e "$dir/stop" ] && [ "$idle" -lt 900 ]; do
  job=$(ls "$dir"/*.sh 2>/dev/null | head -1)
  if [ -z "$job" ]; then sleep 1; idle=$((idle + 1)); continue; fi
  idle=0
  name=${job%.sh}
  mv "$job" "$name.running"
  /usr/bin/time -l sh "$name.running" > "$name.log" 2>&1
  echo "job ended $?" >> "$name.log"
  mv "$name.running" "$name.done"
done
rm -f "$dir/admitted" "$dir/stop"
