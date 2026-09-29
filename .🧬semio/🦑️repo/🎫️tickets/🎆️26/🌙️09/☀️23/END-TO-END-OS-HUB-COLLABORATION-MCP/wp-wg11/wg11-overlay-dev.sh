#!/bin/zsh
# 🪞️ WG11 session 14d: one overlay-lane hold of the renderer fix loop — re-sync the overlay from the tree (APFS clones), apply every
# set in wg11-overlay-dev-patches.txt (dry run, then its flag), then run the given cargo commands with a PRIVATE build-dir + target
# inside the overlay (never the shared build-dir). Lines prefixed [wg11-ov].
# usage: setopt no_bg_nice; nohup zsh wg11-overlay-dev.sh '<cargo cmd>' ['<cargo cmd>' …] > <capture> 2>&1 & disown
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg11
O="$R/.🧬semio/🌐hub/s14-wg11-overlay"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg11-ov] queued $(date '+%F %T') pid=$$ load=$(sysctl -n vm.loadavg)"
export WG11_COMMANDS="${(pj:\n:)@}"
zsh $M[1] overlay wg11 -- zsh -c '
set -u
echo "[wg11-ov] HELD $(date "+%F %T") load=$(sysctl -n vm.loadavg)"
python3 '"$W"'/wg11-overlay-sync.py || exit 1
while read -r patch flag; do
  case "$patch" in ""|\#*) continue;; esac
  python3 '"$W"'/wg11-overlay-apply.py "'"$O"'" "'"$W"'/$patch" "$flag" || { echo "[wg11-ov] patch $patch FAILED"; exit 1; }
done < '"$W"'/wg11-overlay-dev-patches.txt
if [ -f '"$W"'/wg11-overlay-debug.on ]; then python3 '"$W"'/wg11-overlay-debug.py --write || { echo "[wg11-ov] debug instrumentation FAILED"; exit 1; }; fi
cd "'"$O"'" || exit 1
export CARGO_INCREMENTAL=1 CARGO_BUILD_BUILD_DIR="'"$O"'/.cargo-build" CARGO_TARGET_DIR="'"$O"'/target" NX_DAEMON=false
while IFS= read -r command; do
  [ -z "$command" ] && continue
  echo "[wg11-ov] RUN $command $(date "+%F %T")"
  nice -n 15 zsh -c "$command"
  echo "[wg11-ov] RC=$? $(date "+%F %T")"
done <<< "$WG11_COMMANDS"
echo "[wg11-ov] RELEASE $(date "+%F %T")"
'
echo "[wg11-ov] END rc=$? $(date '+%F %T')"
