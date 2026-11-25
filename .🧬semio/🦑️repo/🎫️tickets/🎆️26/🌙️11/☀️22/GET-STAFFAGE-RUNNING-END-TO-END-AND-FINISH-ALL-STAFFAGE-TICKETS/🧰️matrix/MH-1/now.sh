#!/bin/zsh
# 🧾️ The true state of assets now, measured on the catalog files under ONE grant of the memory queue (2500 MB, the steps one after the other inside it): the authors' scan of all given ids (log $GEN/now/scan.log) and the full gate of each (log $GEN/gate/now-<id>.log, last line printed), with the md5 of the file before and after each gate in $GEN/now/md5.log so a result is tied to the bytes it measured: now.sh <id>…   (inside the grant the script calls itself with INSIDE=1)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/now" "$GEN/gate"
if [[ -z "$INSIDE" ]]; then INSIDE=1 exec "$W/q.sh" 2500 ${NOW_TIMEOUT:-14400} "$T" "${0:A}" "$@"; fi
"$B" 🔎️scan.ts --language en --quiet "$@" > "$GEN/now/scan.log" 2>&1; cut -c1-200 "$GEN/now/scan.log"
for a in "$@"; do
  f="$S/🗂️catalog/👤️humans/$a/🔣️.json"; m0="$(md5 -q "$f")"
  "$B" "$M/gate.ts" "$a" --twins > "$GEN/gate/now-$a.log" 2>&1
  echo "$a $m0 $(md5 -q "$f") $(date +%H:%M) $(tail -1 "$GEN/gate/now-$a.log")" | tee -a "$GEN/now/md5.log"
done
