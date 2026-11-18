#!/usr/bin/env bash
# Switch of g1-server from /Users/Shared/semio to /Users/ueli/Documents/semio (2026-11-14, revised 2026-11-15 for the launcher, consent and cache landings).
# Run by the owner in Terminal as ueli, from a copy outside both trees: bash ~/g1-server-switch/switch.sh
# It asks for the sudo password (once or twice). Nothing is deleted except what the list below names.
#
# WHAT THIS SCRIPT REMOVES OR REPLACES (everything else is renamed or rewritten in place):
#   1. The four system services /Library/LaunchDaemons/tech.semio.fleet.{replica,relay,cache,compact}.plist: unloaded and their files removed
#      by the provision script, which renders LaunchAgents of ueli instead (~/Library/LaunchAgents/tech.semio.fleet.{replica,relay,compact}.plist).
#   2. The old cache server on port 6090 (job tech.semio.fleet.cache, `repo remote-cache serve --port 6090`, instance "semio"): stopped, and the
#      LaunchAgent file that the provision script renders for it is removed again. Only when the published fleet facts name port 6094 (the signed
#      sidecar of the replica daemon); otherwise it is kept and the script says so. Its data /var/lib/semio-remote-cache (2.3 MiB) stays.
#   3. In /etc/zshenv and /etc/profile, inside the block "# >>> semio fleet": the three lines SEMIO_REMOTE_CACHE_ENDPOINT, SEMIO_REMOTE_CACHE_ACCESS,
#      SEMIO_REMOTE_CACHE_INSTANCE. The lines SEMIO_FLEET_GROUP, SEMIO_FLEET_NFS_HOST, SEMIO_FLEET_NETWORK stay. Nothing outside the block changes.
#   4. /usr/local/bin/repo: the link is pointed at the launcher ~/.local/share/semio/launcher/bin/repo (it named ~/.local/bin/repo).
#   5. The launcher ~/.local/share/semio/launcher/bin/repo: replaced by the current build (a link to versions/<hash>/repo, signed tech.semio.repo)
#      when its sources changed. macOS then asks once more for the Documents consent.
#   6. Links in ~/.local/bin that point into /Users/Shared/semio (go, gofmt): pointed at the same file below /Users/ueli/Documents/semio.
#   7. launchd jobs tech.semio.repo.service.* and tech.semio.repo.probe.* of ueli (graph service, consent probe): unloaded; they start again by themselves.
# Before the two renames every failure restarts the old services and leaves both trees where they are.
#
# YOUR CLICKS: "Allow" on every macOS dialog that says "... would like to access files in your Documents folder" (for "repo", and for "Terminal"
# if it asks). Stay at the Mac until the script prints "consent: ok" or tells you what is still open.
set -euo pipefail

LIVE=/Users/Shared/semio
CANON=/Users/ueli/Documents/semio
ASIDE=/Users/ueli/Documents/semio.old-20261114
ACCOUNT=ueli
HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
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

[ "$(id -un)" = "$ACCOUNT" ] || fail "run as $ACCOUNT (the script calls sudo itself)"
case "$HERE/" in "$LIVE/"*|"$CANON/"*) fail "run from a copy outside both trees" ;; esac
[ -f "$TOPOLOGY/🥾️provision/🐚️.sh" ] || fail "no copy of the topology directory at $TOPOLOGY"
[ -d "$LIVE" ] && [ ! -L "$LIVE" ] || fail "$LIVE is not a directory"
[ -d "$CANON" ] && [ ! -L "$CANON" ] || fail "$CANON is not a directory"
[ ! -e "$ASIDE" ] || fail "$ASIDE exists already"
[ -f "$LIVE/.cache/f/machine.json" ] || fail "$LIVE is not joined"
[ ! -f "$CANON/.cache/f/machine.json" ] || fail "$CANON is joined already: this script ran before"
grep -q '"originId":"g1-server"' "$LIVE/.cache/f/machine.json" || fail "$LIVE is not origin g1-server"
[ "$(stat -f %d "$LIVE")" = "$(stat -f %d "$CANON")" ] || fail "the two trees are on different volumes: a rename is not possible"
diff -rq "$TOPOLOGY" "$LIVE/$MODULES/🤝️shared-worktree/🗺️topology" >/dev/null 2>&1 || fail "the copy $TOPOLOGY differs from the topology directory of the tree (the fleet code moved on): copy it again"
cmp -s "$HERE/rebind-check.ts" "$LIVE/$SCRIPTS/rebind-check.ts" || fail "the copy of rebind-check.ts differs from the one in the tree: copy it again"
cmp -s "${BASH_SOURCE[0]}" "$LIVE/$SCRIPTS/switch.sh" || fail "this copy of switch.sh differs from the one in the tree: copy it again"
LOCKS=("$LIVE/$TICKETS/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-landing.lock" "$LIVE/$TICKETS/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landing.lock")
NOTE="$LIVE/$TICKETS/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/SERVER-RELOCATION.md"
[ -f "$NOTE" ] || fail "the relocation note is missing: $NOTE"
[ -z "$(hdiutil info | grep -F "$CANON/.cache/b/" || true)" ] || fail "an image of the old checkout is attached"
sudo -v || fail "sudo is needed"

say "waiting until both landing locks are free for two minutes in a row (at most 30 minutes)"
FREE=0
for _ in $(seq 1 360); do
  HELD_LOCK=""
  for lock in "${LOCKS[@]}"; do [ ! -e "$lock" ] || HELD_LOCK="$lock ($(head -n 1 "$lock/owner" 2>/dev/null | cut -c1-80))"; done
  if [ -n "$HELD_LOCK" ]; then [ "$FREE" = 0 ] || say "  a lock was taken again: $HELD_LOCK"; FREE=0; else FREE=$(( FREE + 5 )); fi
  [ "$FREE" -lt 120 ] || break
  sleep 5
done
[ "$FREE" -ge 120 ] || fail "a landing lock is still held after 30 minutes: ${HELD_LOCK:-unknown}; nothing was stopped or renamed"
sudo -v || fail "sudo is needed"
printf '\n**switching at %s** (the owner started the switch script; agents on this Mac stop using `/Users/Shared/semio` now)\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$NOTE"
say "noted in SERVER-RELOCATION.md: switching (10 s for the replica to carry the line to the fleet)"
sleep 10

say "step 0: the launcher (the one program macOS gives the Documents consent to), built by the current installer while everything still runs"
BEFORE_LAUNCHER=$(shasum -a 256 "$LAUNCHER_BIN/repo" 2>/dev/null | cut -c1-16 || true)
set +e
( cd "$LIVE" && env -u SEMIO_WORKSPACE_ROOT "XDG_DATA_HOME=$HOME/.local/share" bash "$LIVE/$MODULES/🚀️launcher/📥️install/🐚️.sh" "$LAUNCHER_BIN" ) 2>&1 | sed 's/^/[switch]   /' | tee -a "$LOG"
INSTALL_CODE=${PIPESTATUS[0]}
set -e
[ "$INSTALL_CODE" = 0 ] || fail "the launcher installer failed: nothing was stopped or renamed"
[ -x "$LAUNCHER_BIN/repo" ] || fail "the launcher installer did not install $LAUNCHER_BIN/repo: nothing was stopped or renamed"
AFTER_LAUNCHER=$(shasum -a 256 "$LAUNCHER_BIN/repo" | cut -c1-16)
if [ "$BEFORE_LAUNCHER" = "$AFTER_LAUNCHER" ]; then say "  launcher unchanged ($AFTER_LAUNCHER)"; else say "  launcher replaced ($BEFORE_LAUNCHER -> $AFTER_LAUNCHER): macOS asks for the Documents consent when the services start"; fi
START=$(date +%s)

say "step 1: stop the four system services and the graph service jobs of $ACCOUNT (they are unloaded, so launchd does not restart them)"
for label in $LABELS; do sudo launchctl bootout "system/tech.semio.fleet.$label" 2>/dev/null || true; done
for label in $(launchctl list | awk '$3 ~ /^tech\.semio\.(repo\.(service|probe)|fleet)\./ {print $3}'); do
  launchctl bootout "$DOMAIN/$label" 2>/dev/null || true; say "  unloaded $label"
done
for _ in $(seq 1 30); do
  PID=$(sed -n 's/.*"pid":\([0-9]*\).*/\1/p' "$LIVE/.cache/f/daemon.lock" 2>/dev/null || true)
  { [ -n "$PID" ] && kill -0 "$PID" 2>/dev/null; } || break
  sleep 1
done

say "step 2: stop what still holds files in $LIVE"
for round in 1 2 3 4 5 6; do
  PIDS=$(holders | grep -vx "$$" || true)
  REST=""
  for pid in $PIDS; do
    case "$(ps -o command= -p "$pid" 2>/dev/null)" in
      *diskimages-helper*|"") ;;
      */Claude.app/*|*/Codex.app/*|*/ChatGPT.app/*) say "  left alone (an agent app, it only reads): $pid" ;;
      *) REST="$REST $pid" ;;
    esac
  done
  [ -n "$REST" ] || break
  for pid in $REST; do say "  stopping $pid: $(ps -o command= -p "$pid" | cut -c1-140)"; if [ "$round" -lt 5 ]; then kill "$pid" 2>/dev/null || true; else kill -KILL "$pid" 2>/dev/null || true; fi; done
  sleep 3
done
if [ -n "${REST:-}" ]; then restart_old; fail "these processes still hold the tree:$REST"; fi

say "step 3: detach the build-cache images (deepest mount first)"
MOUNTS=$(mount | sed -n "s|^/dev/[^ ]* on \($LIVE/.cache/b/.*\) (.*|\1|p" | awk '{print length($0) "\t" $0}' | sort -rn | cut -f2-)
while IFS= read -r point; do
  [ -n "$point" ] || continue
  hdiutil detach "$point" >>"$LOG" 2>&1 || { sleep 3; hdiutil detach "$point" >>"$LOG" 2>&1; } || { restart_old; fail "could not detach $point (see $LOG); nothing was renamed"; }
  say "  detached $point"
done <<< "$MOUNTS"
[ -z "$(mount | grep -F " on $LIVE/" || true)" ] || { restart_old; fail "a volume is still mounted inside $LIVE"; }
[ -z "$(hdiutil info | grep -F "$LIVE/" || true)" ] || { restart_old; fail "an image inside $LIVE is still attached"; }

say "step 4: the two renames"
mv -n "$CANON" "$ASIDE"
[ -d "$ASIDE" ] && [ ! -e "$CANON" ] || { restart_old; fail "the old checkout was not renamed aside"; }
mv -n "$LIVE" "$CANON"
[ -f "$CANON/.cache/f/machine.json" ] && [ ! -e "$LIVE" ] || fail "the live tree was not renamed: $LIVE and $CANON must be checked by hand before anything else"
say "  $LIVE is now $CANON; the old checkout is $ASIDE ($(( $(date +%s) - START )) s since the services stopped)"
for link in "$HOME"/.local/bin/*; do
  [ -L "$link" ] || continue
  target=$(readlink "$link")
  case "$target" in "$LIVE/"*) ln -sfn "$CANON/${target#"$LIVE/"}" "$link"; say "  link $link now points below $CANON" ;; esac
done

say "step 4b: re-bind the build-cache images to the new place (a hint names the move; the image opener verifies each image and rewrites its receipt)"
IMAGES=$CANON/.cache/b
IMAGES_ASIDE=$CANON/.cache/b.relocated-20261114
printf '{"version":1,"movedFrom":"%s","movedTo":"%s"}\n' "$LIVE" "$CANON" > "$IMAGES/image-relocation.json"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -n 1)
set +e
"$BUN" "$HERE/rebind-check.ts" "$CANON" > "$HERE/rebind.log" 2>&1
REBIND_CODE=$?
set -e
sed 's/^/[switch]   /' "$HERE/rebind.log" | tee -a "$LOG"
mkdir -p "$IMAGES_ASIDE.partial"
{ grep '^unbound' "$HERE/rebind.log" || true; } | cut -f2 | while IFS= read -r partition; do
  [ -n "$partition" ] || continue
  mv -n "$IMAGES/$partition" "$IMAGES_ASIDE.partial/$partition"; say "  $partition was refused and is set aside in $IMAGES_ASIDE.partial (created fresh by the next command)"
done
rmdir "$IMAGES_ASIDE.partial" 2>/dev/null || true
[ "$REBIND_CODE" = 0 ] || grep -q '^unbound\|^bound' "$HERE/rebind.log" || say "  the re-bind check did not run (exit $REBIND_CODE): the probe below decides"
PROBE=$HERE/probe.log
( cd "$CANON" && env -u SEMIO_WORKSPACE_ROOT -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE SEMIO_LANGUAGE=en repo cache status > "$PROBE" 2>&1 ) &
PROBE_PID=$!
for _ in $(seq 1 180); do kill -0 "$PROBE_PID" 2>/dev/null || break; sleep 2; done
if kill -0 "$PROBE_PID" 2>/dev/null; then kill "$PROBE_PID" 2>/dev/null || true; PROBE_CODE=124; else wait "$PROBE_PID" && PROBE_CODE=0 || PROBE_CODE=$?; fi
say "  probe 'repo cache status' at the new place: exit $PROBE_CODE (output: $PROBE)"
if [ "$PROBE_CODE" != 0 ] && grep -q 'Native' "$PROBE"; then
  say "  the images were refused at the new place: they are set aside as $IMAGES_ASIDE and the next command creates fresh ones (cold build caches)"
  grep -m 3 'Native' "$PROBE" | cut -c1-300 | tee -a "$LOG"
  for point in $(mount | sed -n "s|^/dev/[^ ]* on \($CANON/.cache/b/.*\) (.*|\1|p" | awk '{print length($0) "\t" $0}' | sort -rn | cut -f2-); do hdiutil detach "$point" >>"$LOG" 2>&1 || true; done
  [ ! -e "$IMAGES_ASIDE" ] || fail "$IMAGES_ASIDE exists already"
  mv -n "$IMAGES" "$IMAGES_ASIDE"
  [ -d "$IMAGES_ASIDE" ] && [ ! -e "$IMAGES" ] || fail "the image folder was not set aside"
else
  say "  images kept: $(ls "$IMAGES" | tr '\n' ' ')"
fi

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
