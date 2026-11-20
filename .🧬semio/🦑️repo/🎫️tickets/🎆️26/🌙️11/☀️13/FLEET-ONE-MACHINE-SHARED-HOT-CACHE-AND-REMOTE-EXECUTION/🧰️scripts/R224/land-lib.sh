# 🛬️ land-lib.sh: the one way a landing script writes into the live, replicated tree. Source it from zsh or bash (`. land-lib.sh`), set LAND_STATE (scratch dir with land.tsv, base/ and new/, see land-manifest.schema.json), optionally define `land_tests` (staged tests that run under the lock; non-zero = red), then call `land_run NAME "what"`.
# Order, binding since R224 (the replica applies a landing file by file, a new imported module can arrive after its importer): window -> lock with owner text and exit trap -> bases compared under the lock -> every NEW file (N) -> wait for the replica delivery (bounded, real acknowledgement of the peers) -> bases compared again -> every MODIFIED file (M) -> tests -> exact restore on any red.
# Test mode: LAND_ROOT=<scratch root> LAND_LOCKBASE=<scratch dir> LAND_NOWINDOW=1 LAND_ACK=skip LAND_FORCE_RED=1 LAND_STORE=<fake store>.
LAND_HERE=$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)
LAND_REAL=${LAND_REAL:-/Users/ueli/Documents/semio}
land_say(){
 local key=$1; shift
 awk -F'\t' -v k="$key" '$1==k{print $2; print $3}' "$LAND_HERE/land-texts.tsv" | { local n=0 line; while IFS= read -r line; do n=$((n+1)); case "${LAND_LANG:-both}" in en) [ $n = 1 ] || continue;; de) [ $n = 2 ] || continue;; esac; printf "$line\n" "$@"; done; }
}
land_sha(){ shasum -a 256 < "$1" | cut -c1-64; }
land_minute(){ echo $((10#$(date -u +%M))); }
land_in_window(){ local m; m=$(land_minute); [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; }; }

land_init(){
 LAND_NAME=$1
 LAND_ROOT=${LAND_ROOT:-$LAND_REAL}
 LAND_LOCKBASE=${LAND_LOCKBASE:-$LAND_REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
 LAND_TSV=${LAND_TSV:-$LAND_STATE/land.tsv}
 LAND_HELD=""; LAND_CREATED=$LAND_STATE/created.dirs; : > "$LAND_CREATED"
 [ -s "$LAND_TSV" ] || { echo "land.tsv missing or empty: $LAND_TSV"; return 1; }
 local bad; bad=$(grep -vE $'^[NM]\t[^\t\\\\/][^\t\\\\]*$' "$LAND_TSV"; grep -E $'(^|/)\\.\\.(/|$)' "$LAND_TSV"); [ -z "$bad" ] || { echo "land.tsv invalid line: $bad"; return 1; }
}

land_wait_window(){
 [ -z "$LAND_NOWINDOW" ] || return 0
 local m left; m=$(land_minute)
 land_in_window && return 0
 if [ $m -ge 35 ]; then left=$(( (60-m)*60 )); else left=$(( (30-m)*60 )); fi
 [ $left -le ${LAND_MAX_WAIT:-720} ] || { land_say window-missed $left ${LAND_MAX_WAIT:-720}; return 4; }
 until land_in_window; do sleep 2; done
}

land_release(){
 local lock held
 held=$(printf '%s\n' "$LAND_HELD" | sed '1!G;h;$!d')
 while IFS= read -r lock; do [ -z "$lock" ] || { rm -f "$lock/owner"; rmdir "$lock" 2>/dev/null; }; done <<EOF
$held
EOF
 LAND_HELD=""
}

land_take(){
 local lock=$1 waited=0 owner
 mkdir -p "$(dirname "$lock")"
 until mkdir "$lock" 2>/dev/null; do
  sleep 2; waited=$((waited+2))
  [ $waited -lt ${LAND_LOCK_WAIT:-150} ] || { land_say no-lock $waited "$(cat "$lock/owner" 2>/dev/null)"; return 1; }
  [ -n "$LAND_NOWINDOW" ] || land_in_window || { land_say window-ended "$(cat "$lock/owner" 2>/dev/null)"; return 4; }
 done
 LAND_HELD="$LAND_HELD
$lock"
 if [ -z "$LAND_NOWINDOW" ] && ! land_in_window; then land_say window-ended "$LAND_NAME"; return 4; fi
 owner="claude-$LAND_NAME ${LAND_DESC:-landing} $(date -u +%Y-%m-%dT%H:%M:%S)Z"
 printf '%s\n' "$owner" > "$lock/owner"
 LAND_WAITED=$waited; LAND_OWNER=$owner
}

land_lock(){
 local lock rc locks="$LAND_LOCKBASE/landing.lock
$LAND_EXTRA_LOCKS"
 while IFS= read -r lock; do [ -z "$lock" ] || { land_take "$lock"; rc=$?; [ $rc = 0 ] || return $rc; }; done <<EOF
$locks
EOF
 LAND_START=$(date -u +%H:%M:%S)
 land_say locked "$LAND_START" "$LAND_WAITED" "$LAND_OWNER"
}

land_check_bases(){
 local kind rel live
 while IFS=$'\t' read -r kind rel; do
  live="$LAND_ROOT/$rel"
  if [ "$kind" = M ]; then [ "$(land_sha "$LAND_STATE/base/$rel")" = "$(land_sha "$live" 2>/dev/null)" ] || { land_say base-differs "$live"; return 2; }
  else [ ! -e "$live" ] || { land_say new-exists "$live"; return 2; }; fi
 done < "$LAND_TSV"
}

land_check_ours(){
 local kind rel live
 while IFS=$'\t' read -r kind rel; do
  live="$LAND_ROOT/$rel"
  if [ "$kind" = M ]; then [ "$(land_sha "$LAND_STATE/base/$rel")" = "$(land_sha "$live" 2>/dev/null)" ] || { land_say changed-meanwhile "$live"; return 2; }
  else [ "$(land_sha "$LAND_STATE/new/$rel")" = "$(land_sha "$live" 2>/dev/null)" ] || { land_say changed-meanwhile "$live"; return 2; }; fi
 done < "$LAND_TSV"
}

land_apply(){
 local want=$1 kind rel live dir missing
 while IFS=$'\t' read -r kind rel; do
  [ "$kind" = "$want" ] || continue
  live="$LAND_ROOT/$rel"; dir=$(dirname "$live"); missing=""
  while [ ! -d "$dir" ]; do missing="$dir
$missing"; dir=$(dirname "$dir"); done
  printf '%s' "$missing" | while IFS= read -r dir; do [ -z "$dir" ] || echo "$dir" >> "$LAND_CREATED"; done
  mkdir -p "$(dirname "$live")" && cp "$LAND_STATE/new/$rel" "$live" || return 3
 done < "$LAND_TSV"
}

land_restore(){
 local kind rel live files=0 dirs=0 skipped=0 dir
 while IFS=$'\t' read -r kind rel; do
  live="$LAND_ROOT/$rel"; [ -e "$live" ] || continue
  if [ "$(land_sha "$live")" != "$(land_sha "$LAND_STATE/new/$rel")" ]; then [ "$kind" = M ] && [ "$(land_sha "$live")" = "$(land_sha "$LAND_STATE/base/$rel")" ] && continue; skipped=$((skipped+1)); land_say restore-skipped "$live"; continue; fi
  if [ "$kind" = M ]; then cp "$LAND_STATE/base/$rel" "$live"; else rm -f "$live"; fi; files=$((files+1))
 done < "$LAND_TSV"
 local created; created=$(sed '1!G;h;$!d' "$LAND_CREATED")
 while IFS= read -r dir; do [ -z "$dir" ] || { rmdir "$dir" 2>/dev/null && dirs=$((dirs+1)); }; done <<EOF
$created
EOF
 land_say restored $files $dirs
}

land_wait_delivery(){
 grep -q $'^N\t' "$LAND_TSV" || return 0
 [ "$LAND_ACK" != skip ] || return 0
 local machine=$LAND_ROOT/.cache/f/machine.json origin store out rc
 origin=$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1]))["originId"])' "$machine") || return 0
 store=${LAND_STORE:-$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1]))["storeRoot"])' "$machine")}
 out=$(python3 -I "$LAND_HERE/land-ack.py" "$store" "$origin" "$LAND_STATE/new.list" "$LAND_STATE/head.before" "${LAND_DELIVERY_LIMIT:-120}" "${LAND_DELIVERY_POLL:-2}"); rc=$?
 if [ $rc = 5 ]; then land_say no-publish "${LAND_DELIVERY_LIMIT:-120}"; return 5; fi
 land_say delivery "$out"
}

land_snapshot(){
 grep $'^N\t' "$LAND_TSV" | cut -f2 > "$LAND_STATE/new.list"
 local machine=$LAND_ROOT/.cache/f/machine.json origin store
 origin=$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1]))["originId"])' "$machine" 2>/dev/null) || { echo '{"segment":0,"bytes":0}' > "$LAND_STATE/head.before"; return 0; }
 store=${LAND_STORE:-$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1]))["storeRoot"])' "$machine")}
 cp "$store/v1/origins/$origin/head.json" "$LAND_STATE/head.before" 2>/dev/null || echo '{"segment":0,"bytes":0}' > "$LAND_STATE/head.before"
}

land_fail(){ local rc=$1; LAND_WRITING=""; land_restore; return $rc; }

land_run(){
 LAND_DESC=$2
 land_init "$1" || return 1
 land_wait_window || return $?
 land_lock || return $?
 land_check_bases || return $?
 land_snapshot
 LAND_WRITING=1
 land_apply N || { land_fail 3; return 3; }
 land_wait_delivery || { land_fail 5; return 5; }
 land_check_ours || { land_fail 2; return 2; }
 land_apply M || { land_fail 3; return 3; }
 [ -z "$LAND_FORCE_RED" ] || { land_say forced-red; land_fail 3; return 3; }
 export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
 if type land_tests >/dev/null 2>&1; then land_tests || { land_say tests-red; land_fail 3; return 3; }; fi
 LAND_WRITING=""
 land_say green "$LAND_START" "$(date -u +%H:%M:%S)"
}

trap land_release EXIT
land_interrupted(){ [ -z "$LAND_WRITING" ] || land_restore; exit 130; }
trap land_interrupted INT TERM
