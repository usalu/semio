#!/usr/bin/env bash
# Second half of the switch of g1-server (the renames are done): provision for the server role at /Users/ueli/Documents/semio, old cache settings, wait for the consent, report.
# Run by the owner in Terminal as ueli: bash ~/g1-server-switch/finish.sh
set -euo pipefail
T="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology"
[ -f /Users/ueli/Documents/semio/.cache/f/machine.json ] && [ ! -e /Users/Shared/semio ] || { echo "ERROR: the renames are not done"; exit 1; }
rsync -a --delete "$T/" "$HOME/g1-server-switch/topology/"
START=1794754224
HERE=$HOME/g1-server-switch
LIVE=/Users/Shared/semio
CANON=/Users/ueli/Documents/semio
ASIDE=/Users/ueli/Documents/semio.old-20261114
ACCOUNT=ueli
TOPOLOGY=$HERE/topology
LOG=$HERE/switch.log
LABELS="replica relay cache compact"
MODULES="🧰️framework/🛍️products/🦑️repo/🔨️modules"
TICKETS=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11"
SCRIPTS="$TICKETS/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🧰️scripts/1114-g1-server-canonical"
LAUNCHER_BIN=$HOME/.local/share/semio/launcher/bin
FACTS=/Users/Shared/semio-store/v1/fleet/cache.json
DOMAIN=gui/$(id -u)

say() { printf '[switch %s] %s\n' "$(date +%H:%M:%S)" "$*" | tee -a "$LOG"; }
fail() { say "ERROR: $*"; exit 1; }
holders() { lsof -n -F pn 2>/dev/null | awk -v root="$LIVE/" -v base="$LIVE" '/^p/{pid=substr($0,2)} /^n/{name=substr($0,2); if(name==base||index(name,root)==1) seen[pid]=1} END{for(pid in seen) print pid}' | sort -n; }
restart_old() {
  say "restarting the old services at $LIVE"
  for label in $LABELS; do sudo launchctl bootstrap system "/Library/LaunchDaemons/tech.semio.fleet.$label.plist" 2>/dev/null || true; done
  printf '\n**switch aborted at %s, not switched**: the tree is still at `/Users/Shared/semio`, the old services were restarted\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$NOTE"
}
cache_lines() { grep -n 'SEMIO_REMOTE_CACHE_' "$1" 2>/dev/null | sed 's/=.*//' | tr '\n' ' ' || true; }
[ "$(id -un)" = "$ACCOUNT" ] || fail "run as $ACCOUNT"; sudo -v || fail "sudo is needed"
say "step 5: provision for the server role with the current script (renders the LaunchAgents of $ACCOUNT, retires the system daemons, rewrites the environment block, installs the boot program)"
say "  when macOS asks \"repo would like to access files in your Documents folder\": Allow"
say "  lines that leave /etc/zshenv: $(cache_lines /etc/zshenv)"
say "  lines that leave /etc/profile: $(cache_lines /etc/profile)"
MARKER=$CANON/.cache/f/native-bootstrap.done
HELD=0
if [ ! -f "$MARKER" ]; then date -u +%Y-%m-%dT%H:%M:%SZ > "$MARKER"; HELD=1; say "  the native bootstrap is held back for this run (the coordinator runs it separately): temporary marker $MARKER"; fi
set +e
SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.71 SEMIO_FLEET_NETWORK=192.168.178.0/24 \
  env -u SEMIO_FLEET_CACHE_ENDPOINT -u SEMIO_WORKSPACE_ROOT -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE \
  bash "$TOPOLOGY/🥾️provision/🐚️.sh" g1 g1-server --user "$ACCOUNT" --server 2>&1 | tee -a "$LOG"
CODE=${PIPESTATUS[0]}
set -e
if [ "$HELD" = 1 ]; then rm -f "$MARKER"; say "  temporary marker removed: preflight keeps showing the native build as pending"; fi
say "provision exit code $CODE (0 done; 3 fleet ready, native build pending; 4 fleet ready, waiting for the Documents consent; 5 fleet code older than the script; 1 error)"

say "step 6: the old cache settings and the old cache server on port 6090"
for file in /etc/zshenv /etc/profile; do
  LEFT=$(cache_lines "$file")
  if [ -z "$LEFT" ]; then say "  $file: no SEMIO_REMOTE_CACHE_ line left"; else say "  WARNING: $file still has: $LEFT (the provision step did not rewrite the block; see its output above)"; fi
done
CACHE_AGENT=$HOME/Library/LaunchAgents/tech.semio.fleet.cache.plist
if grep -q '"port": 6094' "$FACTS" 2>/dev/null && grep -q '"instance": "main"' "$FACTS" 2>/dev/null; then
  launchctl bootout "$DOMAIN/tech.semio.fleet.cache" 2>/dev/null || true
  sudo launchctl bootout system/tech.semio.fleet.cache 2>/dev/null || true
  if [ -f "$CACHE_AGENT" ]; then rm -f "$CACHE_AGENT"; say "  removed $CACHE_AGENT and unloaded tech.semio.fleet.cache: the fleet facts name the sidecar on port 6094, instance main"; fi
  [ ! -f /Library/LaunchDaemons/tech.semio.fleet.cache.plist ] || say "  WARNING: /Library/LaunchDaemons/tech.semio.fleet.cache.plist still exists (the provision step did not retire it)"
else
  say "  kept tech.semio.fleet.cache: $FACTS does not name port 6094 and instance main"
fi

say "step 7: wait for the replica daemon and the consent of the launcher (up to 10 minutes; click Allow on the dialogs)"
CONSENT=open
for round in $(seq 1 300); do
  RUNNING=no; MARK=no
  PID=$(sed -n 's/.*"pid":\([0-9]*\).*/\1/p' "$CANON/.cache/f/daemon.lock" 2>/dev/null || true)
  if [ -n "$PID" ] && kill -0 "$PID" 2>/dev/null; then RUNNING=yes; fi
  if ls "$HOME/Library/Application Support/semio/access/"*/launcher.json >/dev/null 2>&1; then MARK=yes; fi
  if [ "$RUNNING" = yes ] && [ "$MARK" = yes ]; then CONSENT=ok; break; fi
  [ $(( round % 15 )) != 0 ] || say "  waiting: replica daemon running $RUNNING, launcher consent recorded $MARK (a dialog may be waiting on the screen)"
  sleep 2
done
say "consent: $CONSENT (replica daemon running ${RUNNING:-no}, launcher consent recorded ${MARK:-no})"
[ "$CONSENT" = ok ] || say "  still open: System Settings > Privacy & Security > Files & Folders > repo ($LAUNCHER_BIN/repo) > Documents Folder; if the replica agent does not run: launchctl kickstart -k $DOMAIN/tech.semio.fleet.replica"

say "report (services down for $(( $(date +%s) - START )) s until here; log: $LOG)"
{
  ls -l "$LAUNCHER_BIN" /usr/local/bin/repo "$HOME/.local/bin/repo" 2>&1
  codesign -dvvv "$LAUNCHER_BIN/repo" 2>&1 | grep -E '^Identifier|^CDHash=' || true
  grep -o '"bootVersion":[0-9]*' "$HOME/Library/Application Support/semio/fleet/entry/entry.json" || true
  launchctl list | grep 'tech\.semio' || echo "no tech.semio job loaded"
  launchctl print "$DOMAIN/tech.semio.fleet.replica" 2>/dev/null | grep -E 'state =|pid =|runs =|pended' || echo "the replica agent is not loaded"
  lsof -nP -iTCP:6090 -iTCP:6094 -iTCP:8980 -iTCP:8981 -sTCP:LISTEN 2>/dev/null | awk '{print $1, $2, $9}' || true
  grep -E 'graph-server: (alive|serving)' "$CANON/.cache/f/sidecars/build-agent.log" 2>/dev/null | tail -n 3 || true
} 2>&1 | sed 's/^/[switch]   /' | tee -a "$LOG"
exit "$CODE"
