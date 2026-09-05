#!/usr/bin/env bash
# 🧱️ Builds one wasm plugin component per pluginId, serially (cargo takes one global target-dir lock).
# Logs go to the session scratchpad, never to the ticket's 🗑️generated — peer sweeps delete that mid-run.
set -uo pipefail
cd /Users/ueli/Documents/semio || exit 1
export RUSTC_WRAPPER=""
export SEMIO_BUILD_BUDGET_MS="${SEMIO_BUILD_BUDGET_MS:-7200000}"
export SEMIO_CMD_BUDGET_MS="${SEMIO_CMD_BUDGET_MS:-7200000}"
# 🧭️ Resolved at run time: the dev module's directory name is being hand-repaired by a peer
# (variation-selector spelling in flux), so a hardcoded path breaks mid-refactor.
DEV=$(find 🧰️framework -maxdepth 8 -not -path "*/dist/*" -path "*dev/📦️packages/🟦️typescript/📜️script.ts" -print -quit 2>/dev/null)
[ -n "$DEV" ] || { echo "FATAL: dev script.ts not found"; exit 2; }
echo "DEV=$DEV"
LOG="${LOG_DIR:?LOG_DIR required}"
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
# 🔁️ A peer ticket renames artifact directories while these builds run, so a plugin that parsed fine
# at launch can die ~20 min in on a path that moved under it. `🔁️heal-loop.sh` repairs those within
# 45s, so the failure is a lost race, not a real defect -- retry it. Only the `couldn't read ... No
# such file or directory` class is retried; any genuine compile error fails through immediately
# instead of burning the budget on a doomed rebuild.
ATTEMPTS="${BUILD_ATTEMPTS:-4}"
for pid in "$@"; do
  attempt=1
  while :; do
    # 🧹️ `assertExtensionOutputsFresh` refuses to run while an obsolete `🧵️plugin-worker.js` from the
    # previous worker format is still on disk, and these reappear between runs. They are untracked
    # `@generated` output, so park them in the ticket rather than deleting an input we did not author.
    PARK="$TICKET/🗑️generated/stale-extension-workers"
    find "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules" -name "🧵️plugin-worker.js" 2>/dev/null | while IFS= read -r w; do
      mkdir -p "$PARK/$(basename "$(dirname "$w")")"
      mv "$w" "$PARK/$(basename "$(dirname "$w")")/" 2>/dev/null && echo "    parked stale worker: $(basename "$(dirname "$w")")"
    done
    echo "=== BUILD $pid attempt $attempt/$ATTEMPTS $(date '+%H:%M:%S') ==="
    SEMIO_PLUGIN_ONLY="$pid" bun "$DEV" plugin s > "$LOG/build-$pid.txt" 2>&1
    rc=$?
    echo "=== $pid rc=$rc attempt $attempt $(date '+%H:%M:%S') ==="
    [ $rc -eq 0 ] && break
    # 🌊️ Three failure classes are all "a peer edited the tree mid-build", not a defect in this crate:
    #   1. couldn't read <path>            -- a directory was renamed after this build resolved it
    #   2. trailing characters at line N   -- a descriptor JSON was read while being rewritten
    #   3. registry-resolved asset misses  -- the generated registry still names the pre-rename dir;
    #                                         it is regenerated at the start of every attempt
    # Any of these makes the whole run untrustworthy, including the E0277 cascades they produce, so
    # retry on the MARKER rather than trying to prove the remaining errors are all cascades.
    stale=$(grep -cE "^error: couldn't read |trailing characters at line " "$LOG/build-$pid.txt" 2>/dev/null)
    if [ "$stale" -gt 0 ] && [ "$attempt" -lt "$ATTEMPTS" ]; then
      cp "$LOG/build-$pid.txt" "$LOG/build-$pid.attempt$attempt.txt" 2>/dev/null
      echo "--- stale renamed path (x$stale); healer repairs within 45s, retrying ---"
      grep -E "^error: couldn't read" "$LOG/build-$pid.txt" | head -2
      sleep 50
      attempt=$((attempt + 1))
      continue
    fi
    echo "--- tail ---"; tail -25 "$LOG/build-$pid.txt"
    break
  done
done
