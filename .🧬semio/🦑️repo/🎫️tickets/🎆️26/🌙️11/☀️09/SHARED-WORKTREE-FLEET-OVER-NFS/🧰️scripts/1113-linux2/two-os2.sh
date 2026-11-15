#!/bin/bash
# two-os2.sh: macOS replica (g1-server) + Linux container replica (g1-m01) on one temporary store, current code.
# scenarios: edit, stage, commit, case rename + case clash, file-vs-directory clash, kill -9 of the Linux daemon mid-apply, broken module -> code-fallback, final fleet.
# env: BUN, REAL (live worktree), IMAGE. Never touches the live store/worktree (module copy via mirror.sh, read-only mounts).
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
REAL=${REAL:-/Users/Shared/semio}; IMAGE=${IMAGE:-semio-linux-bun142}
BUN=${BUN:-/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
OUT=${1:-$HERE/../../🗑️generated/1113-linux2}; mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd)
W=$(cd "$(mktemp -d "${TMPDIR:-/tmp}/semio-two-os2.XXXXXX")" && pwd -P)
CN=semio-two-os2-$$
LOG="$OUT/two-os2-transcript.txt"; : > "$LOG"
say(){ printf '%s\n' "$*" | tee -a "$LOG"; }
export GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@example.test GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@example.test GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
for v in $(env | sed -n 's/^\(SEMIO_[A-Za-z0-9_]*\)=.*/\1/p'); do unset "$v"; done
MPID=
cleanup(){ [ -n "$MPID" ] && kill "$MPID" 2>/dev/null; docker rm -f "$CN" >/dev/null 2>&1; rm -rf "$W"; }
[ -n "${KEEP:-}" ] || trap cleanup EXIT
MOD=$("$HERE/mirror.sh" "$REAL" "$W/mir") || exit 2
MODREL=${MOD#"$W/mir/"}; SCRIPT="$MODREL/$(cd "$MOD" && ls -d *replica)/📜️script.ts"
mkdir -p "$W/store" "$W/mac"
say "temp dir $W; module $MODREL"
M(){ ( cd "$W/mac" && "$BUN" "$W/mir/$SCRIPT" "$@" ); }
L(){ docker exec -w /lw "$CN" bun "/lw/$SCRIPT" "$@"; }
LG(){ docker exec -w /lw "$CN" git "$@"; }
MG(){ ( cd "$W/mac" && git "$@" ); }
LEX(){ docker exec "$CN" bash -c "$*"; }
wait_for(){ local what=$1 secs=$2; shift 2; local i; for ((i=0;i<secs*2;i++)); do if "$@" >/dev/null 2>&1; then say "ok   $what (~$((i/2))s)"; return 0; fi; sleep 0.5; done; say "FAIL $what after ${secs}s"; return 1; }
FAILED=0; chk(){ "$@" || FAILED=1; }
lstart(){ docker exec -d -w /lw "$CN" bash -c "exec bun /lw/$SCRIPT run --worktree /lw --store /store --poll 100 --scan 5 --lang en >> /lw-daemon.log 2>&1"; }
lkill(){ LEX 'for p in $(ps -eo pid,args | grep -E "^ *[0-9]+ (/usr/local/bin/)?bun .*(script|stage)\.ts|boot\.mjs" | grep -v -e grep -e "bash -c" | awk "{print \$1}"); do kill -9 $p; done; sleep 1; ps -eo pid,args | grep -E "^ *[0-9]+ (/usr/local/bin/)?bun .*(script|stage)\.ts|boot\.mjs" | grep -v -e grep -e "bash -c" | wc -l'; }
tree_mac(){ ( cd "$W/mac" && find . -path ./.git -prune -o -path ./.cache -prune -o -type f -print0 | sort -z | xargs -0 shasum -a 256 2>/dev/null ); }
tree_lin(){ LEX 'cd /lw && find . -path ./.git -prune -o -path ./.cache -prune -o -path ./🧰️framework -prune -o -type f -print0 | sort -z | xargs -0 sha256sum' | sed 's#  \./#  ./#' ; }
same_trees(){ [ "$(tree_mac | sort -k2)" = "$(tree_lin | sort -k2)" ]; }

say "== seed on macOS"
cd "$W/mac" && git init -q -b main && printf '.cache/\nnode_modules/\n*.log\n🧰️framework/\n' > .gitignore && printf 'alpha\n' > a.txt && mkdir dir && printf 'charlie\n' > dir/c.txt && git add -A && git commit -q -m initial
M seed --group g1 --origin g1-server --worktree "$W/mac" --store "$W/store" --lang en 2>&1 | tail -2 | tee -a "$LOG"
(cd "$W/mac" && exec nohup "$BUN" "$W/mir/$SCRIPT" run --worktree "$W/mac" --store "$W/store" --poll 100 --scan 5 --lang en > "$OUT/two-os2-mac-daemon.log" 2>&1 < /dev/null) &
MPID=$!
say "== Linux container: module copied into the worktree, gitignored so it is not shared, join, run"
docker run -d --init --name "$CN" -v "$REAL:$REAL:ro" -v "$W/mir:/mir:ro" -v "$W/store:/store" "$IMAGE" sleep infinity > /dev/null
docker exec "$CN" bash -c "mkdir -p /lw"
docker exec -w /lw "$CN" bun "/mir/$SCRIPT" join --group g1 --origin g1-m01 --worktree /lw --store /store --lang en 2>&1 | tail -2 | tee -a "$LOG"
docker exec "$CN" bash -c 'cp -a /mir/🧰️framework /lw/'
lstart
chk wait_for "linux joined" 90 docker exec "$CN" test -f /lw/a.txt
say "linux: $(docker exec "$CN" uname -sm), bun $(docker exec "$CN" bun --version)"
chk wait_for "linux HEAD equals mac HEAD" 60 bash -c "[ \"\$(docker exec -w /lw $CN git rev-parse HEAD)\" = \"$(MG rev-parse HEAD)\" ]"

say "== 1 edit"
printf 'charlie edited on macOS\n' > "$W/mac/dir/c.txt"
chk wait_for "linux dir/c.txt equals mac" 60 bash -c "[ \"\$(docker exec $CN cat /lw/dir/c.txt)\" = \"charlie edited on macOS\" ]"
say "== 2 stage on Linux"
LEX "printf 'new on linux\n' > /lw/new.txt"
chk wait_for "mac new.txt present" 60 test -f "$W/mac/new.txt"
LG add new.txt dir/c.txt
LT=$(LG write-tree); chk wait_for "mac write-tree equals linux" 60 bash -c "[ \"\$(cd '$W/mac' && git write-tree)\" = \"$LT\" ]"
say "== 3 commit on macOS"
MG commit -q -m "from macOS"; MH=$(MG rev-parse HEAD)
chk wait_for "linux HEAD equals mac" 60 bash -c "[ \"\$(docker exec -w /lw $CN git rev-parse HEAD)\" = \"$MH\" ]"

say "== 4 case rename on macOS of an untracked file: low.txt -> LOW.txt (a tracked one differs by design: git on a case-insensitive volume does not see it)"
printf 'low\n' > "$W/mac/low.txt"
chk wait_for "linux low.txt present" 60 docker exec "$CN" test -f /lw/low.txt
( cd "$W/mac" && mv low.txt low.tmp && mv low.tmp LOW.txt )
chk wait_for "linux lists LOW.txt and not low.txt" 60 docker exec "$CN" bash -c 'cd /lw && [ -f LOW.txt ] && [ ! -e low.txt ]'
say "== 5 case clash: Foo.txt on macOS, foo.txt on Linux at the same time"
printf 'mac-foo\n' > "$W/mac/Foo.txt"; LEX "printf 'linux-foo\n' > /lw/foo.txt"
sleep 15
chk wait_for "trees equal after the case clash" 90 same_trees
say "mac names: $(ls "$W/mac" | tr '\n' ' ')"; say "linux names: $(LEX 'ls /lw' | tr '\n' ' ')"
say "== 6 file-vs-directory clash: file fd on macOS, directory fd/ on Linux"
printf 'file\n' > "$W/mac/fd"; LEX "mkdir -p /lw/fd && printf 'inner\n' > /lw/fd/inner.txt"
sleep 15
chk wait_for "trees equal after the file-vs-directory clash" 90 same_trees
say "mac fd: $(ls -ld "$W/mac"/fd* | awk '{print $1,$NF}' | tr '\n' ';')"; say "linux fd: $(LEX 'ls -ld /lw/fd*' | awk '{print $1,$NF}' | tr '\n' ';')"

say "== 7 kill -9 of the Linux daemon while it applies a bulk change from macOS"
( cd "$W/mac" && mkdir -p bulk && for i in $(seq 1 1500); do printf 'bulk %s\n' "$i" > bulk/f$i.txt; done )
for i in $(seq 1 40); do n=$(LEX 'ls /lw/bulk 2>/dev/null | wc -l'); [ "${n:-0}" -ge 100 ] && break; sleep 0.25; done
say "linux has $(LEX 'ls /lw/bulk 2>/dev/null | wc -l') of 1500 when killed; remaining daemon processes after kill -9: $(lkill)"
lstart
chk wait_for "linux converges after restart" 180 same_trees
say "linux bulk count $(LEX 'ls /lw/bulk | wc -l'), mac $(ls "$W/mac/bulk" | wc -l)"
chk wait_for "sealed snapshot exists on Linux" 120 docker exec "$CN" bash -c 'ls /lw/.cache/f/code/*/sealed.json'

say "== 8 broken module on the Linux side: kill, sabotage, restart -> sealed snapshot with code-fallback"
say "before kill: $(LEX "ps -eo pid,stat,args | grep -E 'bun|defunct' | grep -v grep | cut -c1-110" | tr "\n" "|")"
say "lock: $(LEX "cat /lw/.cache/f/daemon.lock")"
lkill >/dev/null
say "after kill: $(LEX "ps -eo pid,stat,args | grep -E 'bun|defunct' | grep -v grep | cut -c1-110" | tr "\n" "|")"
docker exec "$CN" bash -c "f=\$(ls -d /lw/$MODREL/*tiers)/🟦️.ts; cp \$f /tmp/tiers.good; sed -i 's/ adopt(mtime:/ adoptStale(mtime:/' \$f; grep -c adoptStale \$f"
lstart
chk wait_for "Linux attests code-fallback" 120 bash -c "M_OUT=\$(cd '$W/mac' && '$BUN' '$W/mir/$SCRIPT' fleet --worktree '$W/mac' --store '$W/store' --json 2>&1); echo \"\$M_OUT\" | grep -q code-fallback"
M fleet --worktree "$W/mac" --store "$W/store" --lang en 2>&1 | tee -a "$LOG"; say "fleet exit with fallback: ${PIPESTATUS[0]}"
say "linux status: $(L status --worktree /lw --store /store --lang en 2>&1 | tr '\n' '|')"
printf 'x\n' > "$W/mac/after-fallback.txt"
chk wait_for "the fallback daemon keeps syncing" 60 docker exec "$CN" test -f /lw/after-fallback.txt
say "== 8b restore the module, restart the daemon"
lkill >/dev/null
docker exec "$CN" bash -c "f=\$(ls -d /lw/$MODREL/*tiers)/🟦️.ts; cp /tmp/tiers.good \$f"
lstart
chk wait_for "Linux no longer on fallback" 120 bash -c "! (cd '$W/mac' && '$BUN' '$W/mir/$SCRIPT' fleet --worktree '$W/mac' --store '$W/store' --json 2>&1 | grep -q code-fallback)"

say "porcelain mac (not untracked): [$(MG status --porcelain | grep -v "^??" | tr "\n" "|")]"
say "porcelain linux (not untracked): [$(LG status --porcelain | grep -v "^??" | tr "\n" "|")]"
say "mac ls-files -s dir: $(MG ls-files -s dir | tr "\n" "|"); linux: $(LG ls-files -s dir | tr "\n" "|")"
say "== 9 fleet verdict"
sleep 8
M fleet --worktree "$W/mac" --store "$W/store" --wait 120 --lang en 2>&1 | tee -a "$LOG"; MF=${PIPESTATUS[0]}
L fleet --worktree /lw --store /store --wait 60 --lang en 2>&1 | tee -a "$LOG"; LF=${PIPESTATUS[0]}
say "mac fleet exit $MF, linux fleet exit $LF"
[ "$MF" = 0 ] && [ "$LF" = 0 ] || FAILED=1
chk same_trees
say "daemon logs: mac $(grep -c . "$OUT/two-os2-mac-daemon.log") lines; linux $(LEX 'grep -c . /lw-daemon.log') lines; linux blocked/error events: $(LEX 'grep -E "\"event\":\"(blocked|error)\"" /lw-daemon.log | head -5' | tr '\n' ' ')"
cp "$W/mac/.cache/f/boot/health.json" "$OUT/two-os2-mac-health.json" 2>/dev/null
docker exec "$CN" cat /lw-daemon.log > "$OUT/two-os2-linux-daemon.log" 2>/dev/null
say "== done, failed=$FAILED"
exit $FAILED
