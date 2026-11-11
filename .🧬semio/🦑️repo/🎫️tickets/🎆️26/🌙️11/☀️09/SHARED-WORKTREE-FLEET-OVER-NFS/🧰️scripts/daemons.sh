# usage: source rehearsal-env.sh; source daemons.sh; start_daemon NAME WORKTREE ARGS...; stop_daemon NAME
start_daemon(){ local n="$1" w="$2"; shift 2; ( cd "$w" && exec bun "./$M/$SCRIPT" "$@" ) > "$R/logs/$n.out" 2> "$R/logs/$n.err" < /dev/null & echo "$n $!" >> "$R/pids"; echo "$n pid $!"; }
pid_of(){ awk -v n="$1" '$1==n{p=$2} END{print p}' "$R/pids"; }
stop_daemon(){ local p; p=$(pid_of "$1"); [ -n "$p" ] && kill -TERM "$p" && echo "TERM $1 $p"; }
# restart_all [extra run args]: stop everything recorded, copy the subsystem into the clones, start the 3 daemons and 2 relays
restart_all(){ for n in run-g1-a run-g1-b run-g2-a relay-g1 relay-g2; do stop_daemon $n >/dev/null 2>&1; done; sleep 6; for p in $(awk '{print $2}' "$R/pids"); do kill -0 $p 2>/dev/null && echo "STILL $p"; done; sync_code "$R/g1-a" "$R/g1-b" "$R/g2-a"; : > "$R/pids"
start_daemon run-g1-a "$R/g1-a" run "$@" >/dev/null; start_daemon run-g1-b "$R/g1-b" run "$@" >/dev/null; start_daemon run-g2-a "$R/g2-a" run "$@" >/dev/null
start_daemon relay-g1 "$R/g1-a" relay --group g1 --store "$R/store-g1" --peers g2="$R/store-g2" --worktree "$R/relay-none" --poll 250 >/dev/null; start_daemon relay-g2 "$R/g2-a" relay --group g2 --store "$R/store-g2" --peers g1="$R/store-g1" --worktree "$R/relay-none" --poll 250 >/dev/null; cat "$R/pids"; }
