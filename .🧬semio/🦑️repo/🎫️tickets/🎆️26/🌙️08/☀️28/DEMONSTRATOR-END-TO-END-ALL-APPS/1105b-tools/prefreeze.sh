#!/bin/bash
# 🧊️ Read-only check of every act22 freeze precondition (1105b-audit-act22.md section A) plus later findings.
# Usage: prefreeze.sh        prints one PASS/FAIL line per check, exit 0 only if all pass.
# No repo run, no cargo, no writes; macOS bash 3.2 compatible (no timeout, no mapfile, no associative arrays).
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$HERE"
while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
[ -f "$ROOT/AGENTS.md" ] || { echo "FAIL repo root not found"; exit 2; }
C="$(cd "$HERE/.." && pwd)"
G="$C/🗑️generated/1105b"
cd "$ROOT" || exit 2
fails=0
pass() { echo "PASS $1: $2"; }
fail() { echo "FAIL $1: $2"; fails=$((fails + 1)); }
check() { if [ "$2" = ok ]; then pass "$1" "$3"; else fail "$1" "$3"; fi; }
mtime() { stat -f %m "$1" 2>/dev/null || echo 0; }
newest_of_list() { local n=0 m f; while IFS= read -r f; do [ -n "$f" ] && [ -e "$f" ] || continue; m=$(mtime "$f"); [ "$m" -gt "$n" ] && n=$m; done; echo "$n"; }
stamp() { date -r "$1" '+%m-%d %H:%M' 2>/dev/null || echo "?"; }

free_gib() { df -g "$1" 2>/dev/null | awk 'NR==2 {print $4}'; }

host=$(free_gib "$ROOT"); [ -n "$host" ] || host=0
if [ "$host" -ge 100 ]; then check disk-host ok "${host} GiB free (>= 100)"; else check disk-host bad "${host} GiB free (< 100 GiB required)"; fi

rimg="$ROOT/.cache/b/r/m"; rfree=$(free_gib "$rimg"); [ -n "$rfree" ] || rfree=0
if [ ! -d "$rimg" ]; then check disk-cache-r bad "$rimg missing"; elif mount | grep -q "$rimg"; then
  if [ "$rfree" -ge 20 ]; then check disk-cache-r ok "${rfree} GiB free inside .cache/b/r/m (own mount; >= 20)"; else check disk-cache-r bad "${rfree} GiB free inside .cache/b/r/m (< 20 GiB)"; fi
else check disk-cache-r bad ".cache/b/r/m is not a mounted image (shares the host volume, ${rfree} GiB free)"; fi

rss=$(ps -axo rss=,comm= | awk '/fseventsd/ {s+=$1} END {print s+0}')
if [ "$rss" -lt 4194304 ]; then check fseventsd ok "RSS $((rss / 1024)) MB (< 4096 MB)"; else check fseventsd bad "RSS $((rss / 1024)) MB (>= 4096 MB starves the door)"; fi

slots="$G/slots"; held=$(ls -d "$slots"/s* 2>/dev/null | wc -l | tr -d ' '); queued=$(ls "$slots/queue" 2>/dev/null | wc -l | tr -d ' ')
if [ "$held" = 0 ] && [ "$queued" = 0 ]; then check slots ok "no slot held, queue empty"; else check slots bad "$held slot dir(s) held, $queued queued ($(cat "$slots"/s*/label 2>/dev/null | tr '\n' ',' ))"; fi
k=$(cat "$HERE/slots.k" 2>/dev/null); w=$(cat "$HERE/slots.wextra" 2>/dev/null)
if [ "$k" = 0 ] && [ "$w" = 0 ]; then check slot-policy ok "slots.k=0 slots.wextra=0"; else check slot-policy bad "slots.k=${k:-?} slots.wextra=${w:-?} (freeze policy needs 0 and 0)"; fi

latest() { ls "$G"/"$1"-*."$2" 2>/dev/null | sed -E "s/.*$1-([0-9]+)\.$2$/\1 &/" | sort -n | tail -1; }
lockrow=$(latest lock rc); lockfile=${lockrow#* }; locknum=${lockrow%% *}
if [ -z "$lockrow" ]; then check lock-run bad "no lock-N.rc found"; else
  lv=$(cat "$lockfile" 2>/dev/null)
  if [ "$lv" = 0 ] && [ -f "$G/lock-ok" ]; then check lock-run ok "lock-$locknum.rc=0 ($(stamp "$(mtime "$lockfile")")), lock-ok present"; else check lock-run bad "lock-$locknum.rc=${lv:-?}, lock-ok $( [ -f "$G/lock-ok" ] && echo present || echo missing)"; fi; fi
syncnum=$(ls "$G"/sync-*.txt "$G"/sync-*.rc 2>/dev/null | sed -E 's/.*sync-([0-9]+)\..*/\1/' | sort -n | tail -1)
if [ -z "$syncnum" ]; then check sync-run bad "no sync-N files found"; else
  sfile="$G/sync-$syncnum.txt"; [ -f "$sfile" ] || sfile="$G/sync-$syncnum.rc"; smt=$(mtime "$sfile")
  sproof=""
  [ "$(cat "$G/sync-$syncnum.rc" 2>/dev/null)" = 0 ] && sproof="sync-$syncnum.rc=0"
  if [ -z "$sproof" ] && [ -f "$G/sync-$syncnum.txt" ] && tail -n 3 "$G/sync-$syncnum.txt" | grep -Eq '(^|[^a-z])(rc|exit)[ =:]+0[[:space:]]*$'; then sproof="sync-$syncnum.txt ends in rc 0"; fi
  if [ -z "$sproof" ] && [ -n "$lockfile" ] && [ "$lv" = 0 ] && [ "$(mtime "$lockfile")" -ge "$smt" ]; then sproof="lock-$locknum.rc=0 is newer than sync-$syncnum (combined run)"; fi
  if [ -n "$sproof" ] && [ -f "$G/sync-done" ]; then check sync-run ok "$sproof, sync-done present"; else check sync-run bad "no rc-0 proof for sync-$syncnum ($(basename "$sfile") $(stamp "$smt")), sync-done $( [ -f "$G/sync-done" ] && echo present || echo missing)"; fi; fi
if [ -n "$lockfile" ]; then
  newer=$({ git ls-files '*Cargo.toml'; git ls-files -o --exclude-standard '*Cargo.toml'; } | while IFS= read -r f; do [ -e "$f" ] && [ "$f" -nt "$lockfile" ] && echo "$f"; done | head -5)
  if [ -z "$newer" ]; then check cargo-vs-lock ok "no Cargo.toml newer than lock-$locknum.rc"; else check cargo-vs-lock bad "newer than lock-$locknum.rc: $(echo "$newer" | head -3 | tr '\n' ' ')"; fi
else check cargo-vs-lock bad "no lock run to compare with"; fi

skipdbg=(':!.🧬semio' ':!🎓️teaching/🏛️architecture/📚️course/🚀️deploy' ':!🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy')
dbg=$(git grep -n -I -o -F '[DEBUG]' -- '*.rs' '*.ts' '*.tsx' "${skipdbg[@]}" 2>/dev/null | sed 's/:\[DEBUG\]$//')
dbgu=$(git ls-files -o --exclude-standard -- '*.rs' '*.ts' '*.tsx' "${skipdbg[@]}" 2>/dev/null | while IFS= read -r f; do grep -n -F '[DEBUG]' "$f" 2>/dev/null | cut -d: -f1 | sed "s|^|$f:|"; done)
dbgall=$(printf '%s\n%s\n' "$dbg" "$dbgu" | grep .)
if [ -z "$dbgall" ]; then check no-debug-lines ok "no [DEBUG] in tracked/untracked .rs/.ts/.tsx (course deploy sealed-site checkers excluded: they name the marker as a string)"; else check no-debug-lines bad "$(printf '%s\n' "$dbgall" | wc -l | tr -d ' ') [DEBUG] line(s) in $(printf '%s\n' "$dbgall" | cut -d: -f1 | sort -u | wc -l | tr -d ' ') file(s), first 12:"; printf '%s\n' "$dbgall" | head -12 | sed 's/^/       /'; fi

pinfile="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json"
pin=$(sed -nE 's/.*"channelVersion"[[:space:]]*:[[:space:]]*([0-9]+).*/\1/p' "$pinfile" | head -1)
rs=$(sed -nE 's/^pub const CHANNEL_VERSION: u32 = ([0-9]+);.*/\1/p' "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs" | head -1)
ts=$(sed -nE 's/^export const APP_CHANNEL_VERSION = ([0-9]+);.*/\1/p' "🧰️framework/🔨️modules/🧾️app-channel/🔢️request-sequence/🟦️.ts" | head -1)
ba=$(sed -nE 's/^export const BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION = ([0-9]+);.*/\1/p' "🧰️framework/🔨️modules/🌐️browser-bundle/🎯️action-handoff/🟦️.ts" | head -1)
if [ -n "$pin" ] && [ "$pin" = "$rs" ] && [ "$pin" = "$ts" ] && [ "$pin" = "$ba" ]; then check channel-pin-code ok "pin=$pin rust=$rs ts=$ts browser-actor=$ba"; else check channel-pin-code bad "pin=${pin:-?} rust=${rs:-?} ts=${ts:-?} browser-actor=${ba:-?}"; fi
stale=$(git grep -l -E '"appChannelVersion"[[:space:]]*:[[:space:]]*[0-9]+' -- '*.json' 2>/dev/null | while IFS= read -r f; do grep -E '"appChannelVersion"[[:space:]]*:[[:space:]]*[0-9]+' "$f" | grep -qvE "\"appChannelVersion\"[[:space:]]*:[[:space:]]*${pin:-0}\b" && echo "$f"; done | wc -l | tr -d ' ')
if [ "$stale" = 0 ]; then echo "INFO channel-pin-descriptors: no tracked descriptor JSON with an appChannelVersion other than $pin"; else echo "INFO channel-pin-descriptors: $stale tracked descriptor JSON file(s) still carry another appChannelVersion; expected action after activation: \`repo run workspace:components describe <id> --profile prototype\` (write mode) refreshes them (not a freeze blocker)"; fi

ext="$G/ext-ok"
if [ ! -f "$ext" ]; then check ext-ok bad "ext-ok missing"; else
  scope=$({ git ls-files -- '🌎️hub' ; git ls-files -o --exclude-standard -- '🌎️hub'; git ls-files -- '✏️s/🔌️plugins/*/🧩️extensions/**' ; git ls-files -o --exclude-standard -- '✏️s/🔌️plugins/*/🧩️extensions/**'; } | grep -E '\.(rs|toml|wit)$')
  newest=$(printf '%s\n' "$scope" | newest_of_list); em=$(mtime "$ext")
  if [ "$em" -gt "$newest" ]; then check ext-ok ok "ext-ok $(stamp "$em") newer than last hub/ext crate edit $(stamp "$newest")"; else
    who=$(printf '%s\n' "$scope" | while IFS= read -r f; do [ -e "$f" ] && [ "$f" -nt "$ext" ] && echo "$f"; done | head -3 | tr '\n' ' ')
    check ext-ok bad "ext-ok $(stamp "$em") older than last hub/ext edit $(stamp "$newest"): $who"; fi; fi

prof="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile"
dsch=$(grep -A3 '"prototype-wasm": {' "$prof/🧬️schema/🔣️.json" | sed -nE 's/.*"debug": *([0-9]+).*/\1/p' | head -1)
dfix=$(grep -A1 '"opt-level": 1,' "$prof/🧪️tests/📥️consumption/🔣️.json" 2>/dev/null | grep -c '"debug": 1,')
if [ "$dsch" = 0 ] && [ "$dfix" = 0 ]; then check wasm-debug0 ok "prototype-wasm debug=0 in the profile schema and its consumption fixture"; elif [ "$dsch" = 0 ]; then check wasm-debug0 bad "schema debug=0 but $dfix consumption fixture row(s) still debug=1"; else check wasm-debug0 bad "prototype-wasm debug=${dsch:-?} in the profile schema (patch prototype-wasm-debug0.py not applied)"; fi

echo "--"
if [ "$fails" = 0 ]; then echo "ALL PASS"; exit 0; fi
echo "$fails check(s) FAIL"; exit 1
