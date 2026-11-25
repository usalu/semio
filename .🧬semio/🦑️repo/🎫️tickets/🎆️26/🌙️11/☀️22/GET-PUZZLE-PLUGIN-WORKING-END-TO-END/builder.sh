#!/usr/bin/env bash
# 🏗️ One long-lived private build area for puzzle: a frozen source copy (files under 200 KB plus all rs, toml, lock) layered over the
# live tree, a target on disk that survives between runs and restarts, warm cache below it. Nothing is written to the shared tree.
#   builder.sh start | exec '<snippet>' | refresh <repo-relative path>... | status | stop      (exec takes the one lock)
set -euo pipefail
T="$(cd "$(dirname "$0")" && pwd)"; R="$(cd "$T/../../../../../../.." && pwd)"
ST=/dev/shm/semio-puzzle-builder; mkdir -p "$ST"; PIDF="$ST/pid"
LOWER="${LOWER_DIR:-$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/native-target/b}"
DISK="${BUILDER_DISK:-$T/🗑️generated/builder}"
paused() { while [ -f "$ST/paused" ]; do sleep 5; done; }
DIRS=("🧰️framework" "✏️s/🔌️plugins" "🌎️hub")
alive() { [ -f "$PIDF" ] && kill -0 "$(cat "$PIDF")" 2>/dev/null; }
inns() { nsenter -t "$(cat "$PIDF")" --user --mount --preserve-credentials "$@"; }
copy() { ( cd "$R/$1" && find . -type f \( -size -200k -o -name '*.rs' -o -name '*.toml' -o -name '*.lock' \) -not -path '*/node_modules/*' -not -path '*/target/*' -not -path '*/🗑️generated/*' -print0 | tar --null -T - -cf - ) | ( mkdir -p "$2" && tar -xf - -C "$2" ); }
mask() { find /tmp/snap -name '*.rs' -print0 | xargs -0 grep -lZ '#\[path="/Users/[^"]*debug-ledger\.rs"\]' 2>/dev/null | xargs -0 -r sed -i -E 's#\#\[path="/Users/[^"]*debug-ledger\.rs"\]#\#[path="/tmp/snap-stub.rs"]#'; }
case "${1:-}" in
start)
  alive && { echo "builder: already running $(cat "$PIDF")"; exit 0; }
  export R T LOWER DISK; mkdir -p "$DISK"/{t,ov/u,ov/w,ovs,snap}
  nohup unshare --user --map-root-user --mount --propagation private bash -c '
    set -e; mount -t tmpfs -o mode=1777,size=3000m tmpfs /tmp; mkdir -p /tmp/build/t /tmp/build/b /tmp/snap /tmp/ovs /tmp/zig /tmp/zig-local
    mount --bind "$DISK/t" /tmp/build/t; mount --bind "$DISK/snap" /tmp/snap; mount --bind "$DISK/ovs" /tmp/ovs
    mount -t overlay overlay -o "lowerdir=$LOWER,upperdir=$DISK/ov/u,workdir=$DISK/ov/w,userxattr" /tmp/build/b
    cp "$T/debug-ledger-stub.rs" /tmp/snap-stub.rs; exec sleep infinity' </dev/null >"$ST/log" 2>&1 &
  echo $! >"$PIDF"; sleep 2; alive || { cat "$ST/log"; exit 1; }
  export -f copy mask
  inns bash -c 'set -e; i=0; fresh=0; for d in "🧰️framework" "✏️s/🔌️plugins" "🌎️hub"; do i=$((i+1)); mkdir -p "/tmp/ovs/$i/u" "/tmp/ovs/$i/w" "/tmp/live/$i"; mount --bind "$R/$d" "/tmp/live/$i"; [ -d "/tmp/snap/$i" ] || { copy "$d" "/tmp/snap/$i"; fresh=1; }; done; [ $fresh = 1 ] && mask; [ -f /tmp/snap/FROZEN ] || date +"frozen %F %T" > /tmp/snap/FROZEN
    i=0; for d in "🧰️framework" "✏️s/🔌️plugins" "🌎️hub"; do i=$((i+1)); mount -t overlay overlay -o "lowerdir=/tmp/snap/$i:/tmp/live/$i,upperdir=/tmp/ovs/$i/u,workdir=/tmp/ovs/$i/w,userxattr" "$R/$d"; done; cat /tmp/snap/FROZEN; mount | grep -c "type overlay"' ;;
inner) export BUILDER_INNER=1; shift; paused; alive || exit 2
  inns env R="$R" SNIPPET="$1" BUILDER=1 LANE_DIR="${BUILDER_LANE:+$DISK/lane-$BUILDER_LANE}" bash "$T/builder-run.sh" ;;
exec)
  alive || { echo "builder: not running"; exit 2; }; shift
  LANE="${BUILDER_LANE:-}"; Q="$ST/queue$LANE"; me=$BASHPID; echo "$me" >>"$Q"
  while :; do head=""; while read -r w; do [ -n "$w" ] && kill -0 "$w" 2>/dev/null && { head=$w; break; }; done <"$Q"; [ "$head" = "$me" ] && break; sleep 3; done
  exec 9>"$ST/lock$LANE"; flock 9; paused
  got=""; while [ -z "$got" ]; do for n in 1 2; do exec 8>"$ST/rust-slot-$n"; if flock -n 8; then got=$n; break; fi; done; [ -n "$got" ] || sleep 3; done
  if [ -z "${BUILDER_INNER:-}" ]; then cd "$R"; BUILDER_INNER=1 repo capacity run --memory="${BUILDER_MEM:-2500}" --cpu=2 -- timeout "${BUILDER_TIMEOUT:-10000}" "$T/builder.sh" inner "$1"; exit $?; fi
  ;;
refresh) shift; alive || exit 2
  for p in "$@"; do inns env R="$R" P="$p" bash -c 'i=0; for d in "🧰️framework" "✏️s/🔌️plugins" "🌎️hub"; do i=$((i+1)); case "$P" in "$d"/*) rel="${P#"$d"/}"; if [ -e "/tmp/live/$i/$rel" ]; then mkdir -p "$(dirname "$R/$P")"; if [ -d "/tmp/live/$i/$rel" ]; then mkdir -p "$R/$P" && cp -a "/tmp/live/$i/$rel/." "$R/$P/"; else cp -a "/tmp/live/$i/$rel" "$R/$P"; fi && sed -i -E "s#\\#\\[path=\"/Users/[^\"]*debug-ledger\\.rs\"\\]#\#[path=\"/tmp/snap-stub.rs\"]#" $(find "$R/$P" -name "*.rs" 2>/dev/null) && echo "refreshed $P"; else echo "missing live $P"; fi;; esac; done'; done ;;
status) alive && inns bash -c 'df -m /tmp | tail -1; mount | grep -c "type overlay"; ls /tmp/snap' ; free -m | sed -n 2,3p ;;
stop) alive && kill "$(cat "$PIDF")"; rm -f "$PIDF" ;;
*) echo "usage: builder.sh start|exec|status|stop"; exit 64 ;;
esac
