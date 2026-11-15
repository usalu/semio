#!/bin/bash
# 1113-two-os.sh: macOS host replica (origin g1-server) and Linux container replica (origin g1-m01) on one temporary store.
# usage: 1113-two-os.sh [out-dir]   env: BUN (host bun), REAL (live worktree, default /Users/Shared/semio), IMAGE (default semio-linux-bun)
# Everything lives under a fresh temp dir; the live worktree is only read (module copy via mirror.sh, mounted read-only into the container).
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
REAL=${REAL:-/Users/Shared/semio}; IMAGE=${IMAGE:-semio-linux-bun}
BUN=${BUN:-/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
OUT=${1:-$HERE/../🗑️generated}; mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd)
W=$(cd "$(mktemp -d "${TMPDIR:-/tmp}/semio-two-os.XXXXXX")" && pwd -P)
CN=semio-two-os-$$
LOG="$OUT/1113-two-os-transcript.txt"; : > "$LOG"
say(){ printf '%s\n' "$*" | tee -a "$LOG"; }
export GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@example.test GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@example.test GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
for v in $(env | sed -n 's/^\(SEMIO_[A-Za-z0-9_]*\)=.*/\1/p'); do unset "$v"; done
MPID=
cleanup(){ [ -n "$MPID" ] && kill "$MPID" 2>/dev/null; docker rm -f "$CN" >/dev/null 2>&1; rm -rf "$W"; }
[ -n "${KEEP:-}" ] || trap cleanup EXIT
MOD=$("$HERE/1113-linux/mirror.sh" "$REAL" "$W/mir") || exit 2
MODREL=${MOD#"$W/mir/"}; SCRIPT="$MODREL/$(cd "$MOD" && ls -d *replica)/$(cd "$MOD"/*replica && ls 📜️script.ts)"
mkdir -p "$W/store" "$W/mac"
say "temp dir $W; module copy $MODREL (copied once, so peers editing the live module cannot restart the daemons)"
M(){ ( cd "$W/mac" && "$BUN" "$W/mir/$SCRIPT" "$@" ); }
L(){ docker exec -w /lw "$CN" bun "/mir/$SCRIPT" "$@"; }
LG(){ docker exec -w /lw "$CN" git "$@"; }
MG(){ ( cd "$W/mac" && git "$@" ); }
wait_for(){ local what=$1 secs=$2; shift 2; local i; for ((i=0;i<secs*2;i++)); do if "$@" >/dev/null 2>&1; then say "ok   $what (${i}x0.5s)"; return 0; fi; sleep 0.5; done; say "FAIL $what after ${secs}s"; return 1; }
FAILED=0; chk(){ "$@" || FAILED=1; }

say "== seed a small git repo on macOS"
cd "$W/mac" && git init -q -b main && printf '.cache/\nnode_modules/\n*.log\n' > .gitignore && printf 'alpha\n' > a.txt && mkdir dir && printf 'charlie\n' > dir/c.txt && git add -A && git commit -q -m initial
say "mac HEAD $(MG rev-parse HEAD)"
say "== seed (origin g1-server) and run the macOS daemon"
M seed --group g1 --origin g1-server --worktree "$W/mac" --store "$W/store" 2>&1 | tee -a "$LOG" | tail -3
(cd "$W/mac" && exec nohup "$BUN" "$W/mir/$SCRIPT" run --worktree "$W/mac" --store "$W/store" --poll 100 --scan 5 > "$OUT/1113-two-os-mac-daemon.log" 2>&1 < /dev/null) &
MPID=$!
say "== start the Linux container (origin g1-m01): join, then run"
docker run -d --name "$CN" -v "$REAL:$REAL:ro" -v "$W/mir:/mir:ro" -v "$W/store:/store" -e "GIT_AUTHOR_NAME=t" -e "GIT_AUTHOR_EMAIL=t@example.test" -e "GIT_COMMITTER_NAME=t" -e "GIT_COMMITTER_EMAIL=t@example.test" "$IMAGE" bash -c "mkdir -p /lw && cd /lw && bun /mir/$SCRIPT join --group g1 --origin g1-m01 --worktree /lw --store /store && exec bun /mir/$SCRIPT run --worktree /lw --store /store --poll 100 --scan 5" > /dev/null
wait_for "linux joined (a.txt present)" 90 docker exec "$CN" test -f /lw/a.txt || { docker logs "$CN" 2>&1 | tail -20 | tee -a "$LOG"; exit 1; }
# the scripts run with the worktree as cwd, so the repo bunfig runtime-pin preload (cwd-only) does not apply.
# NOTE: the mirror is mounted at /mir, but symlinks into the live worktree resolve because it is mounted at its host path too.
say "linux: $(docker exec "$CN" uname -sm), bun $(docker exec "$CN" bun --version), git $(docker exec "$CN" git --version)"
say "linux HEAD after join $(LG rev-parse HEAD 2>&1)"
wait_for "linux HEAD equals mac HEAD after join" 60 bash -c "[ \"\$(docker exec -w /lw $CN git rev-parse HEAD)\" = \"$(MG rev-parse HEAD)\" ]" ; chk true

say "== 1. edit a file on macOS, it appears on Linux"
printf 'charlie edited on macOS %s\n' "$$" > "$W/mac/dir/c.txt"
chk wait_for "linux dir/c.txt equals mac" 60 bash -c "[ \"\$(docker exec $CN cat /lw/dir/c.txt)\" = \"\$(cat '$W/mac/dir/c.txt')\" ]"
say "linux sees: $(docker exec "$CN" cat /lw/dir/c.txt)"

say "== 2. stage on Linux, macOS gets the same write-tree"
docker exec "$CN" sh -c "printf 'new on linux\n' > /lw/new.txt"
chk wait_for "mac new.txt present" 60 test -f "$W/mac/new.txt"
LG add new.txt dir/c.txt
LTREE=$(LG write-tree); say "linux write-tree $LTREE"
chk wait_for "mac write-tree equals linux" 60 bash -c "[ \"\$(cd '$W/mac' && git write-tree)\" = \"$LTREE\" ]"
MTREE=$(MG write-tree); say "mac   write-tree $MTREE"

say "== 3. commit on macOS, Linux gets the same HEAD"
MG commit -q -m "from macOS"
MHEAD=$(MG rev-parse HEAD); say "mac   HEAD $MHEAD"
chk wait_for "linux HEAD equals mac" 60 bash -c "[ \"\$(docker exec -w /lw $CN git rev-parse HEAD)\" = \"$MHEAD\" ]"
say "linux HEAD $(LG rev-parse HEAD); linux status --porcelain: [$(LG status --porcelain)]; mac status --porcelain: [$(MG status --porcelain)]"
say "linux log: $(LG log --oneline | tr '\n' '|')"

say "== 4. fleet verdict"
sleep 6
[ -n "${KEEP:-}" ] && [ -n "${SKIPFLEET:-}" ] && { say "KEEP=1: temp dir $W, container $CN, mac daemon pid $MPID"; exit 0; }
M fleet --worktree "$W/mac" --store "$W/store" --wait 90 --lang en 2>&1 | tee -a "$LOG"; MFLEET=${PIPESTATUS[0]}
say "mac fleet exit $MFLEET"
M fleet --worktree "$W/mac" --store "$W/store" --json > "$OUT/1113-two-os-fleet.json" 2>&1
L fleet --worktree /lw --store /store --wait 30 --lang en 2>&1 | tee -a "$LOG"; LFLEET=${PIPESTATUS[0]}
say "linux fleet exit $LFLEET"
[ "$MFLEET" = 0 ] && [ "$LFLEET" = 0 ] || FAILED=1
say "fleet json: $(tr -d '\n ' < "$OUT/1113-two-os-fleet.json" | cut -c1-1500)"
say "== done, failed=$FAILED"
[ -n "${KEEP:-}" ] && say "KEEP=1: temp dir $W, container $CN, mac daemon pid $MPID"
exit $FAILED
