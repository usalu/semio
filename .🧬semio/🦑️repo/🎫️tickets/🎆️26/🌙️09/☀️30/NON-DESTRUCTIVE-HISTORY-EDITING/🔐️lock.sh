#!/bin/zsh
# 🔐️ Fleet locks (rule 51): one writer at a time lands a wave in the shared trees, so the live tree is green between waves and the
# coordinator can activate from it. Locks: `landing` (framework Rust), `stdio`, `puzzle`, `hub` (those trees), `serve` (bundled TypeScript);
# a wave takes every lock whose tree it writes, in that order, and releases in reverse. `lock.sh acquire <lock> <wp> [wait-seconds]` blocks
# (default 480 s, exit 3 on timeout), `lock.sh release <lock> <wp>` frees your own lock, `lock.sh status` prints every holder. A lock is a directory created
# atomically; waiters queue first-in-first-out (a ticket kept alive by the waiting call, dropped 90 s after it stops waiting); nobody
# breaks a lock they do not own — report a holder older than 30 min to the coordinator instead.
dir="${0:A:h}/🗑️generated/coord/locks"
mkdir -p "$dir"
verb="$1"; name="$2"; wp="$3"; wait="${4:-480}"
case "$verb" in
  status)
    for lock in landing stdio puzzle hub serve; do
      waiting="$(for ticket in "$dir/$lock.queue"/*(N); do echo "$(cat "$ticket") ${ticket:t}"; done | sort -n | awk '{printf "%s ", $2}')"
      if [ -d "$dir/$lock" ]; then echo "$lock: HELD by $(cat "$dir/$lock/owner" 2>/dev/null) since $(cat "$dir/$lock/since" 2>/dev/null)${waiting:+ | queue: $waiting}"; else echo "$lock: free${waiting:+ | queue: $waiting}"; fi
    done ;;
  acquire)
    case "$name" in landing|stdio|puzzle|hub|serve) ;; *) echo "unknown lock: $name"; exit 2 ;; esac
    [ -n "$wp" ] || { echo "usage: acquire <landing|stdio|puzzle|hub|serve> <wp> [wait-seconds]"; exit 2; }
    [ "$(cat "$dir/$name/owner" 2>/dev/null)" = "$wp" ] && { echo "$name: already yours"; exit 0; }
    zmodload zsh/datetime
    queue="$dir/$name.queue"; mkdir -p "$queue"
    [ -f "$queue/$wp" ] || print -r -- "$EPOCHREALTIME" > "$queue/$wp"
    waited=0
    while true; do
      touch "$queue/$wp"
      head=""; best=""
      for ticket in "$queue"/*(N); do
        [ $(( EPOCHSECONDS - $(stat -f %m "$ticket" 2>/dev/null || echo 0) )) -gt 90 ] && { rm -f "$ticket"; continue; }
        stamp="$(cat "$ticket" 2>/dev/null)"
        if [ -z "$best" ] || (( stamp < best )); then best="$stamp"; head="${ticket:t}"; fi
      done
      [ -d "$dir/$name" ] && [ ! -f "$dir/$name/owner" ] && [ $(( EPOCHSECONDS - $(stat -f %m "$dir/$name" 2>/dev/null || echo $EPOCHSECONDS) )) -gt 30 ] && rmdir "$dir/$name" 2>/dev/null && echo "$(date '+%F %T') healed ownerless $name" >> "$dir/events.txt"
      if [ "$head" = "$wp" ] && mkdir "$dir/$name" 2>/dev/null; then echo "$wp" > "$dir/$name/owner"; rm -f "$queue/$wp"; break; fi
      [ "$waited" -ge "$wait" ] && { echo "$name: still HELD by $(cat "$dir/$name/owner" 2>/dev/null) since $(cat "$dir/$name/since" 2>/dev/null); queue head: $head — your place is kept for 90 s, re-issue acquire"; exit 3; }
      sleep 10; waited=$((waited + 10))
    done
    echo "$wp" > "$dir/$name/owner"; date '+%F %T' > "$dir/$name/since"
    echo "$(date '+%F %T') acquire $name $wp" >> "$dir/events.txt"
    echo "$name: acquired by $wp" ;;
  release)
    [ -d "$dir/$name" ] || { echo "$name: not held"; exit 0; }
    [ "$(cat "$dir/$name/owner" 2>/dev/null)" = "$wp" ] || { echo "$name: held by $(cat "$dir/$name/owner" 2>/dev/null), not by $wp"; exit 4; }
    rm -f "$dir/$name/owner" "$dir/$name/since"; rmdir "$dir/$name"
    echo "$(date '+%F %T') release $name $wp" >> "$dir/events.txt"
    echo "$name: released by $wp" ;;
  *) echo "usage: lock.sh acquire|release <landing|stdio|puzzle|hub|serve> <wp> [wait-seconds] | status"; exit 2 ;;
esac
